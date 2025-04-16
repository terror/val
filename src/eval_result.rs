use super::*;

pub enum EvalResult<'a> {
  Value(Value<'a>),
  Return(Value<'a>),
}

impl<'a> EvalResult<'a> {
  pub fn unwrap(&self) -> Value<'a> {
    match self {
      EvalResult::Value(v) | EvalResult::Return(v) => v.clone(),
    }
  }

  pub fn is_return(&self) -> bool {
    matches!(self, EvalResult::Return(_))
  }
}
