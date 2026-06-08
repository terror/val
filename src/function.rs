use super::*;

#[derive(Clone, Debug)]
pub enum Function<'src> {
  Builtin {
    arity: BuiltinArity,
    function: BuiltinFunction,
    name: &'src str,
  },
  UserDefined {
    body: Vec<Spanned<Statement<'src>>>,
    environment: Environment<'src>,
    name: Option<&'src str>,
    parameters: Vec<&'src str>,
  },
}

impl<'src> Function<'src> {
  pub(crate) fn call(
    &self,
    arguments: Vec<Value<'src>>,
    config: Config,
    span: Span,
  ) -> Result<Value<'src>, Error> {
    match self {
      Self::Builtin {
        arity,
        function,
        name,
      } => {
        arity.check(name, arguments.len(), span)?;

        function.call(&BuiltinFunctionPayload {
          arguments,
          config,
          span,
        })
      }
      Self::UserDefined {
        body,
        environment,
        name,
        parameters,
      } => {
        if parameters.len() != arguments.len() {
          return Err(Error::new(
            span,
            format!(
              "Function `{}` expects {} arguments, got {}",
              self.name(),
              parameters.len(),
              arguments.len()
            ),
          ));
        }

        let call_environment = Environment::with_parent(environment.clone());

        if let Some(name) = name {
          call_environment.add_function(name, self.clone());
        }

        for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
          call_environment.add_symbol(parameter, argument.clone());
        }

        Evaluator::from(call_environment).enter_function(|evaluator| {
          if let Completion::Return(value) =
            evaluator.evaluate_statements(body)?
          {
            Ok(value)
          } else {
            Ok(Value::Null)
          }
        })
      }
    }
  }

  pub(crate) fn name(&self) -> &str {
    match self {
      Self::Builtin { name, .. } => name,
      Self::UserDefined { name, .. } => name.unwrap_or("<anonymous>"),
    }
  }
}

impl PartialEq for Function<'_> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Builtin { name: a, .. }, Self::Builtin { name: b, .. })
      | (
        Self::UserDefined { name: Some(a), .. },
        Self::UserDefined { name: Some(b), .. },
      ) => a == b,
      _ => false,
    }
  }
}
