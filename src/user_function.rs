use super::*;

#[derive(Debug)]
pub struct UserFunction {
  pub body: Vec<Spanned<Statement>>,
  pub environment: Environment,
  pub name: Option<String>,
  pub parameters: Vec<String>,
}

impl UserFunction {
  pub(crate) fn call(
    function: &Gc<Self>,
    arguments: Vec<Value>,
  ) -> Result<Value> {
    let environment = Environment::with_parent(function.environment.clone());

    if let Some(name) = &function.name {
      environment.add_function(name, Function::UserDefined(function.clone()));
    }

    for (parameter, argument) in function.parameters.iter().zip(arguments) {
      environment.add_symbol(parameter, argument);
    }

    match Evaluator::for_function(environment)
      .evaluate_statements(&function.body)?
    {
      Completion::Return(value) | Completion::Value(value) => Ok(value),
      Completion::Break | Completion::Continue => Ok(Value::Null),
    }
  }

  pub(crate) fn check_arity(&self, len: usize, span: Span) -> Result<()> {
    BuiltinArity::Exact(self.parameters.len()).check(self.name(), len, span)
  }

  pub(crate) fn name(&self) -> &str {
    self.name.as_deref().unwrap_or("<anonymous>")
  }
}

impl Finalize for UserFunction {}

unsafe impl Trace for UserFunction {
  gc::custom_trace!(this, {
    let Self {
      body: _,
      environment,
      name: _,
      parameters: _,
    } = this;

    unsafe { mark(environment) };
  });
}
