use super::*;

#[derive(Clone, Default)]
pub struct Environment {
  pub(crate) config: Config,
  pub(crate) frame: Rc<RefCell<Frame>>,
}

impl Environment {
  pub fn add_function(&self, name: &str, function: Function) {
    self.add_symbol(name, Value::Function(function));
  }

  pub fn add_symbol(&self, name: &str, value: Value) {
    let mut frame = self.frame.borrow_mut();

    frame.symbols.insert(name.to_owned(), value);
  }

  fn assign_existing_symbol(
    &self,
    name: &str,
    value: Value,
  ) -> std::result::Result<(), Value> {
    let parent = {
      let mut frame = self.frame.borrow_mut();

      match frame.symbols.get_mut(name) {
        Some(symbol) => {
          *symbol = value;
          return Ok(());
        }
        None => frame.parent.clone(),
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

  #[must_use]
  pub fn new(config: Config) -> Self {
    let environment = Self {
      config,
      frame: Rc::new(RefCell::new(Frame::default())),
    };

    for builtin in inventory::iter::<&dyn Builtin> {
      for name in once(builtin.name()).chain(builtin.aliases().iter().copied())
      {
        let value = match builtin.value(config) {
          Value::Function(Function::Builtin(function)) => {
            Value::Function(Function::Builtin(BuiltinFunction {
              name,
              ..function
            }))
          }
          value => value,
        };

        environment.add_symbol(name, value);
      }
    }

    environment
  }

  pub(crate) fn resolve_symbol(&self, name: &str) -> Option<Value> {
    let frame = self.frame.borrow();

    frame
      .symbols
      .get(name)
      .cloned()
      .or_else(|| frame.parent.as_ref()?.resolve_symbol(name))
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
