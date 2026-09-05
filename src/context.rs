#[derive(Default)]
pub(crate) struct Context {
  inside_function: bool,
  loop_depth: usize,
}

impl Context {
  pub(crate) fn enter_loop(&mut self) {
    self.loop_depth += 1;
  }

  pub(crate) fn exit_loop(&mut self) {
    self.loop_depth -= 1;
  }

  pub(crate) fn for_function() -> Self {
    Self {
      inside_function: true,
      ..Self::default()
    }
  }

  pub(crate) fn inside_function(&self) -> bool {
    self.inside_function
  }

  pub(crate) fn inside_loop(&self) -> bool {
    self.loop_depth > 0
  }
}
