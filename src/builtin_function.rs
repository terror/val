use super::*;

#[derive(Clone, Debug)]
pub struct BuiltinFunction {
  pub arity: BuiltinArity,
  pub function: fn(&BuiltinFunctionPayload) -> Result<Value>,
  pub name: &'static str,
}

impl BuiltinFunction {
  pub(crate) fn call(
    &self,
    arguments: Vec<Value>,
    config: Config,
    span: Span,
  ) -> Result<Value> {
    (self.function)(&BuiltinFunctionPayload {
      arguments,
      config,
      name: self.name,
      span,
    })
  }

  pub(crate) fn check_arity(&self, len: usize, span: Span) -> Result<()> {
    self.arity.check(self.name, len, span)
  }
}
