use super::*;

pub struct BuiltinFunctionPayload<'src> {
  pub arguments: Vec<Value<'src>>,
  pub config: Config,
  pub span: Span,
}
