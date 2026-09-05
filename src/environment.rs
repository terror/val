use super::*;

#[derive(Clone, Default)]
pub struct Environment {
  pub(crate) config: Config,
  pub(crate) frame: Rc<RefCell<Frame>>,
}

impl Environment {
  pub fn add_function(&self, name: &str, function: Function) {
    let mut frame = self.frame.borrow_mut();

    if let Some(symbol) = frame.symbols.get_mut(name) {
      symbol.function = Some(function);
    } else {
      frame.symbols.insert(
        name.to_owned(),
        Symbol {
          function: Some(function),
          value: None,
        },
      );
    }
  }

  pub fn add_symbol(&self, name: &str, value: Value) {
    let mut frame = self.frame.borrow_mut();

    if let Some(symbol) = frame.symbols.get_mut(name) {
      symbol.value = Some(value);
    } else {
      frame.symbols.insert(
        name.to_owned(),
        Symbol {
          function: None,
          value: Some(value),
        },
      );
    }
  }

  fn assign_existing_symbol(
    &self,
    name: &str,
    value: Value,
  ) -> std::result::Result<(), Value> {
    let parent = {
      let mut frame = self.frame.borrow_mut();

      match frame.symbols.get_mut(name) {
        Some(symbol) if symbol.value.is_some() => {
          symbol.value = Some(value);
          return Ok(());
        }
        _ => frame.parent.clone(),
      }
    };

    match parent {
      Some(parent) => parent.assign_existing_symbol(name, value),
      None => Err(value),
    }
  }

  pub(crate) fn assign_symbol(&self, name: &str, value: Value) {
    if let Err(value) = self.assign_existing_symbol(name, value) {
      self.add_symbol(name, value);
    }
  }

  pub(crate) fn function(
    &self,
    name: &str,
    span: Span,
  ) -> Result<Function, Error> {
    match self.resolve_function(name) {
      Some(function) => Ok(function),
      None if self.resolve_symbol(name).is_some() => {
        Err(Error::new(span, format!("`{name}` is not a function")))
      }
      None => Err(Error::new(
        span,
        format!("Function `{name}` is not defined"),
      )),
    }
  }

  fn local_function(&self, name: &str) -> Option<Function> {
    let frame = self.frame.borrow();

    let symbol = frame.symbols.get(name)?;

    symbol.function.clone().or_else(|| match &symbol.value {
      Some(Value::Function(function)) => Some(function.clone()),
      _ => None,
    })
  }

  fn local_symbol(&self, name: &str) -> Option<Value> {
    let frame = self.frame.borrow();

    let symbol = frame.symbols.get(name)?;

    symbol
      .value
      .clone()
      .or_else(|| symbol.function.clone().map(Value::Function))
  }

  #[must_use]
  pub fn new(config: Config) -> Self {
    let environment = Self {
      config,
      frame: Rc::new(RefCell::new(Frame::default())),
    };

    for builtin in inventory::iter::<&dyn Builtin> {
      for name in once(builtin.name()).chain(builtin.aliases().iter().copied())
      {
        match builtin.value(config) {
          Value::Function(Function::Builtin {
            arity, function, ..
          }) => {
            environment.add_function(
              name,
              Function::Builtin {
                arity,
                function,
                name,
              },
            );
          }
          Value::Function(function) => {
            environment.add_function(name, function);
          }
          value => {
            environment.add_symbol(name, value);
          }
        }
      }
    }

    environment
  }

  fn resolve_function(&self, name: &str) -> Option<Function> {
    self
      .local_function(name)
      .or_else(|| self.frame.borrow().parent.clone()?.resolve_function(name))
  }

  pub(crate) fn resolve_symbol(&self, name: &str) -> Option<Value> {
    self
      .local_symbol(name)
      .or_else(|| self.frame.borrow().parent.clone()?.resolve_symbol(name))
  }

  pub(crate) fn with_parent(parent: Environment) -> Self {
    Self {
      config: parent.config,
      frame: Rc::new(RefCell::new(Frame {
        parent: Some(parent),
        symbols: HashMap::new(),
      })),
    }
  }
}

impl fmt::Debug for Environment {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("Environment")
      .field("config", &self.config)
      .finish_non_exhaustive()
  }
}
