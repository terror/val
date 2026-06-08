use super::*;

#[derive(Clone, Copy, Debug)]
pub enum BuiltinFunction {
  Fallible(
    for<'src> fn(&BuiltinFunctionPayload<'src>) -> Result<Value<'src>, Error>,
  ),
  Infallible(for<'src> fn(&BuiltinFunctionPayload<'src>) -> Value<'src>),
}

impl BuiltinFunction {
  pub(crate) fn call<'src>(
    self,
    payload: &BuiltinFunctionPayload<'src>,
  ) -> Result<Value<'src>, Error> {
    match self {
      Self::Fallible(function) => function(payload),
      Self::Infallible(function) => Ok(function(payload)),
    }
  }
}
