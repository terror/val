use super::*;

#[derive(Debug, Clone)]
pub enum UnaryOp {
  Negate,
  Not,
}

impl Display for UnaryOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      UnaryOp::Negate => write!(f, "-"),
      UnaryOp::Not => write!(f, "!"),
    }
  }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum BinaryOp {
  Add,
  Divide,
  Equal,
  GreaterThan,
  GreaterThanEqual,
  LessThan,
  LessThanEqual,
  Modulo,
  Multiply,
  NotEqual,
  Power,
  Subtract,
}

impl Display for BinaryOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      BinaryOp::Add => write!(f, "+"),
      BinaryOp::Divide => write!(f, "/"),
      BinaryOp::Equal => write!(f, "=="),
      BinaryOp::GreaterThanEqual => write!(f, ">="),
      BinaryOp::GreaterThan => write!(f, ">"),
      BinaryOp::LessThanEqual => write!(f, "<="),
      BinaryOp::LessThan => write!(f, "<"),
      BinaryOp::Modulo => write!(f, "%"),
      BinaryOp::Multiply => write!(f, "*"),
      BinaryOp::NotEqual => write!(f, "!="),
      BinaryOp::Power => write!(f, "^"),
      BinaryOp::Subtract => write!(f, "-"),
    }
  }
}

#[derive(Debug)]
#[allow(unused)]
pub enum Ast<'a> {
  BinaryOp(BinaryOp, Box<Spanned<Self>>, Box<Spanned<Self>>),
  Boolean(bool),
  FunctionCall(&'a str, Vec<Spanned<Self>>),
  Identifier(&'a str),
  Number(f64),
  String(&'a str),
  UnaryOp(UnaryOp, Box<Spanned<Self>>),
}

impl Display for Ast<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Ast::BinaryOp(op, lhs, rhs) => write!(f, "({} {} {})", op, lhs.0, rhs.0),
      Ast::Boolean(b) => write!(f, "{}", b),
      Ast::FunctionCall(name, arguments) => write!(
        f,
        "{}({})",
        name,
        arguments
          .iter()
          .map(|a| a.0.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Ast::Identifier(id) => write!(f, "{}", id),
      Ast::Number(n) => write!(f, "{}", n),
      Ast::String(s) => write!(f, "\"{}\"", s),
      Ast::UnaryOp(op, expr) => write!(f, "{}{}", op, expr.0),
    }
  }
}

impl Ast<'_> {
  pub fn kind(&self) -> String {
    String::from(match self {
      Ast::BinaryOp(_, _, _) => "binary_op",
      Ast::Boolean(_) => "boolean",
      Ast::FunctionCall(_, _) => "function_call",
      Ast::Identifier(_) => "identifier",
      Ast::Number(_) => "number",
      Ast::String(_) => "string",
      Ast::UnaryOp(_, _) => "unary_op",
    })
  }
}
