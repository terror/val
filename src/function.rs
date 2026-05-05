use super::*;

#[derive(Clone, Debug)]
pub enum Function<'src> {
  Builtin {
    function: BuiltinFunction,
    name: &'src str,
  },
  UserDefined {
    body: Vec<Spanned<Statement<'src>>>,
    environment: Environment<'src>,
    name: &'src str,
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
      Self::Builtin { function, .. } => function(&BuiltinFunctionPayload {
        arguments,
        config,
        span,
      }),
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
              name,
              parameters.len(),
              arguments.len()
            ),
          ));
        }

        let mut call_environment =
          Environment::with_parent(environment.clone());

        call_environment.add_function(name, self.clone());

        for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
          call_environment.add_symbol(parameter, argument.clone());
        }

        Evaluator::from(call_environment).enter_function(|evaluator| {
          if body.is_empty() {
            return Ok(Value::Null);
          }

          for statement in body {
            let result = evaluator.eval_statement(statement)?;

            if result.is_return() {
              return Ok(result.unwrap());
            }
          }

          Ok(Value::Null)
        })
      }
    }
  }

  pub(crate) fn name(&self) -> &'src str {
    match self {
      Self::Builtin { name, .. } | Self::UserDefined { name, .. } => name,
    }
  }
}
