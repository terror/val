use super::*;

pub(crate) fn parser<'a>()
-> impl Parser<'a, &'a str, Spanned<Ast<'a>>, extra::Err<Rich<'a, char>>> {
  let ident = text::ident().padded();

  let expr = recursive(|_| {
    let number = text::int(10)
      .map(|s: &str| Ast::Number(s.parse().unwrap()))
      .map_with(|ast, e| (ast, e.span()));

    let atom = number.or(
      ident
        .map(Ast::Identifier)
        .map_with(|ast, e| (ast, e.span())),
    );

    let op = |c| just(c).padded();

    let unary = op('-').repeated().foldr(atom, |_, rhs| {
      let span = rhs.1;
      (Ast::UnaryOp(UnaryOp::Neg, Box::new(rhs)), span)
    });

    unary
  });

  expr
}
