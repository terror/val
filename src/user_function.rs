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
    self: &Rc<Self>,
    arguments: Vec<Value>,
  ) -> Result<Value, Error> {
    let environment = Environment::with_parent(self.environment.clone());

    if let Some(name) = &self.name {
      environment.add_function(name, Function::UserDefined(self.clone()));
    }

    for (parameter, argument) in self.parameters.iter().zip(arguments) {
      environment.add_symbol(parameter, argument);
    }

    match Evaluator::for_function(environment)
      .evaluate_statements(&self.body)?
    {
      Completion::Return(value) | Completion::Value(value) => Ok(value),
      Completion::Break | Completion::Continue => Ok(Value::Null),
    }
  }

  pub(crate) fn check_arity(
    &self,
    len: usize,
    span: Span,
  ) -> Result<(), Error> {
    BuiltinArity::Exact(self.parameters.len()).check(self.name(), len, span)
  }

  pub(crate) fn name(&self) -> &str {
    self.name.as_deref().unwrap_or("<anonymous>")
  }
}
