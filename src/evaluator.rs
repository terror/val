use super::*;

pub struct Evaluator {
  pub(crate) context: Context,
  pub(crate) environment: Environment,
}

impl Evaluator {
  fn assign(
    &mut self,
    mut target: &Spanned<AssignmentTarget>,
    value: Value,
  ) -> Result<()> {
    let span = &target.1;
    let mut indices = Vec::new();

    let (name, name_span) = loop {
      match &target.0 {
        AssignmentTarget::Identifier(name) => break (name, &target.1),
        AssignmentTarget::ListAccess(base, index) => {
          indices.push(index.as_ref());
          target = base;
        }
      }
    };

    if indices.is_empty() {
      self.environment.assign_symbol(name, value);
      return Ok(());
    }

    let indices = indices
      .into_iter()
      .rev()
      .map(|index| {
        self
          .evaluate_list_index(index)
          .map(|value| (value, &index.1))
      })
      .collect::<Result<Vec<_>>>()?;

    let Some(mut root) = self.environment.resolve_symbol(name) else {
      return Err(Error::new(
        name_span,
        format!("Undefined variable `{name}`"),
      ));
    };

    let mut target = &mut root;

    for (index, index_span) in indices {
      let Value::List(list) = target else {
        return Err(Error::new(
          index_span,
          format!("'{}' is not a list (found {})", name, target.type_name()),
        ));
      };

      if index >= list.len() {
        return Err(Error::new(
          span,
          format!(
            "Index {} out of bounds for list of length {}",
            index,
            list.len()
          ),
        ));
      }

      target = &mut list[index];
    }

    *target = value;

    self.environment.assign_symbol(name, root);

    Ok(())
  }

  fn enter_loop<T>(
    &mut self,
    f: impl FnOnce(&mut Self) -> Result<T>,
  ) -> Result<T> {
    self.context.enter_loop();
    let result = f(self);
    self.context.exit_loop();
    result
  }

  /// # Errors
  ///
  /// Returns an evaluation error when a statement or expression is invalid.
  pub fn evaluate(&mut self, ast: &Spanned<Program>) -> Result<Evaluation> {
    let (node, _) = ast;

    let result = self
      .evaluate_statements(&node.statements)
      .map(|completion| match completion {
        Completion::Return(value) | Completion::Value(value) => value,
        Completion::Break | Completion::Continue => Value::Null,
      });

    match result {
      Ok(value) => Ok(Evaluation::Value(value)),
      Err(Error::Exit { code, span }) => Ok(Evaluation::Exit { code, span }),
      Err(error) => Err(error),
    }
  }

  fn evaluate_expression(
    &mut self,
    ast: &Spanned<Expression>,
  ) -> Result<Value> {
    let (node, span) = ast;

    match node {
      Expression::BinaryOp(
        op @ (BinaryOp::Add
        | BinaryOp::Divide
        | BinaryOp::Modulo
        | BinaryOp::Multiply
        | BinaryOp::Power
        | BinaryOp::Subtract),
        lhs,
        rhs,
      ) => {
        let (lhs_value, rhs_value) = (
          self.evaluate_expression(lhs)?,
          self.evaluate_expression(rhs)?,
        );

        let config = self.environment.config;

        match (op, lhs_value, rhs_value) {
          (BinaryOp::Add, Value::String(mut a), Value::String(b)) => {
            a.push_str(&b);
            Ok(Value::String(a))
          }
          (BinaryOp::Add, Value::String(mut a), rhs) => {
            a.push_str(&rhs.display(config));
            Ok(Value::String(a))
          }
          (BinaryOp::Add, lhs, Value::String(b)) => {
            let mut result = lhs.display(config);
            result.push_str(&b);
            Ok(Value::String(result))
          }
          (BinaryOp::Add, Value::List(mut a), Value::List(b)) => {
            a.extend(b);
            Ok(Value::List(a))
          }
          (_, lhs_value, rhs_value) => {
            let (lhs_value, rhs_value) =
              (lhs_value.number(&lhs.1)?, rhs_value.number(&rhs.1)?);

            match op {
              BinaryOp::Add => Ok(lhs_value.add(rhs_value, config)),
              BinaryOp::Divide => lhs_value.div(rhs_value, config),
              BinaryOp::Modulo => lhs_value.rem(rhs_value, config),
              BinaryOp::Multiply => Ok(lhs_value.mul(rhs_value, config)),
              BinaryOp::Power => lhs_value.pow(rhs_value, config),
              BinaryOp::Subtract => Ok(lhs_value.sub(rhs_value, config)),
              _ => unreachable!(),
            }
            .map(Value::Number)
            .map_err(|error| error.with_span(&rhs.1))
          }
        }
      }
      Expression::BinaryOp(BinaryOp::Equal, lhs, rhs) => Ok(Value::Boolean(
        self.evaluate_expression(lhs)? == self.evaluate_expression(rhs)?,
      )),
      Expression::BinaryOp(
        op @ (BinaryOp::LessThan
        | BinaryOp::LessThanEqual
        | BinaryOp::GreaterThan
        | BinaryOp::GreaterThanEqual),
        lhs,
        rhs,
      ) => {
        let (lhs_val, rhs_val) = (
          self.evaluate_expression(lhs)?,
          self.evaluate_expression(rhs)?,
        );

        match (&lhs_val, &rhs_val) {
          (Value::Number(a), Value::Number(b)) => {
            Ok(Value::Boolean(match op {
              BinaryOp::LessThan => a < b,
              BinaryOp::LessThanEqual => a <= b,
              BinaryOp::GreaterThan => a > b,
              BinaryOp::GreaterThanEqual => a >= b,
              _ => unreachable!(),
            }))
          }
          (Value::String(a), Value::String(b)) => {
            Ok(Value::Boolean(match op {
              BinaryOp::LessThan => a < b,
              BinaryOp::LessThanEqual => a <= b,
              BinaryOp::GreaterThan => a > b,
              BinaryOp::GreaterThanEqual => a >= b,
              _ => unreachable!(),
            }))
          }
          _ => Err(Error::new(
            span,
            format!(
              "Cannot compare {} and {} with '{}'",
              lhs_val.type_name(),
              rhs_val.type_name(),
              op
            ),
          )),
        }
      }
      Expression::BinaryOp(BinaryOp::LogicalAnd, lhs, rhs) => {
        Ok(Value::Boolean(
          self.evaluate_expression(lhs)?.boolean(&lhs.1)?
            && self.evaluate_expression(rhs)?.boolean(&rhs.1)?,
        ))
      }
      Expression::BinaryOp(BinaryOp::LogicalOr, lhs, rhs) => {
        Ok(Value::Boolean(
          self.evaluate_expression(lhs)?.boolean(&lhs.1)?
            || self.evaluate_expression(rhs)?.boolean(&rhs.1)?,
        ))
      }
      Expression::BinaryOp(BinaryOp::NotEqual, lhs, rhs) => Ok(Value::Boolean(
        self.evaluate_expression(lhs)? != self.evaluate_expression(rhs)?,
      )),
      Expression::Boolean(boolean) => Ok(Value::Boolean(*boolean)),
      Expression::Function(parameters, body) => Ok(Value::Function(
        Function::UserDefined(Gc::new(UserFunction {
          body: body.clone(),
          environment: self.environment.clone(),
          name: None,
          parameters: parameters.clone(),
          span: span.clone(),
        })),
      )),
      Expression::FunctionCall(function, arguments) => {
        let function = self
          .evaluate_expression(function)?
          .into_function(&function.1)?;

        function.check_arity(arguments.len(), span)?;

        let mut evaluated_arguments = Vec::with_capacity(arguments.len());

        for argument in arguments {
          evaluated_arguments.push(self.evaluate_expression(argument)?);
        }

        function.call(evaluated_arguments, self.environment.config, span)
      }
      Expression::Identifier(name) => {
        match self.environment.resolve_symbol(name) {
          Some(value) => Ok(value),
          None => Err(Error::new(span, format!("Undefined variable `{name}`"))),
        }
      }
      Expression::List(list) => {
        let mut evaluated_list = Vec::with_capacity(list.len());

        for item in list {
          evaluated_list.push(self.evaluate_expression(item)?);
        }

        Ok(Value::List(evaluated_list))
      }
      Expression::ListAccess(list, index) => {
        let list = self.evaluate_expression(list)?.into_list(&list.1)?;

        let index = self.evaluate_list_index(index)?;

        if index >= list.len() {
          return Err(Error::new(
            span,
            format!(
              "Index {} out of bounds for list of length {}",
              index,
              list.len()
            ),
          ));
        }

        Ok(list.into_iter().nth(index).unwrap())
      }
      Expression::Null => Ok(Value::Null),
      Expression::Number(number) => Ok(Value::Number(number.clone())),
      Expression::String(string) => Ok(Value::String(string.clone())),
      Expression::UnaryOp(UnaryOp::Negate, rhs) => Ok(Value::Number(
        self.evaluate_expression(rhs)?.number(&rhs.1)?.neg(),
      )),
      Expression::UnaryOp(UnaryOp::Not, rhs) => Ok(Value::Boolean(
        !self.evaluate_expression(rhs)?.boolean(&rhs.1)?,
      )),
    }
  }

  fn evaluate_list_index(
    &mut self,
    index: &Spanned<Expression>,
  ) -> Result<usize> {
    self
      .evaluate_expression(index)?
      .number(&index.1)?
      .to_non_negative_usize()
      .ok_or_else(|| {
        Error::new(&index.1, "List index must be a non-negative finite number")
      })
  }

  pub(crate) fn evaluate_statement(
    &mut self,
    statement: &Spanned<Statement>,
  ) -> Result<Completion> {
    let (node, span) = statement;

    match node {
      Statement::Assignment(lhs, rhs) => {
        let value = self.evaluate_expression(rhs)?;

        self.assign(lhs, value.clone())?;

        Ok(Completion::Value(value))
      }
      Statement::Block(statements) => self.evaluate_statements(statements),
      Statement::Break => {
        if !self.context.inside_loop() {
          return Err(Error::new(span, "Cannot use 'break' outside of a loop"));
        }

        Ok(Completion::Break)
      }
      Statement::Continue => {
        if !self.context.inside_loop() {
          return Err(Error::new(
            span,
            "Cannot use 'continue' outside of a loop",
          ));
        }

        Ok(Completion::Continue)
      }
      Statement::Expression(expression) => {
        Ok(Completion::Value(self.evaluate_expression(expression)?))
      }
      Statement::For(name, iterable, body) => {
        let list =
          self.evaluate_expression(iterable)?.into_list(&iterable.1)?;

        let mut result = Value::Null;

        self.enter_loop(|evaluator| {
          for item in list {
            evaluator.environment.add_symbol(name, item);

            match evaluator.evaluate_statements(body)? {
              Completion::Break => {
                return Ok(Completion::Value(Value::Null));
              }
              Completion::Continue => result = Value::Null,
              Completion::Return(value) => {
                return Ok(Completion::Return(value));
              }
              Completion::Value(value) => result = value,
            }
          }

          Ok(Completion::Value(result))
        })
      }
      Statement::Function(name, params, body) => {
        let function = Function::UserDefined(Gc::new(UserFunction {
          body: body.clone(),
          environment: self.environment.clone(),
          name: Some(name.clone()),
          parameters: params.clone(),
          span: span.clone(),
        }));

        self
          .environment
          .add_symbol(name, Value::Function(function.clone()));

        Ok(Completion::Value(Value::Function(function)))
      }
      Statement::If(condition, then_branch, else_branch) => {
        if self.evaluate_expression(condition)?.boolean(&condition.1)? {
          self.evaluate_statements(then_branch)
        } else if let Some(else_statements) = else_branch {
          self.evaluate_statements(else_statements)
        } else {
          Ok(Completion::Value(Value::Null))
        }
      }
      Statement::Loop(body) => self.enter_loop(|evaluator| {
        loop {
          match evaluator.evaluate_statements(body)? {
            Completion::Break => {
              return Ok(Completion::Value(Value::Null));
            }
            Completion::Continue | Completion::Value(_) => {}
            Completion::Return(value) => {
              return Ok(Completion::Return(value));
            }
          }
        }
      }),
      Statement::Return(expression) => {
        if !self.context.inside_function() {
          return Err(Error::new(span, "Cannot return outside of a function"));
        }

        Ok(Completion::Return(match expression {
          Some(expression) => self.evaluate_expression(expression)?,
          None => Value::Null,
        }))
      }
      Statement::While(condition, body) => {
        let mut result = Value::Null;

        self.enter_loop(|evaluator| {
          while evaluator
            .evaluate_expression(condition)?
            .boolean(&condition.1)?
          {
            match evaluator.evaluate_statements(body)? {
              Completion::Break => {
                return Ok(Completion::Value(Value::Null));
              }
              Completion::Continue => result = Value::Null,
              Completion::Return(value) => {
                return Ok(Completion::Return(value));
              }
              Completion::Value(value) => result = value,
            }
          }

          Ok(Completion::Value(result))
        })
      }
    }
  }

  pub(crate) fn evaluate_statements(
    &mut self,
    statements: &[Spanned<Statement>],
  ) -> Result<Completion> {
    let mut result = Value::Null;

    for statement in statements {
      let completion = self.evaluate_statement(statement)?;

      match completion {
        Completion::Return(value) => {
          return Ok(Completion::Return(value));
        }
        Completion::Break => return Ok(Completion::Break),
        Completion::Continue => return Ok(Completion::Continue),
        Completion::Value(value) => result = value,
      }
    }

    Ok(Completion::Value(result))
  }

  pub(crate) fn for_function(environment: Environment) -> Self {
    Self {
      context: Context::for_function(),
      environment,
    }
  }
}

impl From<Environment> for Evaluator {
  fn from(environment: Environment) -> Self {
    Self {
      environment,
      context: Context::default(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn exit_is_evaluation_outcome() {
    #[track_caller]
    fn case(source: &str, expected: i32) {
      let ast = parse(source).unwrap();

      let mut evaluator = Evaluator::from(Environment::new(Config::default()));

      let Evaluation::Exit { code, .. } = evaluator.evaluate(&ast).unwrap()
      else {
        panic!("expected exit outcome");
      };

      assert_eq!(code, expected);
    }

    case("exit()", 0);
    case("exit(42)", 42);
    case("fn foo() { exit(1) }\nfoo()", 1);
    case("quit()", 0);
    case("quit(1)", 1);
  }

  #[test]
  fn scientific_notation_round_trip() {
    #[track_caller]
    fn case(source: &str, expected: &str) {
      let config = Config::default();

      let mut evaluator = Evaluator::from(Environment::new(config));

      let Evaluation::Value(value) =
        evaluator.evaluate(&parse(source).unwrap()).unwrap()
      else {
        panic!("expected value");
      };

      let displayed = value.display(config);

      assert_eq!(displayed, expected);

      assert_eq!(
        evaluator.evaluate(&parse(&displayed).unwrap()).unwrap(),
        Evaluation::Value(value)
      );
    }

    case("0.00001", "1e-05");
    case("-0.0000123", "-1.23e-05");
    case("10000000000000000.5", "1.00000000000000005e+16");
  }

  #[test]
  fn user_defined_functions_are_shared() {
    #[track_caller]
    fn case(source: &str) {
      let ast = parse(source).unwrap();

      let mut evaluator = Evaluator::from(Environment::default());

      let Evaluation::Value(value) = evaluator.evaluate(&ast).unwrap() else {
        panic!("expected value");
      };

      let cloned = value.clone();

      assert_eq!(value, cloned);

      let (
        Value::Function(Function::UserDefined(function)),
        Value::Function(Function::UserDefined(cloned)),
      ) = (&value, &cloned)
      else {
        panic!("expected user-defined functions");
      };

      assert!(Gc::ptr_eq(function, cloned));

      assert_ne!(evaluator.evaluate(&ast).unwrap(), Evaluation::Value(value));
    }

    case("fn(foo) { foo }");
    case("fn foo(bar) { bar }");
  }
}
