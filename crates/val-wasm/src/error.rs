use super::*;

#[derive(Clone, Debug, Serialize, Tsify)]
pub enum ErrorKind {
  Evaluator,
  Parser,
}

#[derive(Clone, Debug, Serialize, Tsify)]
pub struct ValError {
  pub kind: ErrorKind,
  pub message: String,
  pub range: Range,
}
