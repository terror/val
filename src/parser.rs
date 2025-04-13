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

    let string = just('"')
      .ignore_then(none_of('"').repeated().to_slice())
      .then_ignore(just('"'))
      .map(Ast::String)
      .map_with(|ast, e| (ast, e.span()));

    let function_call = identifier
      .then(
        expr
          .clone()
          .separated_by(just(','))
          .allow_trailing()
          .collect::<Vec<_>>()
          .delimited_by(just('('), just(')')),
      )
      .map(|(f, args)| Ast::FunctionCall(f, args))
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

    let unary = choice((op('-').to(UnaryOp::Neg), op('!').to(UnaryOp::Not)))
      .repeated()
      .foldr(atom, |op, rhs| {
        let span = rhs.1;
        (Ast::UnaryOp(op, Box::new(rhs)), span)
      });

    let product = unary.clone().foldl(
      choice((
        op('%').to(BinaryOp::Mod),
        op('*').to(BinaryOp::Mul),
        op('/').to(BinaryOp::Div),
        op('<').to(BinaryOp::Lt),
        op('>').to(BinaryOp::Gt),
        op('^').to(BinaryOp::Pow),
      ))
      .then(unary.clone())
      .repeated(),
      |lhs, (op, rhs)| {
        let span = (lhs.1.start..rhs.1.end).into();
        (Ast::BinaryOp(op, Box::new(lhs), Box::new(rhs)), span)
      },
    );

    let sum = product.clone().foldl(
      choice((op('+').to(BinaryOp::Add), op('-').to(BinaryOp::Sub)))
        .then(product)
        .repeated(),
      |lhs, (op, rhs)| {
        let span = (lhs.1.start..rhs.1.end).into();
        (Ast::BinaryOp(op, Box::new(lhs), Box::new(rhs)), span)
      },
    );

    sum
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
