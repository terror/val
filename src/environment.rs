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

    env.add_builtin_function("sin", |args, span| {
      if args.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function 'sin' expects 1 argument, got {}", args.len()),
        ));
      }

      match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.sin())),
        _ => Err(Error::new(span, format!("'{}' is not a number", args[0]))),
      }
    });

    env.add_builtin_function("cos", |args, span| {
      if args.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function 'cos' expects 1 argument, got {}", args.len()),
        ));
      }

      match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.cos())),
        _ => Err(Error::new(span, format!("'{}' is not a number", args[0]))),
      }
    });

    env.add_builtin_function("arc", |args, span| {
      if args.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function 'arc' expects 1 argument, got {}", args.len()),
        ));
      }

      match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.atan())),
        _ => Err(Error::new(span, format!("'{}' is not a number", args[0]))),
      }
    });

    env.add_builtin_function("ln", |args, span| {
      if args.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function 'log' expects 1 argument, got {}", args.len()),
        ));
      }

      match &args[0] {
        Value::Number(n) => {
          if *n <= 0.0 {
            return Err(Error::new(
              span,
              "Cannot take logarithm of zero or negative number",
            ));
          }
          Ok(Value::Number(n.ln()))
        }
        _ => Err(Error::new(span, format!("'{}' is not a number", args[0]))),
      }
    });

    env.add_builtin_function("e", |args, span| {
      if args.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function 'e' expects 1 argument, got {}", args.len()),
        ));
      }

      match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.exp())),
        _ => Err(Error::new(span, format!("'{}' is not a number", args[0]))),
      }
    });

    env.add_builtin_function("sqrt", |args, span| {
      if args.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function 'sqrt' expects 1 argument, got {}", args.len()),
        ));
      }

      match &args[0] {
        Value::Number(n) => {
          if *n < 0.0 {
            return Err(Error::new(
              span,
              "Cannot take square root of negative number",
            ));
          }
          Ok(Value::Number(n.sqrt()))
        }
        _ => Err(Error::new(span, format!("'{}' is not a number", args[0]))),
      }
    });

    env.add_builtin_function("len", |args, span| {
      if args.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function 'len' expects 1 argument, got {}", args.len()),
        ));
      }

      match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err(Error::new(span, format!("'{}' is not a string", args[0]))),
      }
    });

    env.add_variable("e", Value::Number(std::f64::consts::E));
    env.add_variable("pi", Value::Number(std::f64::consts::PI));

    env
  }

  pub fn add_builtin_function(
    &mut self,
    name: &'src str,
    func: BuiltinFunction<'src>,
  ) {
    self.functions.insert(name, func);
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
    args: Vec<Value<'src>>,
    span: Span,
  ) -> Result<Value<'src>, Error> {
    match self.get_function(name) {
      Some(func) => func(args, span),
      None => Err(Error::new(
        span,
        format!("Function '{}' is not implemented", name),
      )),
    }
  }
}
