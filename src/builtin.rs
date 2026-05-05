use super::*;

pub type BuiltinFunction =
  for<'src> fn(BuiltinFunctionPayload<'src>) -> Result<Value<'src>, Error>;

pub type BuiltinConstant = fn(Config) -> Value<'static>;

pub struct BuiltinFunctionPayload<'src> {
  pub arguments: Vec<Value<'src>>,
  pub config: Config,
  pub span: Span,
}

#[derive(Clone, Copy, Debug)]
pub enum Builtin {
  Constant {
    name: &'static str,
    value: BuiltinConstant,
  },
  Function {
    function: BuiltinFunction,
    name: &'static str,
  },
}

impl Builtin {
  pub fn kind(&self) -> &'static str {
    match self {
      Self::Constant { .. } => "constant",
      Self::Function { .. } => "function",
    }
  }

  pub fn name(&self) -> &'static str {
    match self {
      Self::Constant { name, value: _ }
      | Self::Function { function: _, name } => name,
    }
  }

  pub fn value(&self) -> BuiltinConstant {
    match self {
      Self::Constant { name: _, value } => *value,
      Self::Function { .. } => unreachable!(),
    }
  }

  pub fn function(&self) -> BuiltinFunction {
    match self {
      Self::Constant { .. } => unreachable!(),
      Self::Function { function, name: _ } => *function,
    }
  }
}
