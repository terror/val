use super::*;

#[derive(Clone, Debug, Default)]
pub(crate) struct Symbol<'src> {
  pub(crate) function: Option<Function<'src>>,
  pub(crate) value: Option<Value<'src>>,
}
