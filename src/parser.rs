use super::*;

pub(crate) fn parser<'a>()
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
          .delimited_by(just('('), just(')'))
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

    let atom = number.or(call).or(identifier);

    let op = |c| just(c).padded();

    let unary = op('-').repeated().foldr(atom, |_, rhs| {
      let span = rhs.1;
      (Ast::UnaryOp(UnaryOp::Neg, Box::new(rhs)), span)
    });

    unary
  })
}
