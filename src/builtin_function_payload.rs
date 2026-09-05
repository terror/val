use super::*;

pub struct BuiltinFunctionPayload {
  pub arguments: Vec<Value>,
  pub config: Config,
  pub name: &'static str,
  pub span: Span,
}
