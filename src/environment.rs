use super::*;

#[derive(Clone, Debug, Default)]
pub struct Environment<'src> {
  pub config: Config,
  pub parent: Option<Box<Environment<'src>>>,
  symbols: HashMap<&'src str, Symbol<'src>>,
}

impl<'src> Environment<'src> {
  #[must_use]
  pub fn new(config: Config) -> Self {
    let mut environment = Self {
      config,
      parent: None,
      symbols: HashMap::new(),
    };

    for builtin in BUILTINS {
      match builtin {
        Builtin::Constant { value, .. } => {
          environment.add_symbol(builtin.name(), value(&config));
        }
        Builtin::Function { function, .. } => {
          environment.add_function(
            builtin.name(),
            Function::Builtin {
              function: *function,
              name: builtin.name(),
            },
          );
        }
      }
    }

    environment
  }

  pub fn add_symbol(&mut self, name: &'src str, value: Value<'src>) {
    self.symbols.entry(name).or_default().value = Some(value);
  }

  pub fn add_function(&mut self, name: &'src str, function: Function<'src>) {
    self.symbols.entry(name).or_default().function = Some(function);
  }

  pub(crate) fn call_function(
    &self,
    name: &str,
    arguments: Vec<Value<'src>>,
    span: Span,
  ) -> Result<Value<'src>, Error> {
    match self.resolve_function(name) {
      Some(function) => function.call(arguments, self.config, span),
      None if self.resolve_symbol(name).is_some() => {
        Err(Error::new(span, format!("`{name}` is not a function")))
      }
      None => Err(Error::new(
        span,
        format!("Function `{name}` is not defined"),
      )),
    }
  }

  fn resolve_function(&self, name: &str) -> Option<Function<'src>> {
    self
      .local_function(name)
      .or_else(|| self.parent.as_deref()?.resolve_function(name))
  }

  pub(crate) fn resolve_symbol(&self, name: &str) -> Option<Value<'src>> {
    self
      .local_symbol(name)
      .or_else(|| self.parent.as_deref()?.resolve_symbol(name))
  }

  fn local_function(&self, name: &str) -> Option<Function<'src>> {
    let symbol = self.symbols.get(name)?;

    symbol.function.clone().or_else(|| match &symbol.value {
      Some(Value::Function(function)) => Some(function.clone()),
      _ => None,
    })
  }

  fn local_symbol(&self, name: &str) -> Option<Value<'src>> {
    let symbol = self.symbols.get(name)?;

    symbol
      .value
      .clone()
      .or_else(|| symbol.function.clone().map(Value::Function))
  }

  pub(crate) fn with_parent(parent: Environment<'src>) -> Self {
    Self {
      config: parent.config,
      parent: Some(Box::new(parent)),
      symbols: HashMap::new(),
    }
  }
}
