use super::*;

pub type BuiltinFunction<'src> =
  fn(Vec<Value<'src>>, Span) -> Result<Value<'src>, Error>;

#[derive(Default)]
pub struct Environment<'src> {
  functions: HashMap<&'src str, BuiltinFunction<'src>>,
  variables: HashMap<&'src str, Value<'src>>,
}

impl<'src> Environment<'src> {
  pub fn new() -> Self {
    let mut env = Self::default();

    env.add_builtin_function("sin", |arguments, span| {
      if arguments.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function `sin` expects 1 argument, got {}", arguments.len()),
        ));
      }

      Ok(Value::Number(arguments[0].number(span)?.sin()))
    });

    env.add_builtin_function("cos", |arguments, span| {
      if arguments.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function `cos` expects 1 argument, got {}", arguments.len()),
        ));
      }

      Ok(Value::Number(arguments[0].number(span)?.cos()))
    });

    env.add_builtin_function("arc", |arguments, span| {
      if arguments.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function `arc` expects 1 argument, got {}", arguments.len()),
        ));
      }

      Ok(Value::Number(arguments[0].number(span)?.atan()))
    });

    env.add_builtin_function("ln", |arguments, span| {
      if arguments.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function 'log' expects 1 argument, got {}", arguments.len()),
        ));
      }

      let number = arguments[0].number(span)?;

      if number <= 0.0 {
        return Err(Error::new(
          span,
          "Cannot take logarithm of zero or negative number",
        ));
      }

      Ok(Value::Number(number.ln()))
    });

    env.add_builtin_function("e", |arguments, span| {
      if arguments.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function `e` expects 1 argument, got {}", arguments.len()),
        ));
      }

      Ok(Value::Number(arguments[0].number(span)?.exp()))
    });

    env.add_builtin_function("sqrt", |arguments, span| {
      if arguments.len() != 1 {
        return Err(Error::new(
          span,
          format!(
            "Function `sqrt` expects 1 argument, got {}",
            arguments.len()
          ),
        ));
      }

      let number = arguments[0].number(span)?;

      if number < 0.0 {
        return Err(Error::new(
          span,
          "Cannot take square root of negative number",
        ));
      }

      Ok(Value::Number(number.sqrt()))
    });

    env.add_builtin_function("len", |arguments, span| {
      if arguments.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function `len` expects 1 argument, got {}", arguments.len()),
        ));
      }

      Ok(Value::Number(arguments[0].string(span)?.len() as f64))
    });

    env.add_builtin_function("print", |arguments, span| {
      if arguments.len() != 1 {
        return Err(Error::new(
          span,
          format!(
            "Function `print` expects 1 argument, got {}",
            arguments.len()
          ),
        ));
      }

      println!("{}", arguments[0]);

      Ok(Value::Null)
    });

    env.add_builtin_function("quit", |arguments, span| {
      if arguments.is_empty() {
        process::exit(0);
      }

      if arguments.len() != 1 {
        return Err(Error::new(
          span,
          format!(
            "Function `quit` expects 0 or 1 arguments, got {}",
            arguments.len()
          ),
        ));
      }

      process::exit(arguments[0].number(span)? as i32);
    });

    env.add_variable("e", Value::Number(std::f64::consts::E));
    env.add_variable("pi", Value::Number(std::f64::consts::PI));

    env
  }

  pub fn add_builtin_function(
    &mut self,
    name: &'src str,
    function: BuiltinFunction<'src>,
  ) {
    self.functions.insert(name, function);
  }

  pub fn add_variable(&mut self, name: &'src str, value: Value<'src>) {
    self.variables.insert(name, value);
  }

  pub fn get_variable(&self, name: &str) -> Option<&Value<'src>> {
    self.variables.get(name)
  }

  pub fn get_function(&self, name: &str) -> Option<&BuiltinFunction<'src>> {
    self.functions.get(name)
  }

  pub fn call_function(
    &self,
    name: &str,
    arguments: Vec<Value<'src>>,
    span: Span,
  ) -> Result<Value<'src>, Error> {
    match self.get_function(name) {
      Some(func) => func(arguments, span),
      None => Err(Error::new(
        span,
        format!("Function `{}` is not implemented", name),
      )),
    }
  }
}
