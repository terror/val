use super::*;

#[derive(Debug, PartialEq)]
pub enum Evaluation {
  Exit { code: i32, span: Span },
  Value(Value),
}
