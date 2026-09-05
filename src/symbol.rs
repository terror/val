use super::*;

#[derive(Clone, Debug, Default)]
pub(crate) struct Symbol {
  pub(crate) function: Option<Function>,
  pub(crate) value: Option<Value>,
}
