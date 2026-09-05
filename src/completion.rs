use super::*;

pub enum Completion {
  Break,
  Continue,
  Return(Value),
  Value(Value),
}
