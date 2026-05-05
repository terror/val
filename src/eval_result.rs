use super::*;

pub enum EvalResult<'a> {
  Break,
  Continue,
  Return(Value<'a>),
  Value(Value<'a>),
}

impl<'a> EvalResult<'a> {
  pub(crate) fn unwrap(&self) -> Value<'a> {
    match self {
      EvalResult::Value(v) | EvalResult::Return(v) => v.clone(),
      EvalResult::Break | EvalResult::Continue => Value::Null,
    }
  }

  pub(crate) fn is_return(&self) -> bool {
    matches!(self, EvalResult::Return(_))
  }

  pub(crate) fn is_break(&self) -> bool {
    matches!(self, EvalResult::Break)
  }

  pub(crate) fn is_continue(&self) -> bool {
    matches!(self, EvalResult::Continue)
  }
}
