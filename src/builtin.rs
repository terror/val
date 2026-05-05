use super::*;

pub type BuiltinFunction =
  for<'src> fn(BuiltinFunctionPayload<'src>) -> Result<Value<'src>, Error>;

pub struct BuiltinFunctionPayload<'src> {
  pub arguments: Vec<Value<'src>>,
  pub config: Config,
  pub span: Span,
}

#[derive(Clone, Copy, Debug)]
pub enum Builtin {
  Constant {
    name: &'static str,
    value: fn(Config) -> Value<'static>,
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
      Self::Constant { name, .. } | Self::Function { name, .. } => name,
    }
  }
}
