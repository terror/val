use super::*;

pub(crate) type FallibleBuiltinFunction =
  for<'src> fn(&BuiltinFunctionPayload<'src>) -> Result<Value<'src>, Error>;

pub(crate) type InfallibleBuiltinFunction =
  for<'src> fn(&BuiltinFunctionPayload<'src>) -> Value<'src>;

#[derive(Clone, Copy, Debug)]
pub enum BuiltinFunction {
  Fallible(FallibleBuiltinFunction),
  Infallible(InfallibleBuiltinFunction),
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
