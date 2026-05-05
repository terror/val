#[derive(Default)]
pub(crate) struct Context {
  function_depth: usize,
  loop_depth: usize,
}

impl Context {
  pub(crate) fn enter_function(&mut self) {
    self.function_depth += 1;
  }

  pub(crate) fn exit_function(&mut self) {
    self.function_depth -= 1;
  }

  pub(crate) fn enter_loop(&mut self) {
    self.loop_depth += 1;
  }

  pub(crate) fn exit_loop(&mut self) {
    self.loop_depth -= 1;
  }

  pub(crate) fn inside_function(&self) -> bool {
    self.function_depth > 0
  }

  pub(crate) fn inside_loop(&self) -> bool {
    self.loop_depth > 0
  }
}
