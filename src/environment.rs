use super::*;

#[derive(Clone, Debug, Default)]
pub struct Environment<'src> {
  pub config: Config,
  pub functions: HashMap<&'src str, Function<'src>>,
  pub parent: Option<Box<Environment<'src>>>,
  pub variables: HashMap<&'src str, Value<'src>>,
}

impl<'src> Environment<'src> {
  pub fn new(config: Config) -> Self {
    let mut env = Self {
      config: config.clone(),
      functions: HashMap::new(),
      parent: None,
      variables: HashMap::new(),
    };

    for builtin in BUILTINS {
      match builtin {
        Builtin::Constant { .. } => {
          env.add_variable(builtin.name(), builtin.value()(config.clone()));
        }
        Builtin::Function { .. } => {
          env.add_function(
            builtin.name(),
            Function::Builtin(builtin.function()),
          );
        }
      }
    }

    env
  }

  pub fn add_function(&mut self, name: &'src str, function: Function<'src>) {
    self.functions.insert(name, function);
  }

  pub fn add_variable(&mut self, name: &'src str, value: Value<'src>) {
    self.variables.insert(name, value);
  }

  pub fn call_function(
    &self,
    name: &str,
    arguments: Vec<Value<'src>>,
    span: Span,
  ) -> Result<Value<'src>, Error> {
    if let Some(function) = self.functions.get(name) {
      match function {
        Function::Builtin(function) => function(BuiltinFunctionPayload {
          arguments,
          config: self.config.clone(),
          span,
        }),
        Function::UserDefined(Value::Function(
          name,
          parameters,
          body,
          environment,
        )) => {
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

          call_environment.add_function(name, function.clone());

          for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
            let parameter_name = *parameter;

            match argument {
              Value::Function(_, _, _, _) => {
                call_environment.add_function(
                  parameter_name,
                  Function::UserDefined(argument.clone()),
                );
              }
              Value::BuiltinFunction(_, builtin) => {
                call_environment
                  .add_function(parameter_name, Function::Builtin(*builtin));
              }
              _ => {
                call_environment.add_variable(parameter_name, argument.clone());
              }
            }
          }

          let mut evaluator = Evaluator::from(call_environment);
          evaluator.inside_function = true;

          if body.is_empty() {
            return Ok(Value::Null);
          }

          for statement in body.iter() {
            let result = evaluator.eval_statement(statement)?;

            if result.is_return() {
              return Ok(result.unwrap());
            }
          }

          Ok(Value::Null)
        }
        _ => Err(Error::new(span, format!("`{}` is not a function", name))),
      }
    } else {
      Err(Error::new(
        span,
        format!("Function `{}` is not defined", name),
      ))
    }
  }

  pub fn resolve_symbol(&self, symbol: &str) -> Option<Value<'src>> {
    if let Some(value) = self.variables.get(symbol) {
      Some(value.clone())
    } else if let Some((name, function)) = self.functions.get_key_value(symbol)
    {
      match function {
        Function::UserDefined(value) => Some(value.clone()),
        Function::Builtin(builtin) => {
          Some(Value::BuiltinFunction(name, *builtin))
        }
      }
    } else if let Some(parent) = &self.parent {
      parent.resolve_symbol(symbol)
    } else {
      None
    }
  }

  pub fn with_parent(parent: Environment<'src>) -> Self {
    Self {
      config: parent.config.clone(),
      functions: parent.functions.clone(),
      parent: Some(Box::new(parent)),
      variables: HashMap::new(),
    }
  }
}
