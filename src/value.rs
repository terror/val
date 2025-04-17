use super::*;

#[derive(Clone, Debug)]
pub enum Value<'src> {
  Boolean(bool),
  Function(
    &'src str,
    Vec<&'src str>,
    Vec<Spanned<Statement<'src>>>,
    Environment<'src>,
  ),
  List(Vec<Self>),
  Null,
  Number(BigFloat),
  String(&'src str),
}

impl Display for Value<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Boolean(boolean) => write!(f, "{boolean}"),
      Value::Function(name, _, _, _) => write!(f, "<function: {name}>"),
      Value::List(list) => write!(
        f,
        "[{}]",
        list
          .iter()
          .map(|item| match item {
            Value::String(string) => format!("\'{string}\'"),
            _ => item.to_string(),
          })
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Value::Null => write!(f, "null"),
      Value::Number(number) => write!(f, "{number}",),
      Value::String(string) => write!(f, "{string}"),
    }
  }
}

impl PartialEq for Value<'_> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Value::Boolean(a), Value::Boolean(b)) => a == b,
      (Value::Function(a_name, _, _, _), Value::Function(b_name, _, _, _)) => {
        a_name == b_name
      }
      (Value::List(a), Value::List(b)) => {
        a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| a == b)
      }
      (Value::Null, Value::Null) => true,
      (Value::Number(a), Value::Number(b)) => a == b,
      (Value::String(a), Value::String(b)) => a == b,
      _ => false,
    }
  }
}

impl<'a> Value<'a> {
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

  pub fn list(&self, span: Span) -> Result<Vec<Value<'a>>, Error> {
    if let Value::List(x) = self {
      Ok(x.clone())
    } else {
      Err(Error {
        span,
        message: format!("'{}' is not a list", self),
      })
    }
  }

  pub fn number(&self, span: Span) -> Result<BigFloat, Error> {
    if let Value::Number(x) = self {
      Ok(x.clone())
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

  pub fn type_name(&self) -> &'static str {
    match self {
      Value::Boolean(_) => "boolean",
      Value::Function(_, _, _, _) => "function",
      Value::List(_) => "list",
      Value::Null => "null",
      Value::Number(_) => "number",
      Value::String(_) => "string",
    }
  }
}
