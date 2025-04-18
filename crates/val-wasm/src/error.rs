use super::*;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub enum ErrorKind {
  Evaluator,
  Parser,
}

#[derive(Debug, Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct ValError {
  pub kind: ErrorKind,
  pub message: String,
  pub range: Range,
}
