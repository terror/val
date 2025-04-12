use super::*;

#[derive(Debug, Clone)]
pub(crate) enum UnaryOp {
  Neg,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub(crate) enum BinaryOp {
  Add,
  Div,
  Mod,
  Mul,
  Sub,
}

#[derive(Debug)]
#[allow(unused)]
pub(crate) enum Ast<'a> {
  BinaryOp(BinaryOp, Box<Spanned<Self>>, Box<Spanned<Self>>),
  Call(&'a str, Vec<Spanned<Self>>),
  Identifier(&'a str),
  Number(f64),
  UnaryOp(UnaryOp, Box<Spanned<Self>>),
}
