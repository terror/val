use super::*;

#[derive(Serialize, Tsify)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum ParseResult {
  Error { errors: Vec<ValError> },
  Ok { ast: AstNode },
}
