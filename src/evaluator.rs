use super::*;

pub struct Evaluator<'a> {
  pub environment: Environment<'a>,
  pub inside_function: bool,
  pub inside_loop: bool,
}

impl<'a> From<Environment<'a>> for Evaluator<'a> {
  fn from(environment: Environment<'a>) -> Self {
    Self {
      environment,
      inside_function: false,
      inside_loop: false,
    }
  }
}

impl<'a> Evaluator<'a> {
  pub fn eval(
    &mut self,
    ast: &Spanned<Program<'a>>,
  ) -> Result<Value<'a>, Error> {
    let (node, _) = ast;

    match node {
      Program::Statements(statements) => {
        let mut result = Value::Null;

        for statement in statements {
          let eval_result = self.eval_statement(statement)?;

          result = eval_result.unwrap();

          if eval_result.is_return()
            || eval_result.is_break()
            || eval_result.is_continue()
          {
            break;
          }
        }

        Ok(result)
      }
    }
  }

  pub fn eval_statement(
    &mut self,
    statement: &Spanned<Statement<'a>>,
  ) -> Result<EvalResult<'a>, Error> {
    let (node, span) = statement;

    match node {
      Statement::Assignment(lhs, rhs) => {
        let value = self.eval_expression(rhs)?;

        match &lhs.0 {
          Expression::Identifier(name) => {
            self.environment.add_variable(name, value.clone());
          }
          Expression::ListAccess(base_box, index_box) => {
            let (list_name, list_span) = match &base_box.0 {
              Expression::Identifier(name) => (*name, base_box.1),
              _ => {
                return Err(Error::new(
                  base_box.1,
                  "left‑hand side must be a variable or list element",
                ));
              }
            };

            let mut list = match self.environment.resolve_symbol(list_name) {
              Some(Value::List(items)) => items.clone(),
              Some(other) => {
                return Err(Error::new(
                  list_span,
                  format!(
                    "'{}' is not a list (found {})",
                    list_name,
                    other.type_name()
                  ),
                ));
              }
              None => {
                return Err(Error::new(
                  list_span,
                  format!("Undefined variable `{}`", list_name),
                ));
              }
            };

            todo!()

            // let index = self.eval_expression(index_box)?.number(index_box.1)?.to_usize();

            // if index >= list.len() {
            //   return Err(Error::new(
            //     lhs.1,
            //     format!(
            //       "Index {} out of bounds for list of length {}",
            //       index,
            //       list.len()
            //     ),
            //   ));
            // }

            // list[index as usize] = value.clone();

            // self.environment.add_variable(list_name, Value::List(list));
          }

          _ => {
            return Err(Error::new(
              lhs.1,
              "left‑hand side must be a variable or list element",
            ));
          }
        }

        Ok(EvalResult::Value(value))
      }
      Statement::Block(statements) => {
        let mut result = Value::Null;

        for statement in statements {
          let eval_result = self.eval_statement(statement)?;

          result = eval_result.unwrap();

          if eval_result.is_return()
            || eval_result.is_break()
            || eval_result.is_continue()
          {
            return Ok(eval_result);
          }
        }

        Ok(EvalResult::Value(result))
      }
      Statement::Break => {
        if !self.inside_loop {
          return Err(Error::new(
            *span,
            "Cannot use 'break' outside of a loop",
          ));
        }
        Ok(EvalResult::Break)
      }
      Statement::Continue => {
        if !self.inside_loop {
          return Err(Error::new(
            *span,
            "Cannot use 'continue' outside of a loop",
          ));
        }

        Ok(EvalResult::Continue)
      }
      Statement::Expression(expression) => {
        Ok(EvalResult::Value(self.eval_expression(expression)?))
      }
      Statement::Function(name, params, body) => {
        let function = Value::Function(
          name,
          params.clone(),
          body.clone(),
          self.environment.clone(),
        );

        self
          .environment
          .add_function(name, Function::UserDefined(function.clone()));

        Ok(EvalResult::Value(function))
      }
      Statement::If(condition, then_branch, else_branch) => {
        if self.eval_expression(condition)?.boolean(condition.1)? {
          let mut result = Value::Null;

          for statement in then_branch {
            let eval_result = self.eval_statement(statement)?;

            result = eval_result.unwrap();

            if eval_result.is_return()
              || eval_result.is_break()
              || eval_result.is_continue()
            {
              return Ok(eval_result);
            }
          }

          Ok(EvalResult::Value(result))
        } else if let Some(else_statements) = else_branch {
          let mut result = Value::Null;

          for statement in else_statements {
            let eval_result = self.eval_statement(statement)?;

            result = eval_result.unwrap();

            if eval_result.is_return()
              || eval_result.is_break()
              || eval_result.is_continue()
            {
              return Ok(eval_result);
            }
          }

          Ok(EvalResult::Value(result))
        } else {
          Ok(EvalResult::Value(Value::Null))
        }
      }
      Statement::Loop(body) => {
        let old_inside_loop = self.inside_loop;

        self.inside_loop = true;

        loop {
          for statement in body {
            let eval_result = self.eval_statement(statement)?;

            let result = eval_result.unwrap();

            if eval_result.is_return() {
              self.inside_loop = old_inside_loop;
              return Ok(EvalResult::Return(result));
            } else if eval_result.is_break() {
              self.inside_loop = old_inside_loop;
              return Ok(EvalResult::Value(result));
            } else if eval_result.is_continue() {
              break;
            }
          }
        }
      }
      Statement::Return(expr) => {
        if !self.inside_function {
          return Err(Error::new(*span, "Cannot return outside of a function"));
        }

        Ok(EvalResult::Return(match expr {
          Some(expr) => self.eval_expression(expr)?,
          None => Value::Null,
        }))
      }
      Statement::While(condition, body) => {
        let mut result = Value::Null;

        let old_inside_loop = self.inside_loop;

        self.inside_loop = true;

        while self.eval_expression(condition)?.boolean(condition.1)? {
          for statement in body {
            let eval_result = self.eval_statement(statement)?;

            result = eval_result.unwrap();

            if eval_result.is_return() {
              self.inside_loop = old_inside_loop;
              return Ok(EvalResult::Return(result));
            } else if eval_result.is_break() {
              self.inside_loop = old_inside_loop;
              return Ok(EvalResult::Value(result));
            } else if eval_result.is_continue() {
              break;
            }
          }
        }

        self.inside_loop = old_inside_loop;

        Ok(EvalResult::Value(result))
      }
    }
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
          (Value::Number(a), Value::Number(b)) => Ok(Value::Number(
            Float::with_val_round(
              self.environment.config.precision,
              a + b,
              self.environment.config.rounding_mode,
            )
            .0,
          )),
          (Value::String(a), Value::String(b)) => Ok(Value::String(Box::leak(
            format!("{}{}", a, b).into_boxed_str(),
          ))),
          (Value::String(a), _) => Ok(Value::String(Box::leak(
            format!("{}{}", a, rhs_val).into_boxed_str(),
          ))),
          (_, Value::String(b)) => Ok(Value::String(Box::leak(
            format!("{}{}", lhs_val, b).into_boxed_str(),
          ))),
          (Value::List(a), Value::List(b)) => {
            let mut result = a.clone();
            result.extend(b.clone());
            Ok(Value::List(result))
          }
          _ => Ok(Value::Number(
            Float::with_val_round(
              self.environment.config.precision,
              lhs_val.number(lhs.1)? + rhs_val.number(rhs.1)?,
              self.environment.config.rounding_mode,
            )
            .0,
          )),
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

        Ok(Value::Number(
          Float::with_val_round(
            self.environment.config.precision,
            lhs_num / rhs_num,
            self.environment.config.rounding_mode,
          )
          .0,
        ))
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

        Ok(Value::Number(
          Float::with_val_round(
            self.environment.config.precision,
            lhs_num % rhs_num,
            self.environment.config.rounding_mode,
          )
          .0,
        ))
      }
      Expression::BinaryOp(BinaryOp::Multiply, lhs, rhs) => Ok(Value::Number(
        Float::with_val_round(
          self.environment.config.precision,
          self.eval_expression(lhs)?.number(lhs.1)?
            * self.eval_expression(rhs)?.number(rhs.1)?,
          self.environment.config.rounding_mode,
        )
        .0,
      )),
      Expression::BinaryOp(BinaryOp::NotEqual, lhs, rhs) => Ok(Value::Boolean(
        self.eval_expression(lhs)? != self.eval_expression(rhs)?,
      )),
      Expression::BinaryOp(BinaryOp::Power, lhs, rhs) => {
        let (lhs_val, rhs_val) =
          (self.eval_expression(lhs)?, self.eval_expression(rhs)?);

        let (lhs_num, rhs_num) =
          (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

        Ok(Value::Number(
          Float::with_val_round(
            self.environment.config.precision,
            lhs_num.pow(&rhs_num),
            self.environment.config.rounding_mode,
          )
          .0,
        ))
      }
      Expression::BinaryOp(BinaryOp::Subtract, lhs, rhs) => Ok(Value::Number(
        Float::with_val_round(
          self.environment.config.precision,
          self.eval_expression(lhs)?.number(lhs.1)?
            - self.eval_expression(rhs)?.number(rhs.1)?,
          self.environment.config.rounding_mode,
        )
        .0,
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
          Some(value) => Ok(value.clone()),
          None => {
            Err(Error::new(*span, format!("Undefined variable `{}`", name)))
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
              format!("'{}' is not a list", list_value),
            ));
          }
        };

        todo!()

        // let index = match self
        //   .eval_expression(index)?
        //   .number(index.1)?
        //   .to_f64(self.environment.config.rounding_mode)
        // {
        //   Some(n) if n.is_finite() && n >= 0.0 => n as usize,
        //   _ => {
        //     return Err(Error::new(
        //       index.1,
        //       "List index must be a non-negative finite number",
        //     ));
        //   }
        // };

        // if index >= list.len() {
        //   return Err(Error::new(
        //     *span,
        //     format!(
        //       "Index {} out of bounds for list of length {}",
        //       index,
        //       list.len()
        //     ),
        //   ));
        // }

        // Ok(list[index].clone())
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
