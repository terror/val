use super::*;

#[derive(Clone, Debug)]
pub enum Function<'a> {
  Builtin(BuiltinFunction),
  UserDefined(Value<'a>),
}
