use super::*;

pub struct BuiltinFunctionPayload<'a> {
  pub arguments: Vec<Value<'a>>,
  pub config: Config,
  pub span: Span,
}

pub type BuiltinFunction<'a> =
  fn(BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error>;

#[derive(Clone, Debug)]
pub enum Function<'a> {
  Builtin(BuiltinFunction<'a>),
  UserDefined(Value<'a>),
}
