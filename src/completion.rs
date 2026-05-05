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
      Completion::Value(v) | Completion::Return(v) => v.clone(),
      Completion::Break | Completion::Continue => Value::Null,
    }
  }

  pub(crate) fn is_return(&self) -> bool {
    matches!(self, Completion::Return(_))
  }

  pub(crate) fn is_break(&self) -> bool {
    matches!(self, Completion::Break)
  }

  pub(crate) fn is_continue(&self) -> bool {
    matches!(self, Completion::Continue)
  }
}
