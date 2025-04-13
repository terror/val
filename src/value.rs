use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Value<'src> {
  Boolean(bool),
  Function(&'src str),
  List(Vec<Self>),
  Null,
  Number(f64),
  String(&'src str),
}

impl Display for Value<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Boolean(b) => write!(f, "{b}"),
      Value::Function(s) => write!(f, "<function: {s}>"),
      Value::List(l) => write!(f, "{:?}", l),
      Value::Null => write!(f, "null"),
      Value::Number(n) => write!(f, "{n}"),
      Value::String(s) => write!(f, "{s}"),
    }
  }
}

impl Value<'_> {
  pub fn boolean(self, span: Span) -> Result<bool, Error> {
    if let Value::Boolean(x) = self {
      Ok(x)
    } else {
      Err(Error {
        span,
        message: format!("'{}' is not a boolean", self),
      })
    }
  }

  pub fn number(self, span: Span) -> Result<f64, Error> {
    if let Value::Number(x) = self {
      Ok(x)
    } else {
      Err(Error {
        span,
        message: format!("'{}' is not a number", self),
      })
    }
  }
}
