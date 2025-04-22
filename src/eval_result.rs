use super::*;

pub enum EvalResult<'a> {
  Break,
  Continue,
  Return(Value<'a>),
  Value(Value<'a>),
}

impl<'a> EvalResult<'a> {
  pub fn unwrap(&self) -> Value<'a> {
    match self {
      EvalResult::Value(v) | EvalResult::Return(v) => v.clone(),
      EvalResult::Break | EvalResult::Continue => Value::Null,
    }
  }

  pub fn is_return(&self) -> bool {
    matches!(self, EvalResult::Return(_))
  }

  pub fn is_break(&self) -> bool {
    matches!(self, EvalResult::Break)
  }

  pub fn is_continue(&self) -> bool {
    matches!(self, EvalResult::Continue)
  }
}
