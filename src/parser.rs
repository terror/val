use super::*;

pub fn parse(input: &str) -> Result<Spanned<Program<'_>>, Vec<Error>> {
  let result = program_parser().parse(input);

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

fn program_parser<'a>()
-> impl Parser<'a, &'a str, Spanned<Program<'a>>, extra::Err<Rich<'a, char>>> + Clone
{
  let statement = statement_parser();

  statement
    .then(just(';').padded().or_not())
    .map(|(stmt, _)| stmt)
    .repeated()
    .collect::<Vec<_>>()
    .map(Program::Statements)
    .map_with(|ast, e| (ast, e.span()))
}

fn statement_parser<'a>()
-> impl Parser<'a, &'a str, Spanned<Statement<'a>>, extra::Err<Rich<'a, char>>>
+ Clone {
  let expression = expression_parser();

  recursive(|statement| {
    let assignment_statement = text::ident()
      .padded()
      .then_ignore(just('=').padded())
      .then(expression.clone())
      .map(|(name, expr)| Statement::Assignment(name, expr))
      .map_with(|ast, e| (ast, e.span()));

    let block_statement = statement
      .clone()
      .then(just(';').padded().or_not())
      .map(|(stmt, _)| stmt)
      .repeated()
      .collect::<Vec<_>>()
      .delimited_by(just('{').padded(), just('}').padded())
      .map(Statement::Block)
      .map_with(|ast, e| (ast, e.span()));

    let if_statement = just("if")
      .padded()
      .ignore_then(
        expression
          .clone()
          .delimited_by(just('(').padded(), just(')').padded()),
      )
      .then(
        statement
          .clone()
          .then(just(';').padded().or_not())
          .map(|(stmt, _)| stmt)
          .repeated()
          .collect::<Vec<_>>()
          .delimited_by(just('{').padded(), just('}').padded()),
      )
      .then(
        just("else")
          .padded()
          .ignore_then(
            statement
              .clone()
              .then(just(';').padded().or_not())
              .map(|(stmt, _)| stmt)
              .repeated()
              .collect::<Vec<_>>()
              .delimited_by(just('{').padded(), just('}').padded()),
          )
          .or_not(),
      )
      .map(|((condition, then_branch), else_branch)| {
        Statement::If(condition, then_branch, else_branch)
      })
      .map_with(|ast, e| (ast, e.span()));

    let while_statement = just("while")
      .padded()
      .ignore_then(
        expression
          .clone()
          .delimited_by(just('(').padded(), just(')').padded()),
      )
      .then(
        statement
          .clone()
          .then(just(';').padded().or_not())
          .map(|(stmt, _)| stmt)
          .repeated()
          .collect::<Vec<_>>()
          .delimited_by(just('{').padded(), just('}').padded()),
      )
      .map(|(condition, body)| Statement::While(condition, body))
      .map_with(|ast, e| (ast, e.span()));

    let expression_statement = expression
      .map(Statement::Expression)
      .map_with(|ast, e| (ast, e.span()));

    choice((
      assignment_statement,
      block_statement,
      if_statement,
      while_statement,
      expression_statement,
    ))
    .padded()
  })
}

fn expression_parser<'a>()
-> impl Parser<'a, &'a str, Spanned<Expression<'a>>, extra::Err<Rich<'a, char>>>
+ Clone {
  let identifier = text::ident().padded();

  recursive(|expression| {
    let number = text::int(10)
      .then(just('.').then(text::digits(10)).or_not())
      .to_slice()
      .from_str()
      .unwrapped()
      .map(Expression::Number)
      .map_with(|ast, e| (ast, e.span()));

    let boolean = choice((just("true").to(true), just("false").to(false)))
      .map(Expression::Boolean)
      .map_with(|ast, e| (ast, e.span()));

    let double_quoted_string = just('"')
      .ignore_then(none_of('"').repeated().to_slice())
      .then_ignore(just('"'))
      .map(Expression::String)
      .map_with(|ast, e| (ast, e.span()));

    let single_quoted_string = just('\'')
      .ignore_then(none_of('\'').repeated().to_slice())
      .then_ignore(just('\''))
      .map(Expression::String)
      .map_with(|ast, e| (ast, e.span()));

    let string = double_quoted_string.or(single_quoted_string);

    let function_call = identifier
      .then(
        expression
          .clone()
          .separated_by(just(','))
          .allow_trailing()
          .collect::<Vec<_>>()
          .delimited_by(just('('), just(')')),
      )
      .map(|(name, arguments)| Expression::FunctionCall(name, arguments))
      .map_with(|ast, e| (ast, e.span()));

    let identifier = identifier
      .map(Expression::Identifier)
      .map_with(|ast, e| (ast, e.span()));

    let items = expression
      .clone()
      .separated_by(just(','))
      .allow_trailing()
      .collect::<Vec<_>>();

    let list = items
      .clone()
      .map(Expression::List)
      .map_with(|ast, e| (ast, e.span()))
      .delimited_by(just('['), just(']'));

    let atom = number
      .or(boolean)
      .or(expression.clone().delimited_by(just('('), just(')')))
      .or(function_call)
      .or(identifier)
      .or(list)
      .or(string)
      .padded();

    let list_access = atom.clone().foldl(
      expression
        .clone()
        .delimited_by(just('['), just(']'))
        .repeated(),
      |list, index| {
        let span = (list.1.start..index.1.end).into();

        let expression =
          Expression::ListAccess(Box::new(list), Box::new(index));

        (expression, span)
      },
    );

    let op = |c| just(c).padded();

    let unary = choice((op('-').to(UnaryOp::Negate), op('!').to(UnaryOp::Not)))
      .repeated()
      .foldr(list_access, |op, rhs| {
        let span = rhs.1;
        (Expression::UnaryOp(op, Box::new(rhs)), span)
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
        (Expression::BinaryOp(op, Box::new(lhs), Box::new(rhs)), span)
      },
    );

    let sum = product.clone().foldl(
      choice((op('+').to(BinaryOp::Add), op('-').to(BinaryOp::Subtract)))
        .then(product)
        .repeated(),
      |lhs, (op, rhs)| {
        let span = (lhs.1.start..rhs.1.end).into();
        (Expression::BinaryOp(op, Box::new(lhs), Box::new(rhs)), span)
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
      |a, (op, b), e| {
        (Expression::BinaryOp(op, Box::new(a), Box::new(b)), e.span())
      },
    );

    comparison
  })
}

#[cfg(test)]
mod tests {
  use {super::*, pretty_assertions::assert_eq};

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
          assert_eq!(errors.len(), self.errors.len(), "Error count mismatch");

          for (error, expected) in errors.iter().zip(self.errors.iter()) {
            assert_eq!(error, expected, "Error mismatch");
          }
        }
      }
    }
  }

  #[test]
  fn integer_literal() {
    Test::new()
      .program("25")
      .ast("statements(expression(number(25)))")
      .run()
  }

  #[test]
  fn operator_precedence() {
    Test::new()
      .program("2 + 3 * 4")
      .ast("statements(expression(binary_op(+, number(2), binary_op(*, number(3), number(4)))))")
      .run();

    Test::new()
      .program("2 * 3 + 4")
      .ast("statements(expression(binary_op(+, binary_op(*, number(2), number(3)), number(4))))")
      .run();

    Test::new()
      .program("2 * 3 / 4")
      .ast("statements(expression(binary_op(/, binary_op(*, number(2), number(3)), number(4))))")
      .run();

    Test::new()
      .program("2 ^ 3 * 4")
      .ast("statements(expression(binary_op(*, binary_op(^, number(2), number(3)), number(4))))")
      .run();

    Test::new()
      .program("!2 + 3")
      .ast("statements(expression(binary_op(+, unary_op(!, number(2)), number(3))))")
      .run();
  }

  #[test]
  fn assignment() {
    Test::new()
      .program("x = 5")
      .ast("statements(assignment(x, number(5)))")
      .run();
  }

  #[test]
  fn whitespace_handling() {
    Test::new()
      .program("  2  +  3  ")
      .ast("statements(expression(binary_op(+, number(2), number(3))))")
      .run();

    Test::new()
      .program("\n5\n*\n2\n")
      .ast("statements(expression(binary_op(*, number(5), number(2))))")
      .run();

    Test::new()
      .program("\t8\t/\t4\t")
      .ast("statements(expression(binary_op(/, number(8), number(4))))")
      .run();
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
      .errors(vec![Error::new(SimpleSpan::from(3..4), "found '*' expected '-', '!', non-zero digit, '0', 't', 'f', '(', identifier, '[', '\"', or '''")])
      .run();
  }

  #[test]
  fn missing_closing_parenthesis() {
    Test::new()
      .program("(2 + 3")
      .errors(vec![Error::new(
        SimpleSpan::from(6..6),
        "found end of input expected any, '.', '[', '%', '*', '/', '^', '+', '-', '=', '!', '>', '<', or ')'",
      )])
      .run();
  }

  #[test]
  fn multiple_top_level_statements() {
    Test::new().program("1 + 2; 3 * 4").ast("statements(expression(binary_op(+, number(1), number(2))), expression(binary_op(*, number(3), number(4))))").run();
  }

  #[test]
  fn multiple_statements_in_block() {
    Test::new()
      .program("1 + 2; { 3 * 4; 5 - 6 }; 7")
      .ast("statements(expression(binary_op(+, number(1), number(2))), block(expression(binary_op(*, number(3), number(4))), expression(binary_op(-, number(5), number(6)))), expression(number(7)))")
      .run();
  }

  #[test]
  fn newline_separated_statements() {
    Test::new()
    .program("1 + 2\n3 * 4")
    .ast("statements(expression(binary_op(+, number(1), number(2))), expression(binary_op(*, number(3), number(4))))")
    .run();
  }

  #[test]
  fn while_loop() {
    Test::new()
    .program("while (x < 10) { x = x + 1; }")
    .ast("statements(while(binary_op(<, identifier(x), number(10)), block(assignment(x, binary_op(+, identifier(x), number(1))))))")
    .run();
  }

  #[test]
  fn nested_while_loops() {
    Test::new()
    .program("while (x < 10) { while (y < 5) { y = y + 1; }; x = x + 1; }")
    .ast("statements(while(binary_op(<, identifier(x), number(10)), block(while(binary_op(<, identifier(y), number(5)), block(assignment(y, binary_op(+, identifier(y), number(1))))), assignment(x, binary_op(+, identifier(x), number(1))))))")
    .run();
  }

  #[test]
  fn if_statement() {
    Test::new()
    .program("if (x > 5) { y = 10; }")
    .ast("statements(if(binary_op(>, identifier(x), number(5)), block(assignment(y, number(10)))))")
    .run();
  }

  #[test]
  fn if_else_statement() {
    Test::new()
    .program("if (x > 5) { y = 10; } else { y = 5; }")
    .ast("statements(if(binary_op(>, identifier(x), number(5)), block(assignment(y, number(10))), block(assignment(y, number(5)))))")
    .run();
  }

  #[test]
  fn nested_if_statements() {
    Test::new()
    .program("if (x > 5) { if (y > 2) { z = 1; } else { z = 2; } } else { z = 3; }")
    .ast("statements(if(binary_op(>, identifier(x), number(5)), block(if(binary_op(>, identifier(y), number(2)), block(assignment(z, number(1))), block(assignment(z, number(2))))), block(assignment(z, number(3)))))")
    .run();
  }
}
