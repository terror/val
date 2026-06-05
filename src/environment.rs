use super::*;

#[derive(Clone, Default)]
pub struct Environment<'src> {
  pub(crate) config: Config,
  pub(crate) frame: Rc<RefCell<Frame<'src>>>,
}

impl<'src> Environment<'src> {
  pub fn add_function(&self, name: &'src str, function: Function<'src>) {
    self
      .frame
      .borrow_mut()
      .symbols
      .entry(name)
      .or_default()
      .function = Some(function);
  }

  pub fn add_symbol(&self, name: &'src str, value: Value<'src>) {
    self
      .frame
      .borrow_mut()
      .symbols
      .entry(name)
      .or_default()
      .value = Some(value);
  }

  fn assign_existing_symbol(
    &self,
    name: &'src str,
    value: Value<'src>,
  ) -> bool {
    let parent = {
      let mut frame = self.frame.borrow_mut();

      match frame.symbols.get_mut(name) {
        Some(symbol) if symbol.value.is_some() => {
          symbol.value = Some(value);
          return true;
        }
        _ => frame.parent.clone(),
      }
    };

    parent.is_some_and(|parent| parent.assign_existing_symbol(name, value))
  }

  pub(crate) fn assign_symbol(&self, name: &'src str, value: Value<'src>) {
    if !self.assign_existing_symbol(name, value.clone()) {
      self.add_symbol(name, value);
    }
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

  fn local_function(&self, name: &str) -> Option<Function<'src>> {
    let frame = self.frame.borrow();

    let symbol = frame.symbols.get(name)?;

    symbol.function.clone().or_else(|| match &symbol.value {
      Some(Value::Function(function)) => Some(function.clone()),
      _ => None,
    })
  }

  fn local_symbol(&self, name: &str) -> Option<Value<'src>> {
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

    for builtin in BUILTINS {
      match builtin {
        Builtin::Constant { value, .. } => {
          environment.add_symbol(builtin.name(), Value::Number(value(config)));
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

  fn resolve_function(&self, name: &str) -> Option<Function<'src>> {
    self
      .local_function(name)
      .or_else(|| self.frame.borrow().parent.clone()?.resolve_function(name))
  }

  pub(crate) fn resolve_symbol(&self, name: &str) -> Option<Value<'src>> {
    self
      .local_symbol(name)
      .or_else(|| self.frame.borrow().parent.clone()?.resolve_symbol(name))
  }

  pub(crate) fn with_parent(parent: Environment<'src>) -> Self {
    Self {
      config: parent.config,
      frame: Rc::new(RefCell::new(Frame {
        parent: Some(parent),
        symbols: HashMap::new(),
      })),
    }
  }
}

impl fmt::Debug for Environment<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("Environment")
      .field("config", &self.config)
      .finish_non_exhaustive()
  }
}
