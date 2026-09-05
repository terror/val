use {
  super::*,
  chumsky::input::MapExtra,
  chumsky::pratt::{infix, left, postfix, prefix, right},
};

type ParserError<'a> = extra::Err<Rich<'a, char>>;

const RESERVED_WORDS: [&str; 13] = [
  "break", "continue", "else", "false", "fn", "for", "if", "in", "loop",
  "null", "return", "true", "while",
];

/// # Errors
///
/// Returns parser errors when input cannot be parsed into a complete program.
pub fn parse(input: &str) -> Result<Spanned<Program>, Vec<Error>> {
  program_parser()
    .parse(input)
    .into_result()
    .map_err(|errors| {
      errors
        .into_iter()
        .map(|error| Error::new(error.span().to_owned(), error.to_string()))
        .collect()
    })
}

fn program_parser<'a>()
-> impl Parser<'a, &'a str, Spanned<Program>, ParserError<'a>> + Clone {
  padding_parser()
    .ignore_then(statement_list_parser(statement_parser()))
    .then_ignore(padding_parser())
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
    .separated_by(padded_parser(just(',')))
    .allow_trailing()
    .collect::<Vec<_>>()
}

fn index_parser<'a, P>(
  expression: P,
) -> impl Parser<'a, &'a str, (Spanned<Expression>, SimpleSpan), ParserError<'a>>
+ Clone
where
  P: Parser<'a, &'a str, Spanned<Expression>, ParserError<'a>> + Clone,
{
  expression
    .delimited_by(padded_parser(just('[')), padded_parser(just(']')))
    .padded_by(padding_parser())
    .map_with(|expression, error| (expression, error.span()))
}

fn identifier_parser<'a>()
-> impl Parser<'a, &'a str, String, ParserError<'a>> + Clone {
  padded_parser(text::ident().try_map(|identifier, span| {
    if RESERVED_WORDS.contains(&identifier) {
      Err(Rich::custom(
        span,
        format!("`{identifier}` is a reserved word"),
      ))
    } else {
      Ok(identifier.to_owned())
    }
  }))
}

fn keyword_parser<'a>(
  keyword: &'static str,
) -> impl Parser<'a, &'a str, (), ParserError<'a>> + Clone {
  padded_parser(text::keyword(keyword)).ignored()
}

fn padded_parser<'a, P, T>(
  parser: P,
) -> impl Parser<'a, &'a str, T, ParserError<'a>> + Clone
where
  P: Parser<'a, &'a str, T, ParserError<'a>> + Clone,
{
  parser.padded_by(padding_parser())
}

fn padding_parser<'a>() -> impl Parser<'a, &'a str, (), ParserError<'a>> + Clone
{
  custom(|input| {
    loop {
      let checkpoint = input.save();

      match input.next() {
        Some(character) if character.is_whitespace() => {}
        Some('/') if input.peek() == Some('/') => {
          input.next();

          while input.peek().is_some_and(|character| character != '\n') {
            input.next();
          }
        }
        _ => {
          input.rewind(checkpoint);
          break;
        }
      }
    }

    Ok(())
  })
}

fn statement_list_parser<'a, P>(
  statement: P,
) -> impl Parser<'a, &'a str, Vec<Spanned<Statement>>, ParserError<'a>> + Clone
where
  P: Parser<'a, &'a str, Spanned<Statement>, ParserError<'a>> + Clone,
{
  statement
    .then_ignore(padded_parser(just(';')).or_not())
    .repeated()
    .collect::<Vec<_>>()
}

fn statement_parser<'a>()
-> impl Parser<'a, &'a str, Spanned<Statement>, ParserError<'a>> + Clone {
  recursive(|statement| {
    let statement_block = statement_list_parser(statement.clone())
      .delimited_by(padded_parser(just('{')), padded_parser(just('}')));

    let expression = expression_parser(statement_block.clone());

    let simple_ident = identifier_parser().map_with(|name, error| {
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
      .then_ignore(padded_parser(just('=')))
      .then(expression.clone())
      .map(|(lhs, rhs)| Statement::Assignment(lhs, rhs));

    let function_statement = keyword_parser("fn")
      .ignore_then(identifier_parser())
      .then(
        comma_separated_parser(identifier_parser())
          .delimited_by(padded_parser(just('(')), padded_parser(just(')'))),
      )
      .then(statement_block.clone())
      .map(|((name, params), body)| Statement::Function(name, params, body));

    let block_statement = statement_block.clone().map(Statement::Block);

    let condition_parser = expression
      .clone()
      .delimited_by(padded_parser(just('(')), padded_parser(just(')')));

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
      });

    let while_statement = keyword_parser("while")
      .ignore_then(condition_parser)
      .then(statement_block.clone())
      .map(|(condition, body)| Statement::While(condition, body));

    let for_statement = keyword_parser("for")
      .ignore_then(identifier_parser())
      .then_ignore(keyword_parser("in"))
      .then(expression.clone())
      .then(statement_block.clone())
      .map(|((name, iterable), body)| Statement::For(name, iterable, body));

    let loop_statement = keyword_parser("loop")
      .ignore_then(statement_block.clone())
      .map(Statement::Loop);

    let return_statement = keyword_parser("return")
      .ignore_then(expression.clone().or_not())
      .map(Statement::Return);

    let break_statement = keyword_parser("break").map(|()| Statement::Break);

    let continue_statement =
      keyword_parser("continue").map(|()| Statement::Continue);

    let expression_statement = expression.map(Statement::Expression);

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
    .map_with(|ast, error| (ast, error.span()))
    .padded_by(padding_parser())
    .boxed()
  })
}

fn expression_parser<'a, P>(
  statement_block: P,
) -> impl Parser<'a, &'a str, Spanned<Expression>, ParserError<'a>> + Clone
where
  P: Parser<'a, &'a str, Vec<Spanned<Statement>>, ParserError<'a>> + Clone,
  P: 'a,
{
  let identifier = identifier_parser();

  recursive(|expression| {
    let number = text::int(10)
      .then(just('.').then(text::digits(10)).or_not())
      .then(
        one_of("eE")
          .then(one_of("+-").or_not())
          .then(text::digits(10))
          .or_not(),
      )
      .to_slice()
      .then_ignore(any().filter(text::Char::is_ident_continue).not())
      .try_map(|number: &str, span| {
        Number::try_from(number)
          .map_err(|error| Rich::custom(span, error.to_string()))
      })
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
      .map(str::to_owned)
      .map(Expression::String)
      .map_with(|ast, error| (ast, error.span()));

    let single_quoted_string = just('\'')
      .ignore_then(none_of('\'').repeated().to_slice())
      .then_ignore(just('\''))
      .map(str::to_owned)
      .map(Expression::String)
      .map_with(|ast, error| (ast, error.span()));

    let string = double_quoted_string.or(single_quoted_string);

    let arguments = comma_separated_parser(expression.clone())
      .delimited_by(padded_parser(just('(')), padded_parser(just(')')))
      .padded_by(padding_parser())
      .map_with(|arguments, error| (arguments, error.span()));

    let function = keyword_parser("fn")
      .ignore_then(
        comma_separated_parser(identifier_parser())
          .delimited_by(padded_parser(just('(')), padded_parser(just(')'))),
      )
      .then(statement_block.clone())
      .map(|(params, body)| Expression::Function(params, body))
      .map_with(|ast, error| (ast, error.span()));

    let identifier = identifier
      .map(Expression::Identifier)
      .map_with(|ast, error| (ast, error.span()));

    let list = comma_separated_parser(expression.clone())
      .delimited_by(padded_parser(just('[')), padded_parser(just(']')))
      .map(Expression::List)
      .map_with(|ast, error| (ast, error.span()));

    let atom = number
      .or(boolean)
      .or(null)
      .or(expression.clone().delimited_by(just('('), just(')')))
      .or(function)
      .or(list)
      .or(identifier)
      .or(string)
      .padded_by(padding_parser());

    let binary =
      |lhs: Spanned<Expression>,
       op: BinaryOp,
       rhs: Spanned<Expression>,
       error: &mut MapExtra<'a, '_, &'a str, ParserError<'a>>| {
        (
          Expression::BinaryOp(op, Box::new(lhs), Box::new(rhs)),
          error.span(),
        )
      };

    let unary =
      |op: UnaryOp,
       rhs: Spanned<Expression>,
       error: &mut MapExtra<'a, '_, &'a str, ParserError<'a>>| {
        (Expression::UnaryOp(op, Box::new(rhs)), error.span())
      };

    let operand = atom.pratt((
      postfix(
        8,
        arguments,
        |function,
         (arguments, _),
         error: &mut MapExtra<'a, '_, &'a str, ParserError<'a>>| {
          let span = error.span();

          let expression =
            Expression::FunctionCall(Box::new(function), arguments);

          (expression, span)
        },
      ),
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
      prefix(7, padded_parser(just('-')).to(UnaryOp::Negate), unary),
      prefix(7, padded_parser(just('!')).to(UnaryOp::Not), unary),
      infix(
        right(6),
        padded_parser(just('^')).to(BinaryOp::Power),
        binary,
      ),
      infix(
        left(5),
        choice((
          padded_parser(just('%')).to(BinaryOp::Modulo),
          padded_parser(just('*')).to(BinaryOp::Multiply),
          padded_parser(just('/')).to(BinaryOp::Divide),
        )),
        binary,
      ),
      infix(
        left(4),
        choice((
          padded_parser(just('+')).to(BinaryOp::Add),
          padded_parser(just('-')).to(BinaryOp::Subtract),
        )),
        binary,
      ),
      infix(
        left(3),
        choice((
          padded_parser(just(">=")).to(BinaryOp::GreaterThanEqual),
          padded_parser(just("<=")).to(BinaryOp::LessThanEqual),
          padded_parser(just(">")).to(BinaryOp::GreaterThan),
          padded_parser(just("<")).to(BinaryOp::LessThan),
        )),
        binary,
      ),
      infix(
        left(2),
        choice((
          padded_parser(just("==")).to(BinaryOp::Equal),
          padded_parser(just("!=")).to(BinaryOp::NotEqual),
        )),
        binary,
      ),
      infix(
        left(1),
        padded_parser(just("&&")).to(BinaryOp::LogicalAnd),
        binary,
      ),
      infix(
        left(0),
        padded_parser(just("||")).to(BinaryOp::LogicalOr),
        binary,
      ),
    ));

    let target =
      operand
        .clone()
        .try_map(|(expression, span), _| match expression {
          Expression::FunctionCall(function, arguments) => {
            Ok((function, arguments))
          }
          _ => Err(Rich::custom(span, "Pipe target must be a function call")),
        });

    operand.foldl_with(
      padded_parser(just("|>")).ignore_then(target).repeated(),
      |lhs, (function, mut arguments), error| {
        arguments.insert(0, lhs);

        (Expression::FunctionCall(function, arguments), error.span())
      },
    )
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
  fn comments() {
    Test::new()
      .program("// foo\n// bar\n")
      .ast("statements()")
      .run();

    Test::new()
      .program("// foo\na = [1, // bar\n 2,]\na[// baz\n0] + 3 // bob")
      .ast("statements(assignment(identifier(a), list(number(1), number(2))), expression(binary_op(+, list_access(identifier(a), number(0)), number(3))))")
      .run();
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
      .ast("statements(for(x, list(number(1), number(2), number(3)), block(expression(function_call(identifier(println), identifier(x))))))")
      .run();
  }

  #[test]
  fn function_call() {
    Test::new()
      .program("make_counter(0)(1)")
      .ast("statements(expression(function_call(function_call(identifier(make_counter), number(0)), number(1))))")
      .run();

    Test::new()
      .program("f[0](x)")
      .ast("statements(expression(function_call(list_access(identifier(f), number(0)), identifier(x))))")
      .run();
  }

  #[test]
  fn function_expression() {
    Test::new()
      .program("fn(x) { return x + 1 }")
      .ast("statements(expression(function([x], block(return(binary_op(+, identifier(x), number(1)))))))")
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
  fn invalid_number_literals() {
    #[track_caller]
    fn case(program: &str) {
      assert!(parse(program).is_err(), "{program}");
    }

    case("01");
    case("1foo");
    case("1e");
    case("1E+");
    case("1e-");
    case("1e--2");
    case("1e+-2");
    case("1e+ 2");
    case("1e.2");
    case("1e2e3");
    case("1e2foo");
    case("1e4294967296");
    case("1e-4294967296");
    case("1e9223372036854775808");
    case("1.0e-9223372036854775808");
  }

  #[test]
  fn invalid_operator() {
    Test::new()
      .program("2 +* 3")
      .errors(vec![Error::new(
        SimpleSpan::from(3..4),
        "found '*' expected '-', '!', int, '\"true\"', '\"false\"', '\"null\"', '(', '\"fn\"', '[', identifier, '\"', or '''",
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
    .ast("statements(loop(block(if(binary_op(==, binary_op(%, identifier(x), number(2)), number(0)), block(continue)), expression(function_call(identifier(println), identifier(x))), assignment(identifier(x), binary_op(+, identifier(x), number(1))))))")
    .run();
  }

  #[test]
  fn missing_closing_parenthesis() {
    Test::new()
      .program("(2 + 3")
      .errors(vec![Error::new(
        SimpleSpan::from(6..6),
        "found end of input expected any, '.', 'e', 'E', '(', '[', '^', '%', '*', '/', '+', '-', '>', '<', '=', '!', '&', '|', or ')'",
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
  fn pipe_operator() {
    #[track_caller]
    fn case(program: &str, ast: &str) {
      Test::new().program(program).ast(ast).run();
    }

    case(
      "25 |> sqrt() |> println()",
      "statements(expression(function_call(identifier(println), function_call(identifier(sqrt), number(25)))))",
    );
    case(
      "1 + 2 * 3 |> foo(bar())",
      "statements(expression(function_call(identifier(foo), binary_op(+, number(1), binary_op(*, number(2), number(3))), function_call(identifier(bar)))))",
    );
    case(
      "false || true && 1 < 2 == true |> foo()",
      "statements(expression(function_call(identifier(foo), binary_op(||, boolean(false), binary_op(&&, boolean(true), binary_op(==, binary_op(<, number(1), number(2)), boolean(true)))))))",
    );
    case(
      "1 |> foo(2)(3)",
      "statements(expression(function_call(function_call(identifier(foo), number(2)), number(1), number(3))))",
    );
    case(
      "1 |> foo[0](2)",
      "statements(expression(function_call(list_access(identifier(foo), number(0)), number(1), number(2))))",
    );
    case(
      "1 |> foo(2 |> bar())",
      "statements(expression(function_call(identifier(foo), number(1), function_call(identifier(bar), number(2)))))",
    );
    case(
      "(1 |> foo()) + 2",
      "statements(expression(binary_op(+, function_call(identifier(foo), number(1)), number(2))))",
    );
    case(
      "1\n// foo\n|> bar()\n|> baz()",
      "statements(expression(function_call(identifier(baz), function_call(identifier(bar), number(1)))))",
    );
  }

  #[test]
  fn pipe_operator_requires_call() {
    #[track_caller]
    fn case(program: &str) {
      let errors = parse(program).unwrap_err();

      assert!(
        errors.iter().any(|error| {
          error.to_string() == "Pipe target must be a function call"
        }),
        "{program}: {errors:?}",
      );
    }

    case("1 |> foo");
    case("1 |> 2");
    case("1 |> foo() + 2");
    case("1 |> foo()[0]");
  }

  #[test]
  fn pipe_operator_spans() {
    let program = parse("1 |> foo(2)").unwrap();
    let Program::Statements(statements) = program.0;
    let Statement::Expression((
      Expression::FunctionCall(function, arguments),
      span,
    )) = &statements[0].0
    else {
      panic!("expected function call");
    };

    assert_eq!(*span, SimpleSpan::from(0..11));
    assert_eq!(function.1, SimpleSpan::from(5..8));
    assert_eq!(arguments[0].1, SimpleSpan::from(0..1));
    assert_eq!(arguments[1].1, SimpleSpan::from(9..10));
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
  fn reserved_words_are_not_identifiers() {
    #[track_caller]
    fn case(program: &str, word: &str, start: usize) {
      assert_eq!(
        parse(program).unwrap_err(),
        [Error::new(
          SimpleSpan::from(start..start + word.len()),
          format!("`{word}` is a reserved word"),
        )],
      );
    }

    for word in RESERVED_WORDS {
      case(&format!("fn foo({word}) {{}}"), word, 7);
    }

    for program in [
      "true = 1",
      "fn true() {}",
      "for true in [] {}",
      "fn(true) {}",
    ] {
      assert!(parse(program).is_err());
    }
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
  fn scientific_notation() {
    #[track_caller]
    fn case(program: &str, number: &str) {
      Test::new()
        .program(program)
        .ast(&format!("statements(expression(number({number})))"))
        .run();
    }

    case("1e-05", "1e-05");
    case("1E+05", "100000");
    case("1.5e2", "150");
    case("1e0", "1");
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
    .ast("statements(while(binary_op(<, identifier(x), number(10)), block(if(binary_op(==, binary_op(%, identifier(x), number(2)), number(0)), block(continue)), expression(function_call(identifier(println), identifier(x))), assignment(identifier(x), binary_op(+, identifier(x), number(1))))))")
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
