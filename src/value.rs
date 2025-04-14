use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
      Value::Boolean(boolean) => write!(f, "{boolean}"),
      Value::Function(name) => write!(f, "<function: {name}>"),
      Value::List(list) => write!(
        f,
        "[{}]",
        list
          .iter()
          .map(|item| item.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Value::Null => write!(f, "null"),
      Value::Number(number) => write!(f, "{number}"),
      Value::String(string) => write!(f, "\'{string}\'"),
    }
  }
}

impl Value<'_> {
  pub fn boolean(&self, span: Span) -> Result<bool, Error> {
    if let Value::Boolean(x) = self {
      Ok(*x)
    } else {
      Err(Error {
        span,
        message: format!("'{}' is not a boolean", self),
      })
    }
  }

  pub fn list(&self, span: Span) -> Result<&[Value], Error> {
    if let Value::List(x) = self {
      Ok(x)
    } else {
      Err(Error {
        span,
        message: format!("'{}' is not a list", self),
      })
    }
  }

  pub fn number(&self, span: Span) -> Result<f64, Error> {
    if let Value::Number(x) = self {
      Ok(*x)
    } else {
      Err(Error {
        span,
        message: format!("'{}' is not a number", self),
      })
    }
  }

  pub fn string(&self, span: Span) -> Result<&str, Error> {
    if let Value::String(x) = self {
      Ok(*x)
    } else {
      Err(Error {
        span,
        message: format!("'{}' is not a string", self),
      })
    }
  }
}
