use {
  super::*,
  chumsky::input::MapExtra,
  chumsky::pratt::{infix, left, postfix, prefix, right},
};

type ParserError<'a> = extra::Err<Rich<'a, char>>;

/// # Errors
///
/// Returns parser errors when input cannot be parsed into a complete program.
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
-> impl Parser<'a, &'a str, Spanned<Program<'a>>, ParserError<'a>> + Clone {
  statement_list_parser(statement_parser())
    .map(Program::Statements)
    .map_with(|ast, error| (ast, error.span()))
}

fn comma_separated_parser<'a, P, T>(
  parser: P,
) -> impl Parser<'a, &'a str, Vec<T>, ParserError<'a>> + Clone
where
  P: Parser<'a, &'a str, T, ParserError<'a>> + Clone,
{
  parser
    .separated_by(just(',').padded())
    .allow_trailing()
    .collect::<Vec<_>>()
}

fn index_parser<'a, P>(
  expression: P,
) -> impl Parser<'a, &'a str, (Spanned<Expression<'a>>, SimpleSpan), ParserError<'a>>
+ Clone
where
  P: Parser<'a, &'a str, Spanned<Expression<'a>>, ParserError<'a>> + Clone,
{
  expression
    .delimited_by(just('['), just(']'))
    .padded()
    .map_with(|expression, error| (expression, error.span()))
}

fn keyword_parser<'a>(
  keyword: &'static str,
) -> impl Parser<'a, &'a str, (), ParserError<'a>> + Clone {
  text::keyword(keyword).padded().ignored()
}

fn statement_list_parser<'a, P>(
  statement: P,
) -> impl Parser<'a, &'a str, Vec<Spanned<Statement<'a>>>, ParserError<'a>> + Clone
where
  P: Parser<'a, &'a str, Spanned<Statement<'a>>, ParserError<'a>> + Clone,
{
  statement
    .then(just(';').padded().or_not())
    .map(|(statement, _)| statement)
    .repeated()
    .collect::<Vec<_>>()
}

fn statement_parser<'a>()
-> impl Parser<'a, &'a str, Spanned<Statement<'a>>, ParserError<'a>> + Clone {
  let expression = expression_parser();

  recursive(|statement| {
    let statement_block = statement_list_parser(statement.clone())
      .delimited_by(just('{').padded(), just('}').padded());

    let simple_ident = text::ident().padded().map_with(|name, error| {
      let span = error.span();
      (AssignmentTarget::Identifier(name), span)
    });

    let assignment_target = simple_ident.foldl(
      index_parser(expression.clone()).repeated(),
      |base, (index, span)| {
        let span = (base.1.start..span.end).into();

        let target =
          AssignmentTarget::ListAccess(Box::new(base), Box::new(index));

        (target, span)
      },
    );

    let assignment_statement = assignment_target
      .then_ignore(just('=').padded())
      .then(expression.clone())
      .map(|(lhs, rhs)| Statement::Assignment(lhs, rhs))
      .map_with(|ast, error| (ast, error.span()));

    let function_statement = keyword_parser("fn")
      .ignore_then(text::ident().padded())
      .then(
        comma_separated_parser(text::ident().padded())
          .delimited_by(just('(').padded(), just(')').padded()),
      )
      .then(statement_block.clone())
      .map(|((name, params), body)| Statement::Function(name, params, body))
      .map_with(|ast, error| (ast, error.span()));

    let block_statement = statement_block
      .clone()
      .map(Statement::Block)
      .map_with(|ast, error| (ast, error.span()));

    let condition_parser = expression
      .clone()
      .delimited_by(just('(').padded(), just(')').padded());

    let if_statement = keyword_parser("if")
      .ignore_then(condition_parser.clone())
      .then(statement_block.clone())
      .then(
        keyword_parser("else")
          .ignore_then(statement_block.clone())
          .or_not(),
      )
      .map(|((condition, then_branch), else_branch)| {
        Statement::If(condition, then_branch, else_branch)
      })
      .map_with(|ast, error| (ast, error.span()));

    let while_statement = keyword_parser("while")
      .ignore_then(condition_parser)
      .then(statement_block.clone())
      .map(|(condition, body)| Statement::While(condition, body))
      .map_with(|ast, error| (ast, error.span()));

    let for_statement = keyword_parser("for")
      .ignore_then(text::ident().padded())
      .then_ignore(keyword_parser("in"))
      .then(expression.clone())
      .then(statement_block.clone())
      .map(|((name, iterable), body)| Statement::For(name, iterable, body))
      .map_with(|ast, error| (ast, error.span()));

    let loop_statement = keyword_parser("loop")
      .ignore_then(statement_block.clone())
      .map(Statement::Loop)
      .map_with(|ast, error| (ast, error.span()));

    let return_statement = keyword_parser("return")
      .ignore_then(expression.clone().or_not())
      .map(Statement::Return)
      .map_with(|ast, error| (ast, error.span()));

    let break_statement = keyword_parser("break")
      .map(|()| Statement::Break)
      .map_with(|ast, error| (ast, error.span()));

    let continue_statement = keyword_parser("continue")
      .map(|()| Statement::Continue)
      .map_with(|ast, error| (ast, error.span()));

    let expression_statement = expression
      .map(Statement::Expression)
      .map_with(|ast, error| (ast, error.span()));

    choice((
      assignment_statement,
      function_statement,
      block_statement,
      if_statement,
      while_statement,
      for_statement,
      loop_statement,
      return_statement,
      break_statement,
      continue_statement,
      expression_statement,
    ))
    .padded()
    .boxed()
  })
}

fn expression_parser<'a>()
-> impl Parser<'a, &'a str, Spanned<Expression<'a>>, ParserError<'a>> + Clone {
  let identifier = text::ident().padded();

  recursive(|expression| {
    let number = text::int(10)
      .then(just('.').then(text::digits(10)).or_not())
      .to_slice()
      .map(|number| Number::try_from(number).unwrap())
      .map(Expression::Number)
      .map_with(|ast, error| (ast, error.span()));

    let boolean = choice((
      keyword_parser("true").to(true),
      keyword_parser("false").to(false),
    ))
    .map(Expression::Boolean)
    .map_with(|ast, error| (ast, error.span()));

    let null = keyword_parser("null")
      .map(|()| Expression::Null)
      .map_with(|ast, error| (ast, error.span()));

    let double_quoted_string = just('"')
      .ignore_then(none_of('"').repeated().to_slice())
      .then_ignore(just('"'))
      .map(Expression::String)
      .map_with(|ast, error| (ast, error.span()));

    let single_quoted_string = just('\'')
      .ignore_then(none_of('\'').repeated().to_slice())
      .then_ignore(just('\''))
      .map(Expression::String)
      .map_with(|ast, error| (ast, error.span()));

    let string = double_quoted_string.or(single_quoted_string);

    let function_call = identifier
      .then(
        comma_separated_parser(expression.clone())
          .delimited_by(just('('), just(')')),
      )
      .map(|(name, arguments)| Expression::FunctionCall(name, arguments))
      .map_with(|ast, error| (ast, error.span()));

    let identifier = identifier
      .map(Expression::Identifier)
      .map_with(|ast, error| (ast, error.span()));

    let list = comma_separated_parser(expression.clone())
      .delimited_by(just('['), just(']'))
      .map(Expression::List)
      .map_with(|ast, error| (ast, error.span()));

    let atom = number
      .or(boolean)
      .or(null)
      .or(expression.clone().delimited_by(just('('), just(')')))
      .or(function_call)
      .or(list)
      .or(identifier)
      .or(string)
      .padded();

    let binary =
      |lhs: Spanned<Expression<'a>>,
       op: BinaryOp,
       rhs: Spanned<Expression<'a>>,
       error: &mut MapExtra<'a, '_, &'a str, ParserError<'a>>| {
        (
          Expression::BinaryOp(op, Box::new(lhs), Box::new(rhs)),
          error.span(),
        )
      };

    let unary =
      |op: UnaryOp,
       rhs: Spanned<Expression<'a>>,
       error: &mut MapExtra<'a, '_, &'a str, ParserError<'a>>| {
        (Expression::UnaryOp(op, Box::new(rhs)), error.span())
      };

    atom.pratt((
      postfix(
        8,
        index_parser(expression.clone()),
        |list,
         (index, _),
         error: &mut MapExtra<'a, '_, &'a str, ParserError<'a>>| {
          let span = error.span();

          let expression =
            Expression::ListAccess(Box::new(list), Box::new(index));

          (expression, span)
        },
      ),
      prefix(7, just('-').padded().to(UnaryOp::Negate), unary),
      prefix(7, just('!').padded().to(UnaryOp::Not), unary),
      infix(right(6), just('^').padded().to(BinaryOp::Power), binary),
      infix(
        left(5),
        choice((
          just('%').padded().to(BinaryOp::Modulo),
          just('*').padded().to(BinaryOp::Multiply),
          just('/').padded().to(BinaryOp::Divide),
        )),
        binary,
      ),
      infix(
        left(4),
        choice((
          just('+').padded().to(BinaryOp::Add),
          just('-').padded().to(BinaryOp::Subtract),
        )),
        binary,
      ),
      infix(
        left(3),
        choice((
          just(">=").padded().to(BinaryOp::GreaterThanEqual),
          just("<=").padded().to(BinaryOp::LessThanEqual),
          just(">").padded().to(BinaryOp::GreaterThan),
          just("<").padded().to(BinaryOp::LessThan),
        )),
        binary,
      ),
      infix(
        left(2),
        choice((
          just("==").padded().to(BinaryOp::Equal),
          just("!=").padded().to(BinaryOp::NotEqual),
        )),
        binary,
      ),
      infix(
        left(1),
        just("&&").padded().to(BinaryOp::LogicalAnd),
        binary,
      ),
      infix(left(0), just("||").padded().to(BinaryOp::LogicalOr), binary),
    ))
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
    fn ast(self, ast: &'a str) -> Self {
      Self { ast, ..self }
    }

    fn errors(self, errors: Vec<Error>) -> Self {
      Self { errors, ..self }
    }

    fn new() -> Self {
      Self {
        ast: "",
        errors: Vec::new(),
        program: "",
      }
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
  fn assignment() {
    Test::new()
      .program("x = 5")
      .ast("statements(assignment(identifier(x), number(5)))")
      .run();

    Test::new()
      .program("foo[0][1] = bar")
      .ast("statements(assignment(list_access(list_access(identifier(foo), number(0)), number(1)), identifier(bar)))")
      .run();
  }

  #[test]
  fn break_statement() {
    Test::new().program("break").ast("statements(break)").run();
  }

  #[test]
  fn continue_statement() {
    Test::new()
      .program("continue")
      .ast("statements(continue)")
      .run();
  }

  #[test]
  fn for_loop() {
    Test::new()
      .program("for x in [1, 2, 3] { println(x) }")
      .ast("statements(for(x, list(number(1), number(2), number(3)), block(expression(function_call(println,identifier(x))))))")
      .run();
  }

  #[test]
  fn function_with_return() {
    Test::new()
    .program("fn add(a, b) { return a + b; }")
    .ast("statements(function(add, [a, b], block(return(binary_op(+, identifier(a), identifier(b))))))")
    .run();
  }

  #[test]
  fn if_else_statement() {
    Test::new()
    .program("if (x > 5) { y = 10; } else { y = 5; }")
    .ast("statements(if(binary_op(>, identifier(x), number(5)), block(assignment(identifier(y), number(10))), block(assignment(identifier(y), number(5)))))")
    .run();
  }

  #[test]
  fn if_statement() {
    Test::new()
    .program("if (x > 5) { y = 10; }")
    .ast("statements(if(binary_op(>, identifier(x), number(5)), block(assignment(identifier(y), number(10)))))")
    .run();
  }

  #[test]
  fn integer_literal() {
    Test::new()
      .program("25")
      .ast("statements(expression(number(25)))")
      .run();
  }

  #[test]
  fn invalid_operator() {
    Test::new()
      .program("2 +* 3")
      .errors(vec![Error::new(
        SimpleSpan::from(2..3),
        "found '+' expected identifier, '(', '[', '\"', ''', or end of input",
      )])
      .run();
  }

  #[test]
  fn list_access() {
    Test::new()
      .program("a = [1, 2, 3]; a[0]")
      .ast("statements(assignment(identifier(a), list(number(1), number(2), number(3))), expression(list_access(identifier(a), number(0))))")
      .run();
  }

  #[test]
  fn list_access_with_comparison() {
    Test::new()
      .program("a = [1, 2, 3]; a[0] == 1")
      .ast("statements(assignment(identifier(a), list(number(1), number(2), number(3))), expression(binary_op(==, list_access(identifier(a), number(0)), number(1))))")
      .run();
  }

  #[test]
  fn list_access_with_expressions() {
    Test::new()
      .program("a = [1, 2, 3]; a[1 + 1]")
      .ast("statements(assignment(identifier(a), list(number(1), number(2), number(3))), expression(list_access(identifier(a), binary_op(+, number(1), number(1)))))")
      .run();
  }

  #[test]
  fn loop_statement() {
    Test::new()
    .program("loop { x = x + 1; }")
    .ast("statements(loop(block(assignment(identifier(x), binary_op(+, identifier(x), number(1))))))")
    .run();
  }

  #[test]
  fn loop_with_break() {
    Test::new()
    .program("loop { if (x > 10) { break; }; x = x + 1; }")
    .ast("statements(loop(block(if(binary_op(>, identifier(x), number(10)), block(break)), assignment(identifier(x), binary_op(+, identifier(x), number(1))))))")
    .run();
  }

  #[test]
  fn loop_with_continue() {
    Test::new()
    .program("loop { if (x % 2 == 0) { continue; }; println(x); x = x + 1; }")
    .ast("statements(loop(block(if(binary_op(==, binary_op(%, identifier(x), number(2)), number(0)), block(continue)), expression(function_call(println,identifier(x))), assignment(identifier(x), binary_op(+, identifier(x), number(1))))))")
    .run();
  }

  #[test]
  fn missing_closing_parenthesis() {
    Test::new()
      .program("(2 + 3")
      .errors(vec![Error::new(
        SimpleSpan::from(6..6),
        "found end of input expected any, '.', '[', '^', '%', '*', '/', '+', '-', '>', '<', '=', '!', '&', '|', or ')'",
      )])
      .run();
  }
  #[test]
  fn multiple_statements_in_block() {
    Test::new()
      .program("1 + 2; { 3 * 4; 5 - 6 }; 7")
      .ast("statements(expression(binary_op(+, number(1), number(2))), block(expression(binary_op(*, number(3), number(4))), expression(binary_op(-, number(5), number(6)))), expression(number(7)))")
      .run();
  }

  #[test]
  fn multiple_top_level_statements() {
    Test::new().program("1 + 2; 3 * 4").ast("statements(expression(binary_op(+, number(1), number(2))), expression(binary_op(*, number(3), number(4))))").run();
  }

  #[test]
  fn nested_if_statements() {
    Test::new()
    .program("if (x > 5) { if (y > 2) { z = 1; } else { z = 2; } } else { z = 3; }")
    .ast("statements(if(binary_op(>, identifier(x), number(5)), block(if(binary_op(>, identifier(y), number(2)), block(assignment(identifier(z), number(1))), block(assignment(identifier(z), number(2))))), block(assignment(identifier(z), number(3)))))")
    .run();
  }

  #[test]
  fn nested_list_access() {
    Test::new()
      .program("a = [[1, 2], [3, 4]]; a[0][1]")
      .ast("statements(assignment(identifier(a), list(list(number(1), number(2)), list(number(3), number(4)))), expression(list_access(list_access(identifier(a), number(0)), number(1))))")
      .run();
  }

  #[test]
  fn nested_while_loops() {
    Test::new()
    .program("while (x < 10) { while (y < 5) { y = y + 1; }; x = x + 1; }")
    .ast("statements(while(binary_op(<, identifier(x), number(10)), block(while(binary_op(<, identifier(y), number(5)), block(assignment(identifier(y), binary_op(+, identifier(y), number(1))))), assignment(identifier(x), binary_op(+, identifier(x), number(1))))))")
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
  fn power_right_associativity() {
    Test::new()
      .program("2 ^ 2 ^ 2 ^ 2")
      .ast("statements(expression(binary_op(^, number(2), binary_op(^, number(2), binary_op(^, number(2), number(2))))))")
      .run();

    Test::new()
      .program("2 ^ (2 ^ (2 ^ 2))")
      .ast("statements(expression(binary_op(^, number(2), binary_op(^, number(2), binary_op(^, number(2), number(2))))))")
      .run();

    Test::new()
      .program("((2 ^ 2) ^ 2) ^ 2")
      .ast("statements(expression(binary_op(^, binary_op(^, binary_op(^, number(2), number(2)), number(2)), number(2))))")
      .run();
  }

  #[test]
  fn return_statement() {
    Test::new()
      .program("return 5")
      .ast("statements(return(number(5)))")
      .run();

    Test::new()
      .program("return")
      .ast("statements(return())")
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
  fn while_loop() {
    Test::new()
    .program("while (x < 10) { x = x + 1; }")
    .ast("statements(while(binary_op(<, identifier(x), number(10)), block(assignment(identifier(x), binary_op(+, identifier(x), number(1))))))")
    .run();
  }

  #[test]
  fn while_with_break() {
    Test::new()
    .program("while (x < 10) { if (x == 5) { break; }; x = x + 1; }")
    .ast("statements(while(binary_op(<, identifier(x), number(10)), block(if(binary_op(==, identifier(x), number(5)), block(break)), assignment(identifier(x), binary_op(+, identifier(x), number(1))))))")
    .run();
  }

  #[test]
  fn while_with_continue() {
    Test::new()
    .program("while (x < 10) { if (x % 2 == 0) { continue; }; println(x); x = x + 1; }")
    .ast("statements(while(binary_op(<, identifier(x), number(10)), block(if(binary_op(==, binary_op(%, identifier(x), number(2)), number(0)), block(continue)), expression(function_call(println,identifier(x))), assignment(identifier(x), binary_op(+, identifier(x), number(1))))))")
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
}
