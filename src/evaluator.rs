use {super::*, crate::context::Context};

pub struct Evaluator<'a> {
  context: Context,
  pub environment: Environment<'a>,
}

impl<'a> From<Environment<'a>> for Evaluator<'a> {
  fn from(environment: Environment<'a>) -> Self {
    Self {
      environment,
      context: Context::default(),
    }
  }
}

fn finite_non_negative_usize(number: f64) -> Option<usize> {
  if number.is_finite() && number >= 0.0 {
    let number = number.trunc();
    format!("{number:.0}").parse().ok()
  } else {
    None
  }
}

impl<'a> Evaluator<'a> {
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
  pub fn eval(
    &mut self,
    ast: &Spanned<Program<'a>>,
  ) -> Result<Value<'a>, Error> {
    let (node, _) = ast;

    match node {
      Program::Statements(statements) => {
        let mut result = Value::Null;

        for statement in statements {
          let completion = self.eval_statement(statement)?;

          result = completion.unwrap();

          if completion.is_return()
            || completion.is_break()
            || completion.is_continue()
          {
            break;
          }
        }

        Ok(result)
      }
    }
  }

  pub(crate) fn eval_statement(
    &mut self,
    statement: &Spanned<Statement<'a>>,
  ) -> Result<Completion<'a>, Error> {
    let (node, span) = statement;

    match node {
      Statement::Assignment(lhs, rhs) => {
        let value = self.eval_expression(rhs)?;

        self.assign(lhs, value.clone())?;

        Ok(Completion::Value(value))
      }
      Statement::Block(statements) => {
        let mut result = Value::Null;

        for statement in statements {
          let completion = self.eval_statement(statement)?;

          result = completion.unwrap();

          if completion.is_return()
            || completion.is_break()
            || completion.is_continue()
          {
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
        Ok(Completion::Value(self.eval_expression(expression)?))
      }
      Statement::For(name, iterable, body) => {
        let list = self.eval_expression(iterable)?.list(iterable.1)?;
        let mut result = Value::Null;

        self.enter_loop(|evaluator| {
          for item in list {
            evaluator.environment.add_symbol(name, item);

            for statement in body {
              let completion = evaluator.eval_statement(statement)?;

              result = completion.unwrap();

              if completion.is_return() {
                return Ok(Completion::Return(result));
              } else if completion.is_break() {
                return Ok(Completion::Value(result));
              } else if completion.is_continue() {
                break;
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
        if self.eval_expression(condition)?.boolean(condition.1)? {
          let mut result = Value::Null;

          for statement in then_branch {
            let completion = self.eval_statement(statement)?;

            result = completion.unwrap();

            if completion.is_return()
              || completion.is_break()
              || completion.is_continue()
            {
              return Ok(completion);
            }
          }

          Ok(Completion::Value(result))
        } else if let Some(else_statements) = else_branch {
          let mut result = Value::Null;

          for statement in else_statements {
            let completion = self.eval_statement(statement)?;

            result = completion.unwrap();

            if completion.is_return()
              || completion.is_break()
              || completion.is_continue()
            {
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
            let completion = evaluator.eval_statement(statement)?;

            let result = completion.unwrap();

            if completion.is_return() {
              return Ok(Completion::Return(result));
            } else if completion.is_break() {
              return Ok(Completion::Value(result));
            } else if completion.is_continue() {
              break;
            }
          }
        }
      }),
      Statement::Return(expr) => {
        if !self.context.inside_function() {
          return Err(Error::new(*span, "Cannot return outside of a function"));
        }

        Ok(Completion::Return(match expr {
          Some(expr) => self.eval_expression(expr)?,
          None => Value::Null,
        }))
      }
      Statement::While(condition, body) => {
        let mut result = Value::Null;

        self.enter_loop(|evaluator| {
          while evaluator.eval_expression(condition)?.boolean(condition.1)? {
            for statement in body {
              let completion = evaluator.eval_statement(statement)?;

              result = completion.unwrap();

              if completion.is_return() {
                return Ok(Completion::Return(result));
              } else if completion.is_break() {
                return Ok(Completion::Value(result));
              } else if completion.is_continue() {
                break;
              }
            }
          }

          Ok(Completion::Value(result))
        })
      }
    }
  }

  fn assign(
    &mut self,
    target: &Spanned<AssignmentTarget<'a>>,
    value: Value<'a>,
  ) -> Result<(), Error> {
    match &target.0 {
      AssignmentTarget::Identifier(name) => {
        self.environment.add_symbol(name, value);
        Ok(())
      }
      AssignmentTarget::ListAccess(_, _) => {
        let (name, name_span) = Self::assignment_root(target);
        let mut indices = Vec::new();
        Self::assignment_indices(target, &mut indices);

        let Some(root) = self.environment.resolve_symbol(name) else {
          return Err(Error::new(
            name_span,
            format!("Undefined variable `{name}`"),
          ));
        };

        let root =
          self.assign_indices(name, root, &indices, value, target.1)?;
        self.environment.add_symbol(name, root);
        Ok(())
      }
    }
  }

  fn assignment_root(
    target: &Spanned<AssignmentTarget<'a>>,
  ) -> (&'a str, Span) {
    match &target.0 {
      AssignmentTarget::Identifier(name) => (name, target.1),
      AssignmentTarget::ListAccess(base, _) => Self::assignment_root(base),
    }
  }

  fn assignment_indices<'target>(
    target: &'target Spanned<AssignmentTarget<'a>>,
    indices: &mut Vec<&'target Spanned<Expression<'a>>>,
  ) {
    match &target.0 {
      AssignmentTarget::Identifier(_) => {}
      AssignmentTarget::ListAccess(base, index) => {
        Self::assignment_indices(base, indices);
        indices.push(index);
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

    let index = self.list_index(index)?;

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

  fn list_index(
    &mut self,
    index: &Spanned<Expression<'a>>,
  ) -> Result<usize, Error> {
    self
      .eval_expression(index)?
      .number(index.1)?
      .to_f64(self.environment.config.rounding_mode)
      .and_then(finite_non_negative_usize)
      .ok_or_else(|| {
        Error::new(index.1, "List index must be a non-negative finite number")
      })
  }

  fn eval_expression(
    &mut self,
    ast: &Spanned<Expression<'a>>,
  ) -> Result<Value<'a>, Error> {
    let (node, span) = ast;

    match node {
      Expression::BinaryOp(BinaryOp::Add, lhs, rhs) => {
        let (lhs_val, rhs_val) =
          (self.eval_expression(lhs)?, self.eval_expression(rhs)?);

        match (&lhs_val, &rhs_val) {
          (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.add(
            b,
            self.environment.config.precision,
            self.environment.config.rounding_mode,
          ))),
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
          _ => Ok(Value::Number(lhs_val.number(lhs.1)?.add(
            &rhs_val.number(rhs.1)?,
            self.environment.config.precision,
            self.environment.config.rounding_mode,
          ))),
        }
      }
      Expression::BinaryOp(BinaryOp::Divide, lhs, rhs) => {
        let (lhs_val, rhs_val) =
          (self.eval_expression(lhs)?, self.eval_expression(rhs)?);

        let (lhs_num, rhs_num) =
          (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

        if rhs_num.is_zero() {
          return Err(Error::new(rhs.1, "Division by zero"));
        }

        Ok(Value::Number(lhs_num.div(
          &rhs_num,
          self.environment.config.precision,
          self.environment.config.rounding_mode,
        )))
      }
      Expression::BinaryOp(BinaryOp::Equal, lhs, rhs) => Ok(Value::Boolean(
        self.eval_expression(lhs)? == self.eval_expression(rhs)?,
      )),
      Expression::BinaryOp(
        op @ (BinaryOp::LessThan
        | BinaryOp::LessThanEqual
        | BinaryOp::GreaterThan
        | BinaryOp::GreaterThanEqual),
        lhs,
        rhs,
      ) => {
        let (lhs_val, rhs_val) =
          (self.eval_expression(lhs)?, self.eval_expression(rhs)?);

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
          self.eval_expression(lhs)?.boolean(lhs.1)?
            && self.eval_expression(rhs)?.boolean(rhs.1)?,
        ))
      }
      Expression::BinaryOp(BinaryOp::LogicalOr, lhs, rhs) => {
        Ok(Value::Boolean(
          self.eval_expression(lhs)?.boolean(lhs.1)?
            || self.eval_expression(rhs)?.boolean(rhs.1)?,
        ))
      }
      Expression::BinaryOp(BinaryOp::Modulo, lhs, rhs) => {
        let (lhs_val, rhs_val) =
          (self.eval_expression(lhs)?, self.eval_expression(rhs)?);

        let (lhs_num, rhs_num) =
          (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

        if rhs_num.is_zero() {
          return Err(Error::new(rhs.1, "Modulo by zero"));
        }

        let quotient = lhs_num.div(
          &rhs_num,
          self.environment.config.precision,
          self.environment.config.rounding_mode,
        );

        let floored_quotient = quotient.floor();

        let product = floored_quotient.mul(
          &rhs_num,
          self.environment.config.precision,
          self.environment.config.rounding_mode,
        );

        let remainder = lhs_num.sub(
          &product,
          self.environment.config.precision,
          self.environment.config.rounding_mode,
        );

        Ok(Value::Number(remainder))
      }
      Expression::BinaryOp(BinaryOp::Multiply, lhs, rhs) => Ok(Value::Number(
        self.eval_expression(lhs)?.number(lhs.1)?.mul(
          &self.eval_expression(rhs)?.number(rhs.1)?,
          self.environment.config.precision,
          self.environment.config.rounding_mode,
        ),
      )),
      Expression::BinaryOp(BinaryOp::NotEqual, lhs, rhs) => Ok(Value::Boolean(
        self.eval_expression(lhs)? != self.eval_expression(rhs)?,
      )),
      Expression::BinaryOp(BinaryOp::Power, lhs, rhs) => {
        let (lhs_val, rhs_val) =
          (self.eval_expression(lhs)?, self.eval_expression(rhs)?);

        let (lhs_num, rhs_num) =
          (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

        let result = with_consts(|consts| {
          lhs_num.pow(
            &rhs_num,
            self.environment.config.precision,
            self.environment.config.rounding_mode,
            consts,
          )
        });

        Ok(Value::Number(result))
      }
      Expression::BinaryOp(BinaryOp::Subtract, lhs, rhs) => Ok(Value::Number(
        self.eval_expression(lhs)?.number(lhs.1)?.sub(
          &self.eval_expression(rhs)?.number(rhs.1)?,
          self.environment.config.precision,
          self.environment.config.rounding_mode,
        ),
      )),
      Expression::Boolean(boolean) => Ok(Value::Boolean(*boolean)),
      Expression::FunctionCall(name, arguments) => {
        let mut evaluated_arguments = Vec::with_capacity(arguments.len());

        for argument in arguments {
          evaluated_arguments.push(self.eval_expression(argument)?);
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
          evaluated_list.push(self.eval_expression(item)?);
        }

        Ok(Value::List(evaluated_list))
      }
      Expression::ListAccess(list, index) => {
        let list_value = self.eval_expression(list)?;

        let list = match &list_value {
          Value::List(items) => items.clone(),
          _ => {
            return Err(Error::new(
              list.1,
              format!("'{list_value}' is not a list"),
            ));
          }
        };

        let Some(index) = self
          .eval_expression(index)?
          .number(index.1)?
          .to_f64(self.environment.config.rounding_mode)
          .and_then(finite_non_negative_usize)
        else {
          return Err(Error::new(
            index.1,
            "List index must be a non-negative finite number",
          ));
        };

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
      Expression::UnaryOp(UnaryOp::Negate, rhs) => {
        Ok(Value::Number(-self.eval_expression(rhs)?.number(rhs.1)?))
      }
      Expression::UnaryOp(UnaryOp::Not, rhs) => {
        Ok(Value::Boolean(!self.eval_expression(rhs)?.boolean(rhs.1)?))
      }
    }
  }
}
