use super::*;

pub enum Completion<'a> {
  Break,
  Continue,
  Return(Value<'a>),
  Value(Value<'a>),
}

impl<'a> Completion<'a> {
  pub(crate) fn unwrap(&self) -> Value<'a> {
    match self {
      Completion::Return(value) | Completion::Value(value) => value.clone(),
      Completion::Break | Completion::Continue => Value::Null,
    }
  }
}
