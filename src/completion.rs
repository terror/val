use super::*;

pub enum Completion<'a> {
  Break,
  Continue,
  Return(Value<'a>),
  Value(Value<'a>),
}
