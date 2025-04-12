use super::*;

#[derive(Debug, Clone)]
pub enum UnaryOp {
  Neg,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum BinaryOp {
  Add,
  Div,
  Mod,
  Mul,
  Pow,
  Sub,
}

#[derive(Debug)]
#[allow(unused)]
pub enum Ast<'a> {
  BinaryOp(BinaryOp, Box<Spanned<Self>>, Box<Spanned<Self>>),
  Call(&'a str, Vec<Spanned<Self>>),
  Identifier(&'a str),
  Number(f64),
  UnaryOp(UnaryOp, Box<Spanned<Self>>),
}
