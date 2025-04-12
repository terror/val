use super::*;

pub(crate) type BuiltinFunction<'src> =
  fn(Vec<Value<'src>>, Span) -> Result<Value<'src>, Error>;

pub(crate) struct Environment<'src> {
  functions: HashMap<&'src str, BuiltinFunction<'src>>,
  variables: HashMap<&'src str, Value<'src>>,
}

impl<'src> Environment<'src> {
  pub(crate) fn new() -> Self {
    let mut env = Self {
      functions: HashMap::new(),
      variables: HashMap::new(),
    };

    env.add_builtin_function("sin", |args, span| {
      if args.len() != 1 {
        return Err(Error::new(
          span,
          format!("Function 'sin' expects 1 argument, got {}", args.len()),
        ));
      }

      match &args[0] {
        Value::Num(n) => Ok(Value::Num(n.sin())),
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
        Value::Num(n) => Ok(Value::Num(n.cos())),
        _ => Err(Error::new(span, format!("'{}' is not a number", args[0]))),
      }
    });

    env.add_variable("e", Value::Num(std::f64::consts::E));
    env.add_variable("pi", Value::Num(std::f64::consts::PI));

    env
  }

  pub(crate) fn add_builtin_function(
    &mut self,
    name: &'src str,
    func: BuiltinFunction<'src>,
  ) {
    self.functions.insert(name, func);
  }

  pub(crate) fn add_variable(&mut self, name: &'src str, value: Value<'src>) {
    self.variables.insert(name, value);
  }

  pub(crate) fn get_variable(&self, name: &str) -> Option<&Value<'src>> {
    self.variables.get(name)
  }

  pub(crate) fn get_function(
    &self,
    name: &str,
  ) -> Option<&BuiltinFunction<'src>> {
    self.functions.get(name)
  }

  pub(crate) fn call_function(
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
