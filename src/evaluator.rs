use super::*;

pub struct Evaluator {
  pub(crate) context: Context,
  pub(crate) environment: Environment,
}

impl Evaluator {
  fn assign(
    &mut self,
    target: &Spanned<AssignmentTarget>,
    value: Value,
  ) -> Result<(), Error> {
    match &target.0 {
      AssignmentTarget::Identifier(name) => {
        self.environment.assign_symbol(name, value);
        Ok(())
      }
      AssignmentTarget::ListAccess(_, _) => {
        let (name, name_span) = target.0.root(target.1);

        let indices = target.0.indices();

        let Some(root) = self.environment.resolve_symbol(name) else {
          return Err(Error::new(
            name_span,
            format!("Undefined variable `{name}`"),
          ));
        };

        let root =
          self.assign_indices(name, root, &indices, value, target.1)?;

        self.environment.assign_symbol(name, root);

        Ok(())
      }
    }
  }

  fn assign_indices(
    &mut self,
    name: &str,
    value: Value,
    indices: &[&Spanned<Expression>],
    assigned: Value,
    span: Span,
  ) -> Result<Value, Error> {
    let Some((index, rest)) = indices.split_first() else {
      return Ok(assigned);
    };

    let mut list = match value {
      Value::List(items) => items,
      other => {
        return Err(Error::new(
          index.1,
          format!("'{}' is not a list (found {})", name, other.type_name()),
        ));
      }
    };

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

    let value = std::mem::replace(&mut list[index], Value::Null);

    list[index] = self.assign_indices(name, value, rest, assigned, span)?;

    Ok(Value::List(list))
  }

  pub(crate) fn enter_function<T>(
    &mut self,
    f: impl FnOnce(&mut Self) -> Result<T, Error>,
  ) -> Result<T, Error> {
    self.context.enter_function();
    let result = f(self);
    self.context.exit_function();
    result
  }

  fn enter_loop<T>(
    &mut self,
    f: impl FnOnce(&mut Self) -> Result<T, Error>,
  ) -> Result<T, Error> {
    self.context.enter_loop();
    let result = f(self);
    self.context.exit_loop();
    result
  }

  /// # Errors
  ///
  /// Returns an evaluation error when a statement or expression is invalid.
  pub fn evaluate(
    &mut self,
    ast: &Spanned<Program>,
  ) -> Result<Evaluation, Error> {
    let (node, _) = ast;

    let result = match node {
      Program::Statements(statements) => self
        .evaluate_statements(statements)
        .map(|completion| match completion {
          Completion::Return(value) | Completion::Value(value) => value,
          Completion::Break | Completion::Continue => Value::Null,
        }),
    };

    match result {
      Ok(value) => Ok(Evaluation::Value(value)),
      Err(Error::Exit { code, span }) => Ok(Evaluation::Exit { code, span }),
      Err(error) => Err(error),
    }
  }

  fn evaluate_expression(
    &mut self,
    ast: &Spanned<Expression>,
  ) -> Result<Value, Error> {
    let (node, span) = ast;

    match node {
      Expression::BinaryOp(BinaryOp::Add, lhs, rhs) => {
        let (lhs_val, rhs_val) = (
          self.evaluate_expression(lhs)?,
          self.evaluate_expression(rhs)?,
        );

        match (lhs_val, rhs_val) {
          (Value::Number(a), Value::Number(b)) => {
            Ok(Value::Number(a.add(&b, self.environment.config)))
          }
          (Value::String(mut a), Value::String(b)) => {
            a.push_str(&b);
            Ok(Value::String(a))
          }
          (Value::String(mut a), rhs) => {
            a.push_str(&rhs.display(self.environment.config));
            Ok(Value::String(a))
          }
          (lhs, Value::String(b)) => {
            let mut result = lhs.display(self.environment.config);
            result.push_str(&b);
            Ok(Value::String(result))
          }
          (Value::List(mut a), Value::List(b)) => {
            a.extend(b);
            Ok(Value::List(a))
          }
          (lhs_value, rhs_value) => Ok(Value::Number(
            lhs_value
              .number(lhs.1)?
              .add(rhs_value.number(rhs.1)?, self.environment.config),
          )),
        }
      }
      Expression::BinaryOp(BinaryOp::Divide, lhs, rhs) => {
        let (lhs_val, rhs_val) = (
          self.evaluate_expression(lhs)?,
          self.evaluate_expression(rhs)?,
        );

        let (lhs_num, rhs_num) =
          (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

        lhs_num
          .div(rhs_num, self.environment.config)
          .map(Value::Number)
          .map_err(|error| error.with_span(rhs.1))
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
            *span,
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
          self.evaluate_expression(lhs)?.boolean(lhs.1)?
            && self.evaluate_expression(rhs)?.boolean(rhs.1)?,
        ))
      }
      Expression::BinaryOp(BinaryOp::LogicalOr, lhs, rhs) => {
        Ok(Value::Boolean(
          self.evaluate_expression(lhs)?.boolean(lhs.1)?
            || self.evaluate_expression(rhs)?.boolean(rhs.1)?,
        ))
      }
      Expression::BinaryOp(BinaryOp::Modulo, lhs, rhs) => {
        let (lhs_val, rhs_val) = (
          self.evaluate_expression(lhs)?,
          self.evaluate_expression(rhs)?,
        );

        let (lhs_num, rhs_num) =
          (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

        lhs_num
          .rem(rhs_num, self.environment.config)
          .map(Value::Number)
          .map_err(|error| error.with_span(rhs.1))
      }
      Expression::BinaryOp(BinaryOp::Multiply, lhs, rhs) => Ok(Value::Number(
        self.evaluate_expression(lhs)?.number(lhs.1)?.mul(
          self.evaluate_expression(rhs)?.number(rhs.1)?,
          self.environment.config,
        ),
      )),
      Expression::BinaryOp(BinaryOp::NotEqual, lhs, rhs) => Ok(Value::Boolean(
        self.evaluate_expression(lhs)? != self.evaluate_expression(rhs)?,
      )),
      Expression::BinaryOp(BinaryOp::Power, lhs, rhs) => {
        let (lhs_val, rhs_val) = (
          self.evaluate_expression(lhs)?,
          self.evaluate_expression(rhs)?,
        );

        let (lhs_num, rhs_num) =
          (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

        lhs_num
          .pow(rhs_num, self.environment.config)
          .map(Value::Number)
          .map_err(|error| error.with_span(rhs.1))
      }
      Expression::BinaryOp(BinaryOp::Subtract, lhs, rhs) => Ok(Value::Number(
        self.evaluate_expression(lhs)?.number(lhs.1)?.sub(
          self.evaluate_expression(rhs)?.number(rhs.1)?,
          self.environment.config,
        ),
      )),
      Expression::Boolean(boolean) => Ok(Value::Boolean(*boolean)),
      Expression::Function(parameters, body) => {
        Ok(Value::Function(Function::UserDefined {
          body: body.clone(),
          environment: self.environment.clone(),
          identity: Rc::new(()),
          name: None,
          parameters: parameters.clone(),
        }))
      }
      Expression::FunctionCall(function, arguments) => {
        let function = match &function.0 {
          Expression::Identifier(name) => {
            self.environment.function(name, *span)
          }
          _ => match self.evaluate_expression(function)? {
            Value::Function(function) => Ok(function),
            value => Err(Error::new(
              function.1,
              format!("'{value}' is not a function"),
            )),
          },
        }?;

        function.check_arity(arguments.len(), *span)?;

        let mut evaluated_arguments = Vec::with_capacity(arguments.len());

        for argument in arguments {
          evaluated_arguments.push(self.evaluate_expression(argument)?);
        }

        function.call(evaluated_arguments, self.environment.config, *span)
      }
      Expression::Identifier(name) => {
        match self.environment.resolve_symbol(name) {
          Some(value) => Ok(value),
          None => {
            Err(Error::new(*span, format!("Undefined variable `{name}`")))
          }
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
        let list = self.evaluate_expression(list)?.into_list(list.1)?;

        let index = self.evaluate_list_index(index)?;

        if index >= list.len() {
          return Err(Error::new(
            *span,
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
        self.evaluate_expression(rhs)?.number(rhs.1)?.neg(),
      )),
      Expression::UnaryOp(UnaryOp::Not, rhs) => Ok(Value::Boolean(
        !self.evaluate_expression(rhs)?.boolean(rhs.1)?,
      )),
    }
  }

  fn evaluate_list_index(
    &mut self,
    index: &Spanned<Expression>,
  ) -> Result<usize, Error> {
    self
      .evaluate_expression(index)?
      .number(index.1)?
      .to_non_negative_usize()
      .ok_or_else(|| {
        Error::new(index.1, "List index must be a non-negative finite number")
      })
  }

  pub(crate) fn evaluate_statement(
    &mut self,
    statement: &Spanned<Statement>,
  ) -> Result<Completion, Error> {
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
          return Err(Error::new(
            *span,
            "Cannot use 'break' outside of a loop",
          ));
        }

        Ok(Completion::Break)
      }
      Statement::Continue => {
        if !self.context.inside_loop() {
          return Err(Error::new(
            *span,
            "Cannot use 'continue' outside of a loop",
          ));
        }

        Ok(Completion::Continue)
      }
      Statement::Expression(expression) => {
        Ok(Completion::Value(self.evaluate_expression(expression)?))
      }
      Statement::For(name, iterable, body) => {
        let list = self.evaluate_expression(iterable)?.into_list(iterable.1)?;

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
        let function = Function::UserDefined {
          body: body.clone(),
          environment: self.environment.clone(),
          identity: Rc::new(()),
          name: Some(name.clone()),
          parameters: params.clone(),
        };

        self.environment.add_function(name, function.clone());

        Ok(Completion::Value(Value::Function(function)))
      }
      Statement::If(condition, then_branch, else_branch) => {
        if self.evaluate_expression(condition)?.boolean(condition.1)? {
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
          return Err(Error::new(*span, "Cannot return outside of a function"));
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
            .boolean(condition.1)?
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
  ) -> Result<Completion, Error> {
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
}
