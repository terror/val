use super::*;

#[derive(Clone, Debug, Serialize)]
pub enum ErrorKind {
  Evaluator,
  Parser,
}

#[derive(Clone, Debug, Serialize)]
pub struct ValError {
  pub kind: ErrorKind,
  pub message: String,
  pub range: Range,
}
