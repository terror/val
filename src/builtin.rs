use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Builtin {
  Constant {
    name: &'static str,
    value: fn(Config) -> Number,
  },
  Function {
    arity: BuiltinArity,
    function: BuiltinFunction,
    name: &'static str,
  },
}

impl Builtin {
  #[must_use]
  pub fn kind(&self) -> &'static str {
    match self {
      Self::Constant { .. } => "constant",
      Self::Function { .. } => "function",
    }
  }

  #[must_use]
  pub fn name(&self) -> &'static str {
    match self {
      Self::Constant { name, .. } | Self::Function { name, .. } => name,
    }
  }
}
