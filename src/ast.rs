use super::*;

#[derive(Debug, Clone)]
pub enum UnaryOp {
  Neg,
}

impl Display for UnaryOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      UnaryOp::Neg => write!(f, "-"),
    }
  }
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

impl Display for BinaryOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      BinaryOp::Add => write!(f, "+"),
      BinaryOp::Div => write!(f, "/"),
      BinaryOp::Mod => write!(f, "%"),
      BinaryOp::Mul => write!(f, "*"),
      BinaryOp::Pow => write!(f, "^"),
      BinaryOp::Sub => write!(f, "-"),
    }
  }
}

#[derive(Debug)]
#[allow(unused)]
pub enum Ast<'a> {
  BinaryOp(BinaryOp, Box<Spanned<Self>>, Box<Spanned<Self>>),
  FunctionCall(&'a str, Vec<Spanned<Self>>),
  Identifier(&'a str),
  Number(f64),
  UnaryOp(UnaryOp, Box<Spanned<Self>>),
}

impl Ast<'_> {
  pub fn kind(&self) -> String {
    String::from(match self {
      Ast::BinaryOp(_, _, _) => "binary_op",
      Ast::FunctionCall(_, _) => "function_call",
      Ast::Identifier(_) => "identifier",
      Ast::Number(_) => "number",
      Ast::UnaryOp(_, _) => "unary_op",
    })
  }
}
