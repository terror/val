use super::*;

#[derive(Clone, Debug)]
pub enum Function {
  Builtin(BuiltinFunction),
  UserDefined(Rc<UserFunction>),
}

impl Function {
  pub(crate) fn call(
    &self,
    arguments: Vec<Value>,
    config: Config,
    span: Span,
  ) -> Result<Value, Error> {
    match self {
      Self::Builtin(function) => function.call(arguments, config, span),
      Self::UserDefined(function) => function.call(arguments),
    }
  }

  pub(crate) fn check_arity(
    &self,
    len: usize,
    span: Span,
  ) -> Result<(), Error> {
    match self {
      Self::Builtin(function) => function.check_arity(len, span),
      Self::UserDefined(function) => function.check_arity(len, span),
    }
  }

  pub(crate) fn name(&self) -> &str {
    match self {
      Self::Builtin(function) => function.name,
      Self::UserDefined(function) => function.name(),
    }
  }
}

impl PartialEq for Function {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Builtin(a), Self::Builtin(b)) => a.name == b.name,
      (Self::UserDefined(a), Self::UserDefined(b)) => Rc::ptr_eq(a, b),
      _ => false,
    }
  }
}
