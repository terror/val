use super::*;

pub struct Evaluator<'a> {
  pub(crate) context: Context,
  pub(crate) environment: Environment<'a>,
}

impl<'a> Evaluator<'a> {
  fn assign(
    &mut self,
    target: &Spanned<AssignmentTarget<'a>>,
    value: Value<'a>,
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
    name: &'a str,
    value: Value<'a>,
    indices: &[&Spanned<Expression<'a>>],
    assigned: Value<'a>,
    span: Span,
  ) -> Result<Value<'a>, Error> {
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

    list[index] =
      self.assign_indices(name, list[index].clone(), rest, assigned, span)?;

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
    ast: &Spanned<Program<'a>>,
  ) -> Result<Value<'a>, Error> {
    let (node, _) = ast;

    match node {
      Program::Statements(statements) => {
        let mut result = Value::Null;

        for statement in statements {
          let completion = self.evaluate_statement(statement)?;

          result = completion.unwrap();

          if matches!(
            &completion,
            Completion::Return(_) | Completion::Break | Completion::Continue
          ) {
            break;
          }
        }

        Ok(result)
      }
    }
  }

  fn evaluate_expression(
    &mut self,
    ast: &Spanned<Expression<'a>>,
  ) -> Result<Value<'a>, Error> {
    let (node, span) = ast;

    match node {
      Expression::BinaryOp(BinaryOp::Add, lhs, rhs) => {
        let (lhs_val, rhs_val) = (
          self.evaluate_expression(lhs)?,
          self.evaluate_expression(rhs)?,
        );

        match (&lhs_val, &rhs_val) {
          (Value::Number(a), Value::Number(b)) => {
            Ok(Value::Number(a.add(b, self.environment.config)))
          }
          (Value::String(a), Value::String(b)) => {
            Ok(Value::String(Box::leak(format!("{a}{b}").into_boxed_str())))
          }
          (Value::String(a), _) => Ok(Value::String(Box::leak(
            format!("{a}{rhs_val}").into_boxed_str(),
          ))),
          (_, Value::String(b)) => Ok(Value::String(Box::leak(
            format!("{lhs_val}{b}").into_boxed_str(),
          ))),
          (Value::List(a), Value::List(b)) => {
            let mut result = a.clone();
            result.extend(b.clone());
            Ok(Value::List(result))
          }
          _ => Ok(Value::Number(
            lhs_val
              .number(lhs.1)?
              .add(&rhs_val.number(rhs.1)?, self.environment.config),
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

        if rhs_num.is_zero() {
          return Err(Error::new(rhs.1, "Division by zero"));
        }

        Ok(Value::Number(
          lhs_num.div(&rhs_num, self.environment.config),
        ))
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

        if rhs_num.is_zero() {
          return Err(Error::new(rhs.1, "Modulo by zero"));
        }

        Ok(Value::Number(
          lhs_num.rem(&rhs_num, self.environment.config),
        ))
      }
      Expression::BinaryOp(BinaryOp::Multiply, lhs, rhs) => Ok(Value::Number(
        self.evaluate_expression(lhs)?.number(lhs.1)?.mul(
          &self.evaluate_expression(rhs)?.number(rhs.1)?,
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

        Ok(Value::Number(
          lhs_num.pow(&rhs_num, self.environment.config),
        ))
      }
      Expression::BinaryOp(BinaryOp::Subtract, lhs, rhs) => Ok(Value::Number(
        self.evaluate_expression(lhs)?.number(lhs.1)?.sub(
          &self.evaluate_expression(rhs)?.number(rhs.1)?,
          self.environment.config,
        ),
      )),
      Expression::Boolean(boolean) => Ok(Value::Boolean(*boolean)),
      Expression::FunctionCall(name, arguments) => {
        let mut evaluated_arguments = Vec::with_capacity(arguments.len());

        for argument in arguments {
          evaluated_arguments.push(self.evaluate_expression(argument)?);
        }

        self
          .environment
          .call_function(name, evaluated_arguments, *span)
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
        let list_value = self.evaluate_expression(list)?;

        let list = match &list_value {
          Value::List(items) => items.clone(),
          _ => {
            return Err(Error::new(
              list.1,
              format!("'{list_value}' is not a list"),
            ));
          }
        };

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

        Ok(list[index].clone())
      }
      Expression::Null => Ok(Value::Null),
      Expression::Number(number) => Ok(Value::Number(number.clone())),
      Expression::String(string) => Ok(Value::String(string)),
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
    index: &Spanned<Expression<'a>>,
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
    statement: &Spanned<Statement<'a>>,
  ) -> Result<Completion<'a>, Error> {
    let (node, span) = statement;

    match node {
      Statement::Assignment(lhs, rhs) => {
        let value = self.evaluate_expression(rhs)?;

        self.assign(lhs, value.clone())?;

        Ok(Completion::Value(value))
      }
      Statement::Block(statements) => {
        let mut result = Value::Null;

        for statement in statements {
          let completion = self.evaluate_statement(statement)?;

          result = completion.unwrap();

          if matches!(
            &completion,
            Completion::Return(_) | Completion::Break | Completion::Continue
          ) {
            return Ok(completion);
          }
        }

        Ok(Completion::Value(result))
      }
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
        let list = self.evaluate_expression(iterable)?.list(iterable.1)?;

        let mut result = Value::Null;

        self.enter_loop(|evaluator| {
          for item in list {
            evaluator.environment.add_symbol(name, item);

            for statement in body {
              let completion = evaluator.evaluate_statement(statement)?;

              result = completion.unwrap();

              match completion {
                Completion::Break => return Ok(Completion::Value(result)),
                Completion::Continue => break,
                Completion::Return(_) => return Ok(Completion::Return(result)),
                Completion::Value(_) => {}
              }
            }
          }

          Ok(Completion::Value(result))
        })
      }
      Statement::Function(name, params, body) => {
        let function = Function::UserDefined {
          body: body.clone(),
          environment: self.environment.clone(),
          name,
          parameters: params.clone(),
        };

        self.environment.add_function(name, function.clone());

        Ok(Completion::Value(Value::Function(function)))
      }
      Statement::If(condition, then_branch, else_branch) => {
        if self.evaluate_expression(condition)?.boolean(condition.1)? {
          let mut result = Value::Null;

          for statement in then_branch {
            let completion = self.evaluate_statement(statement)?;

            result = completion.unwrap();

            if matches!(
              &completion,
              Completion::Return(_) | Completion::Break | Completion::Continue
            ) {
              return Ok(completion);
            }
          }

          Ok(Completion::Value(result))
        } else if let Some(else_statements) = else_branch {
          let mut result = Value::Null;

          for statement in else_statements {
            let completion = self.evaluate_statement(statement)?;

            result = completion.unwrap();

            if matches!(
              &completion,
              Completion::Return(_) | Completion::Break | Completion::Continue
            ) {
              return Ok(completion);
            }
          }

          Ok(Completion::Value(result))
        } else {
          Ok(Completion::Value(Value::Null))
        }
      }
      Statement::Loop(body) => self.enter_loop(|evaluator| {
        loop {
          for statement in body {
            let completion = evaluator.evaluate_statement(statement)?;

            let result = completion.unwrap();

            match completion {
              Completion::Break => return Ok(Completion::Value(result)),
              Completion::Continue => break,
              Completion::Return(_) => return Ok(Completion::Return(result)),
              Completion::Value(_) => {}
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
            for statement in body {
              let completion = evaluator.evaluate_statement(statement)?;

              result = completion.unwrap();

              match completion {
                Completion::Break => return Ok(Completion::Value(result)),
                Completion::Continue => break,
                Completion::Return(_) => return Ok(Completion::Return(result)),
                Completion::Value(_) => {}
              }
            }
          }

          Ok(Completion::Value(result))
        })
      }
    }
  }
}

impl<'a> From<Environment<'a>> for Evaluator<'a> {
  fn from(environment: Environment<'a>) -> Self {
    Self {
      environment,
      context: Context::default(),
    }
  }
}
