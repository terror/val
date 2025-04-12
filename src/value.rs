use super::*;

#[derive(Clone, Debug, PartialEq)]
#[allow(unused)]
pub enum Value<'src> {
  Null,
  Bool(bool),
  Num(f64),
  Str(&'src str),
  List(Vec<Self>),
  Func(&'src str),
}

impl Display for Value<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Null => write!(f, "null"),
      Value::Bool(b) => write!(f, "{}", b),
      Value::Num(n) => write!(f, "{}", n),
      Value::Str(s) => write!(f, "{}", s),
      Value::List(l) => write!(f, "{:?}", l),
      Value::Func(s) => write!(f, "<function: {}>", s),
    }
  }
}

impl Value<'_> {
  pub fn num(self, span: Span) -> Result<f64, Error> {
    if let Value::Num(x) = self {
      Ok(x)
    } else {
      Err(Error {
        span,
        message: format!("'{}' is not a number", self),
      })
    }
  }
}
