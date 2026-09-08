use super::*;

pub(crate) enum Completion {
  Break,
  Continue,
  Return(Value),
  Value(Value),
}
