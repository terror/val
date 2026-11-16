use super::*;

#[derive(Clone)]
pub struct Environment {
  pub(crate) config: Config,
  pub(crate) frame: Gc<GcCell<Frame>>,
  pub(crate) input: Rc<RefCell<dyn BufRead>>,
  pub(crate) output: Rc<RefCell<dyn Write>>,
}

impl Environment {
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
  pub fn empty(config: Config) -> Self {
    Self {
      config,
      frame: Gc::new(GcCell::new(Frame::default())),
      input: Rc::new(RefCell::new(BufReader::new(io::stdin()))),
      output: Rc::new(RefCell::new(io::stdout())),
    }
  }

  #[must_use]
  pub fn new(config: Config) -> Self {
    let environment = Self::empty(config);

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

  #[must_use]
  pub fn with_io(
    self,
    input: impl BufRead + 'static,
    output: impl Write + 'static,
  ) -> Self {
    Self {
      input: Rc::new(RefCell::new(input)),
      output: Rc::new(RefCell::new(output)),
      ..self
    }
  }

  pub(crate) fn with_parent(parent: Environment) -> Self {
    Self {
      config: parent.config,
      input: parent.input.clone(),
      output: parent.output.clone(),
      frame: Gc::new(GcCell::new(Frame {
        parent: Some(parent),
        symbols: HashMap::new(),
      })),
    }
  }
}

impl Debug for Environment {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("Environment")
      .field("config", &self.config)
      .finish_non_exhaustive()
  }
}

impl Default for Environment {
  fn default() -> Self {
    Self::new(Config::default())
  }
}

impl Finalize for Environment {}

unsafe impl Trace for Environment {
  gc::custom_trace!(this, {
    let Self {
      config: _,
      frame,
      input: _,
      output: _,
    } = this;

    unsafe { mark(frame) };
  });
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Clone, Default)]
  struct Output(Rc<RefCell<Vec<u8>>>);

  impl Write for Output {
    fn flush(&mut self) -> io::Result<()> {
      Ok(())
    }

    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
      self.0.borrow_mut().write(buffer)
    }
  }

  #[test]
  fn with_io() {
    let output = Output::default();

    let environment = Environment::new(Config::default())
      .with_io(io::Cursor::new("bar\n"), output.clone());

    let ast = parse("fn foo(x) { println(x) } print(input('foo')); foo('baz')")
      .unwrap();

    Evaluator::from(environment).evaluate(&ast).unwrap();

    assert_eq!(*output.0.borrow(), b"foobarbaz\n");
  }
}
