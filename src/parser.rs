use super::*;

fn parser<'a>()
-> impl Parser<'a, &'a str, Spanned<Ast<'a>>, extra::Err<Rich<'a, char>>> {
  let identifier = text::ident().padded();

  recursive(|expr| {
    let number = text::int(10)
      .map(|s: &str| Ast::Number(s.parse().unwrap()))
      .map_with(|ast, e| (ast, e.span()));

    let call = identifier
      .then(
        expr
          .clone()
          .separated_by(just(','))
          .allow_trailing()
          .collect::<Vec<_>>()
          .delimited_by(just('('), just(')')),
      )
      .map(|(f, args)| Ast::Call(f, args))
      .map_with(|ast, e| (ast, e.span()));

    let identifier = identifier
      .map(Ast::Identifier)
      .map_with(|ast, e| (ast, e.span()));

    let atom = number
      .or(expr.delimited_by(just('('), just(')')))
      .or(call)
      .or(identifier)
      .padded();

    let op = |c| just(c).padded();

    let unary = op('-').repeated().foldr(atom, |_, rhs| {
      let span = rhs.1;
      (Ast::UnaryOp(UnaryOp::Neg, Box::new(rhs)), span)
    });

    let product = unary.clone().foldl(
      choice((
        op('*').to(BinaryOp::Mul),
        op('/').to(BinaryOp::Div),
        op('%').to(BinaryOp::Mod),
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

pub(crate) fn parse<'a>(
  input: &'a str,
) -> Result<Spanned<Ast<'a>>, Vec<Error>> {
  let result = parser().parse(input);

  match result.into_output_errors() {
    (Some(ast), errors) if errors.is_empty() => Ok(ast),
    (_, errors) => Err(
      errors
        .into_iter()
        .map(|error| {
          Error::new(
            error.span().to_owned(),
            format!("error: {}", error.to_string()),
          )
        })
        .collect(),
    ),
  }
}
