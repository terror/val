use super::*;

const RED_ZONE: usize = 256 * 1024;

#[derive(Clone, Debug)]
pub enum Function {
  Builtin(BuiltinFunction),
  UserDefined(Gc<UserFunction>),
}

impl Function {
  pub(crate) fn call(
    &self,
    arguments: Vec<Value>,
    environment: &Environment,
    span: &Span,
  ) -> Result<Value> {
    if stacker::remaining_stack().is_some_and(|remaining| remaining < RED_ZONE)
    {
      return Err(Error::new(span, "Maximum recursion depth exceeded"));
    }

    match self {
      Self::Builtin(function) => function.call(arguments, environment, span),
      Self::UserDefined(function) => UserFunction::call(function, arguments),
    }
  }

  pub(crate) fn check_arity(&self, len: usize, span: &Span) -> Result<()> {
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

impl Finalize for Function {}

impl PartialEq for Function {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Builtin(a), Self::Builtin(b)) => a.name == b.name,
      (Self::UserDefined(a), Self::UserDefined(b)) => Gc::ptr_eq(a, b),
      _ => false,
    }
  }
}

unsafe impl Trace for Function {
  gc::custom_trace!(this, {
    match this {
      Self::Builtin(_) => {}
      Self::UserDefined(function) => unsafe { mark(function) },
    }
  });
}
