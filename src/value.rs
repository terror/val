use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
  Boolean(bool),
  Function(Function),
  List(Vec<Self>),
  Null,
  Number(Number),
  String(String),
}

impl Value {
  pub(crate) fn boolean(&self, span: Span) -> Result<bool> {
    if let Value::Boolean(x) = self {
      Ok(*x)
    } else {
      Err(Error::new(span, format!("'{self}' is not a boolean")))
    }
  }

  #[must_use]
  pub fn display(&self, config: Config) -> String {
    match self {
      Value::Boolean(boolean) => boolean.to_string(),
      Value::Function(function) => format!("<function: {}>", function.name()),
      Value::List(list) => format!(
        "[{}]",
        list
          .iter()
          .map(|item| match item {
            Value::String(string) => format!("\'{string}\'"),
            _ => item.display(config),
          })
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Value::Null => "null".into(),
      Value::Number(number) => number.display(config),
      Value::String(string) => string.clone(),
    }
  }

  pub(crate) fn into_function(self, span: Span) -> Result<Function> {
    match self {
      Value::Function(x) => Ok(x),
      value => Err(Error::new(span, format!("'{value}' is not a function"))),
    }
  }

  pub(crate) fn into_list(self, span: Span) -> Result<Vec<Value>> {
    match self {
      Value::List(x) => Ok(x),
      value => Err(Error::new(span, format!("'{value}' is not a list"))),
    }
  }

  pub(crate) fn list(&self, span: Span) -> Result<&[Value]> {
    if let Value::List(x) = self {
      Ok(x)
    } else {
      Err(Error::new(span, format!("'{self}' is not a list")))
    }
  }

  pub(crate) fn number(&self, span: Span) -> Result<&Number> {
    if let Value::Number(x) = self {
      Ok(x)
    } else {
      Err(Error::new(span, format!("'{self}' is not a number")))
    }
  }

  pub(crate) fn string(&self, span: Span) -> Result<&str> {
    if let Value::String(x) = self {
      Ok(x)
    } else {
      Err(Error::new(span, format!("'{self}' is not a string")))
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

impl Display for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.display(Config::default()))
  }
}

impl Finalize for Value {}

unsafe impl Trace for Value {
  gc::custom_trace!(this, {
    match this {
      Self::Function(function) => unsafe { mark(function) },
      Self::List(list) => unsafe { mark(list) },
      Self::Boolean(_) | Self::Null | Self::Number(_) | Self::String(_) => {}
    }
  });
}
