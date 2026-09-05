use super::*;

#[derive(Clone, Debug)]
pub enum Function {
  Builtin {
    arity: BuiltinArity,
    function: fn(&BuiltinFunctionPayload) -> Result<Value, Error>,
    name: &'static str,
  },
  UserDefined {
    body: Vec<Spanned<Statement>>,
    environment: Environment,
    identity: Rc<()>,
    name: Option<String>,
    parameters: Vec<String>,
  },
}

impl Function {
  pub(crate) fn call(
    &self,
    arguments: Vec<Value>,
    config: Config,
    span: Span,
  ) -> Result<Value, Error> {
    match self {
      Self::Builtin { function, name, .. } => {
        function(&BuiltinFunctionPayload {
          arguments,
          config,
          name,
          span,
        })
      }
      Self::UserDefined {
        body,
        environment,
        name,
        parameters,
        ..
      } => {
        let call_environment = Environment::with_parent(environment.clone());

        if let Some(name) = name {
          call_environment.add_function(name, self.clone());
        }

        for (parameter, argument) in parameters.iter().zip(arguments) {
          call_environment.add_symbol(parameter, argument);
        }

        Evaluator::from(call_environment).enter_function(|evaluator| {
          match evaluator.evaluate_statements(body)? {
            Completion::Return(value) | Completion::Value(value) => Ok(value),
            Completion::Break | Completion::Continue => Ok(Value::Null),
          }
        })
      }
    }
  }

  pub(crate) fn check_arity(
    &self,
    len: usize,
    span: Span,
  ) -> Result<(), Error> {
    match self {
      Self::Builtin { arity, name, .. } => arity.check(name, len, span),
      Self::UserDefined { parameters, .. } => {
        if parameters.len() == len {
          return Ok(());
        }

        Err(Error::new(
          span,
          format!(
            "Function `{}` expects {} arguments, got {}",
            self.name(),
            parameters.len(),
            len
          ),
        ))
      }
    }
  }

  pub(crate) fn name(&self) -> &str {
    match self {
      Self::Builtin { name, .. } => name,
      Self::UserDefined { name, .. } => {
        name.as_deref().unwrap_or("<anonymous>")
      }
    }
  }
}

impl PartialEq for Function {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Builtin { name: a, .. }, Self::Builtin { name: b, .. }) => a == b,
      (
        Self::UserDefined { identity: a, .. },
        Self::UserDefined { identity: b, .. },
      ) => Rc::ptr_eq(a, b),
      _ => false,
    }
  }
}
