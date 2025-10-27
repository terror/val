use super::*;

#[derive(Clone, Debug)]
pub enum Value<'src> {
  Boolean(bool),
  BuiltinFunction(&'src str, BuiltinFunction<'src>),
  Function(
    &'src str,
    Vec<&'src str>,
    Vec<Spanned<Statement<'src>>>,
    Environment<'src>,
  ),
  List(Vec<Self>),
  Null,
  Number(Float),
  String(&'src str),
}

impl Display for Value<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Boolean(boolean) => write!(f, "{boolean}"),
      Value::BuiltinFunction(name, _) => write!(f, "<function: {name}>"),
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
      Value::Number(number) => write!(f, "{}", number.display()),
      Value::String(string) => write!(f, "{string}"),
    }
  }
}

impl PartialEq for Value<'_> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Value::Boolean(a), Value::Boolean(b)) => a == b,
      (
        Value::BuiltinFunction(a_name, _),
        Value::BuiltinFunction(b_name, _),
      ) => a_name == b_name,
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
  pub fn format_with_config(&self, config: &Config) -> String {
    match self {
      Value::Boolean(boolean) => boolean.to_string(),
      Value::BuiltinFunction(name, _) | Value::Function(name, _, _, _) => {
        format!("<function: {name}>")
      }
      Value::List(list) => {
        let items = list
          .iter()
          .map(|item| match item {
            Value::String(string) => format!("\'{string}\'"),
            _ => item.format_with_config(config),
          })
          .collect::<Vec<_>>()
          .join(", ");

        format!("[{items}]")
      }
      Value::Null => "null".into(),
      Value::Number(number) => {
        number.display_with_digits(config.digits, config.rounding_mode)
      }
      Value::String(string) => (*string).into(),
    }
  }

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

  pub fn number(&self, span: Span) -> Result<Float, Error> {
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
      Value::BuiltinFunction(_, _) => "function",
      Value::Function(_, _, _, _) => "function",
      Value::List(_) => "list",
      Value::Null => "null",
      Value::Number(_) => "number",
      Value::String(_) => "string",
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn float_from_str(s: &str, precision: usize) -> Float {
    with_consts(|consts| {
      Float::parse(
        s,
        Radix::Dec,
        precision,
        astro_float::RoundingMode::FromZero,
        consts,
      )
    })
  }

  #[test]
  fn number_format_respects_digits() {
    let config = Config {
      precision: 256,
      rounding_mode: astro_float::RoundingMode::ToEven,
      digits: Some(2),
    };

    let value = Value::Number(float_from_str("3.4567", config.precision));

    assert_eq!(value.format_with_config(&config), "3.46");
  }

  #[test]
  fn list_format_propagates_digits() {
    let config = Config {
      precision: 256,
      rounding_mode: astro_float::RoundingMode::ToZero,
      digits: Some(3),
    };

    let items = vec![
      Value::Number(float_from_str("1.234567", config.precision)),
      Value::String("hello"),
    ];

    let value = Value::List(items);

    assert_eq!(value.format_with_config(&config), "[1.234, 'hello']");
  }
}
