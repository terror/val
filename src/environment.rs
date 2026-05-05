use super::*;

#[derive(Clone, Debug, Default)]
pub struct Environment<'src> {
  pub config: Config,
  pub parent: Option<Box<Environment<'src>>>,
  symbols: HashMap<&'src str, Symbol<'src>>,
}

#[derive(Clone, Debug, Default)]
struct Symbol<'src> {
  function: Option<Function<'src>>,
  value: Option<Value<'src>>,
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
    if let Some(function) = self.resolve_function(name) {
      function.call(arguments, self.config, span)
    } else if self.resolve_symbol(name).is_some() {
      Err(Error::new(span, format!("`{name}` is not a function")))
    } else {
      Err(Error::new(
        span,
        format!("Function `{name}` is not defined"),
      ))
    }
  }

  fn resolve_function(&self, name: &str) -> Option<Function<'src>> {
    if let Some(symbol) = self.symbols.get(name) {
      if let Some(function) = &symbol.function {
        Some(function.clone())
      } else if let Some(Value::Function(function)) = &symbol.value {
        Some(function.clone())
      } else if let Some(parent) = &self.parent {
        parent.resolve_function(name)
      } else {
        None
      }
    } else if let Some(parent) = &self.parent {
      parent.resolve_function(name)
    } else {
      None
    }
  }

  pub(crate) fn resolve_symbol(&self, symbol: &str) -> Option<Value<'src>> {
    if let Some(symbol) = self.symbols.get(symbol) {
      if let Some(value) = &symbol.value {
        Some(value.clone())
      } else {
        symbol.function.clone().map(Value::Function)
      }
    } else if let Some(parent) = &self.parent {
      parent.resolve_symbol(symbol)
    } else {
      None
    }
  }

  pub(crate) fn with_parent(parent: Environment<'src>) -> Self {
    Self {
      config: parent.config,
      parent: Some(Box::new(parent)),
      symbols: HashMap::new(),
    }
  }
}
