use super::*;

#[derive(Clone, Debug)]
pub enum Value<'src> {
  Boolean(bool),
  Function(Function<'src>),
  List(Vec<Self>),
  Null,
  Number(Number),
  String(&'src str),
}

impl Display for Value<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Boolean(boolean) => write!(f, "{boolean}"),
      Value::Function(function) => write!(f, "<function: {}>", function.name()),
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
      Value::Number(number) => write!(f, "{number}"),
      Value::String(string) => write!(f, "{string}"),
    }
  }
}

impl PartialEq for Value<'_> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Value::Boolean(a), Value::Boolean(b)) => a == b,
      (Value::Function(a), Value::Function(b)) => a.name() == b.name(),
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
  pub(crate) fn boolean(&self, span: Span) -> Result<bool, Error> {
    if let Value::Boolean(x) = self {
      Ok(*x)
    } else {
      Err(Error {
        span,
        message: format!("'{self}' is not a boolean"),
      })
    }
  }

  pub(crate) fn list(&self, span: Span) -> Result<Vec<Value<'a>>, Error> {
    if let Value::List(x) = self {
      Ok(x.clone())
    } else {
      Err(Error {
        span,
        message: format!("'{self}' is not a list"),
      })
    }
  }

  pub(crate) fn number(&self, span: Span) -> Result<Number, Error> {
    if let Value::Number(x) = self {
      Ok(x.clone())
    } else {
      Err(Error {
        span,
        message: format!("'{self}' is not a number"),
      })
    }
  }

  pub(crate) fn string(&self, span: Span) -> Result<&str, Error> {
    if let Value::String(x) = self {
      Ok(*x)
    } else {
      Err(Error {
        span,
        message: format!("'{self}' is not a string"),
      })
    }
  }

  pub(crate) fn type_name(&self) -> &'static str {
    match self {
      Value::Boolean(_) => "boolean",
      Value::Function(_) => "function",
      Value::List(_) => "list",
      Value::Null => "null",
      Value::Number(_) => "number",
      Value::String(_) => "string",
    }
  }
}
