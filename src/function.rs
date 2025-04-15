use super::*;

pub type BuiltinFunction<'src> =
  fn(Vec<Value<'src>>, Span) -> Result<Value<'src>, Error>;

#[derive(Clone, Debug)]
pub enum Function<'a> {
  Builtin(BuiltinFunction<'a>),
  UserDefined(Value<'a>),
}
