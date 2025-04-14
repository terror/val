use super::*;

fn parser<'a>()
-> impl Parser<'a, &'a str, Spanned<Ast<'a>>, extra::Err<Rich<'a, char>>> {
  let identifier = text::ident().padded();

  recursive(|expr| {
    let number = text::int(10)
      .then(just('.').then(text::digits(10)).or_not())
      .to_slice()
      .from_str()
      .unwrapped()
      .map(Ast::Number)
      .map_with(|ast, e| (ast, e.span()));

    let boolean = choice((just("true").to(true), just("false").to(false)))
      .map(Ast::Boolean)
      .map_with(|ast, e| (ast, e.span()));

    let double_quoted_string = just('"')
      .ignore_then(none_of('"').repeated().to_slice())
      .then_ignore(just('"'))
      .map(Ast::String)
      .map_with(|ast, e| (ast, e.span()));

    let single_quoted_string = just('\'')
      .ignore_then(none_of('\'').repeated().to_slice())
      .then_ignore(just('\''))
      .map(Ast::String)
      .map_with(|ast, e| (ast, e.span()));

    let string = double_quoted_string.or(single_quoted_string);

    let function_call = identifier
      .then(
        expr
          .clone()
          .separated_by(just(','))
          .allow_trailing()
          .collect::<Vec<_>>()
          .delimited_by(just('('), just(')')),
      )
      .map(|(name, arguments)| Ast::FunctionCall(name, arguments))
      .map_with(|ast, e| (ast, e.span()));

    let identifier = identifier
      .map(Ast::Identifier)
      .map_with(|ast, e| (ast, e.span()));

    let atom = number
      .or(boolean)
      .or(string)
      .or(expr.delimited_by(just('('), just(')')))
      .or(function_call)
      .or(identifier)
      .padded();

    let op = |c| just(c).padded();

    let unary = choice((op('-').to(UnaryOp::Negate), op('!').to(UnaryOp::Not)))
      .repeated()
      .foldr(atom, |op, rhs| {
        let span = rhs.1;
        (Ast::UnaryOp(op, Box::new(rhs)), span)
      });

    let product = unary.clone().foldl(
      choice((
        op('%').to(BinaryOp::Modulo),
        op('*').to(BinaryOp::Multiply),
        op('/').to(BinaryOp::Divide),
        op('^').to(BinaryOp::Power),
      ))
      .then(unary.clone())
      .repeated(),
      |lhs, (op, rhs)| {
        let span = (lhs.1.start..rhs.1.end).into();
        (Ast::BinaryOp(op, Box::new(lhs), Box::new(rhs)), span)
      },
    );

    let sum = product.clone().foldl(
      choice((op('+').to(BinaryOp::Add), op('-').to(BinaryOp::Subtract)))
        .then(product)
        .repeated(),
      |lhs, (op, rhs)| {
        let span = (lhs.1.start..rhs.1.end).into();
        (Ast::BinaryOp(op, Box::new(lhs), Box::new(rhs)), span)
      },
    );

    let comparison = sum.clone().foldl_with(
      just("==")
        .to(BinaryOp::Equal)
        .or(just("!=").to(BinaryOp::NotEqual))
        .or(just(">=").to(BinaryOp::GreaterThanEqual))
        .or(just("<=").to(BinaryOp::LessThanEqual))
        .or(just("<").to(BinaryOp::LessThan))
        .or(just(">").to(BinaryOp::GreaterThan))
        .then(sum)
        .repeated(),
      |a, (op, b), e| (Ast::BinaryOp(op, Box::new(a), Box::new(b)), e.span()),
    );

    comparison
  })
}

pub fn parse(input: &str) -> Result<Spanned<Ast<'_>>, Vec<Error>> {
  let result = parser().parse(input);

  match result.into_output_errors() {
    (Some(ast), errors) if errors.is_empty() => Ok(ast),
    (_, errors) => Err(
      errors
        .into_iter()
        .map(|error| Error::new(error.span().to_owned(), error.to_string()))
        .collect(),
    ),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Test<'a> {
    ast: &'a str,
    errors: Vec<Error>,
    program: &'a str,
  }

  impl<'a> Test<'a> {
    fn new() -> Self {
      Self {
        ast: "",
        errors: Vec::new(),
        program: "",
      }
    }

    fn ast(self, ast: &'a str) -> Self {
      Self { ast, ..self }
    }

    fn errors(self, errors: Vec<Error>) -> Self {
      Self { errors, ..self }
    }

    fn program(self, program: &'a str) -> Self {
      Self { program, ..self }
    }

    fn run(self) {
      match parse(self.program) {
        Ok(ast) => {
          assert_eq!(ast.0.to_string(), self.ast, "AST mismatch");
        }
        Err(errors) => {
          for (error, expected) in errors.iter().zip(self.errors.iter()) {
            assert_eq!(error, expected, "Error mismatch");
          }
        }
      }
    }
  }

  #[test]
  fn integer_literal() {
    Test::new().program("25").ast("25").run()
  }

  #[test]
  fn operator_precedence() {
    Test::new().program("2 + 3 * 4").ast("(+ 2 (* 3 4))").run();
    Test::new().program("2 * 3 + 4").ast("(+ (* 2 3) 4)").run();
    Test::new().program("2 * 3 / 4").ast("(/ (* 2 3) 4)").run();
    Test::new().program("2 ^ 3 * 4").ast("(* (^ 2 3) 4)").run();
    Test::new().program("!2 + 3").ast("(+ !2 3)").run();
  }

  #[test]
  fn whitespace_handling() {
    Test::new().program("  2  +  3  ").ast("(+ 2 3)").run();
    Test::new().program("\n5\n*\n2\n").ast("(* 5 2)").run();
    Test::new().program("\t8\t/\t4\t").ast("(/ 8 4)").run();
  }

  #[test]
  fn unclosed_string() {
    Test::new()
      .program("\"unclosed")
      .errors(vec![Error::new(
        SimpleSpan::from(9..9),
        "found end of input expected something else, or '\"'",
      )])
      .run();
  }

  #[test]
  fn invalid_operator() {
    Test::new()
      .program("2 +* 3")
      .errors(vec![Error::new(SimpleSpan::from(3..4), "found '*' expected '-', '!', non-zero digit, '0', 't', 'f', '\"', ''', '(', or identifier")])
      .run();
  }

  #[test]
  fn missing_closing_parenthesis() {
    Test::new()
      .program("(2 + 3")
      .errors(vec![Error::new(
        SimpleSpan::from(6..6),
        "found end of input expected any, '.', '%', '*', '/', '^', '+', '-', '=', '!', '>', '<', or ')'",
      )])
      .run();
  }
}
