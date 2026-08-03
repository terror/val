use super::*;

#[derive(Debug, PartialEq)]
pub enum Evaluation<'a> {
  Exit { code: i32, span: Span },
  Value(Value<'a>),
}
