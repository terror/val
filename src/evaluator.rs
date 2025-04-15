use super::*;

pub struct Evaluator<'a> {
  environment: Environment<'a>,
}

impl<'a> Evaluator<'a> {
  pub fn new() -> Self {
    Self {
      environment: Environment::new(),
    }
  }

  pub fn eval(
    &mut self,
    ast: &Spanned<Program<'a>>,
  ) -> Result<Value<'a>, Error> {
    let (node, _) = ast;

    match node {
      Program::Statements(statements) => {
        let mut result = Value::Null;

        for statement in statements {
          result = self.eval_statement(statement)?;
        }

        Ok(result)
      }
    }
  }

  fn eval_statement(
    &mut self,
    statement: &Spanned<Statement<'a>>,
  ) -> Result<Value<'a>, Error> {
    let (node, _) = statement;

    match node {
      Statement::Assignment(name, expr) => {
        let value = self.eval_expression(expr)?;
        self.environment.add_variable(name, value.clone());
        Ok(value)
      }
      Statement::Block(statements) => {
        let mut result = Value::Null;

        for statement in statements {
          result = self.eval_statement(statement)?;
        }

        Ok(result)
      }
      Statement::Expression(expression) => self.eval_expression(expression),
    }
  }

  fn eval_expression(
    &mut self,
    ast: &Spanned<Expression<'a>>,
  ) -> Result<Value<'a>, Error> {
    let (node, span) = ast;

    match node {
      Expression::BinaryOp(BinaryOp::Add, lhs, rhs) => Ok(Value::Number(
        self.eval_expression(lhs)?.number(lhs.1)?
          + self.eval_expression(rhs)?.number(rhs.1)?,
      )),
      Expression::BinaryOp(BinaryOp::Divide, lhs, rhs) => {
        let (lhs_val, rhs_val) =
          (self.eval_expression(lhs)?, self.eval_expression(rhs)?);

        let (lhs_num, rhs_num) =
          (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

        if rhs_num == 0.0 {
          return Err(Error::new(rhs.1, "Division by zero"));
        }

        Ok(Value::Number(lhs_num / rhs_num))
      }
      Expression::BinaryOp(BinaryOp::Equal, lhs, rhs) => Ok(Value::Boolean(
        self.eval_expression(lhs)? == self.eval_expression(rhs)?,
      )),
      Expression::BinaryOp(BinaryOp::GreaterThan, lhs, rhs) => Ok(
        Value::Boolean(self.eval_expression(lhs)? > self.eval_expression(rhs)?),
      ),
      Expression::BinaryOp(BinaryOp::GreaterThanEqual, lhs, rhs) => {
        Ok(Value::Boolean(
          self.eval_expression(lhs)? >= self.eval_expression(rhs)?,
        ))
      }
      Expression::BinaryOp(BinaryOp::LessThan, lhs, rhs) => Ok(Value::Boolean(
        self.eval_expression(lhs)? < self.eval_expression(rhs)?,
      )),
      Expression::BinaryOp(BinaryOp::LessThanEqual, lhs, rhs) => {
        Ok(Value::Boolean(
          self.eval_expression(lhs)? <= self.eval_expression(rhs)?,
        ))
      }
      Expression::BinaryOp(BinaryOp::Modulo, lhs, rhs) => {
        let (lhs_val, rhs_val) =
          (self.eval_expression(lhs)?, self.eval_expression(rhs)?);

        let (lhs_num, rhs_num) =
          (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

        if rhs_num == 0.0 {
          return Err(Error::new(rhs.1, "Modulo by zero"));
        }

        Ok(Value::Number(lhs_num % rhs_num))
      }
      Expression::BinaryOp(BinaryOp::Multiply, lhs, rhs) => Ok(Value::Number(
        self.eval_expression(lhs)?.number(lhs.1)?
          * self.eval_expression(rhs)?.number(rhs.1)?,
      )),
      Expression::BinaryOp(BinaryOp::NotEqual, lhs, rhs) => Ok(Value::Boolean(
        self.eval_expression(lhs)? != self.eval_expression(rhs)?,
      )),
      Expression::BinaryOp(BinaryOp::Power, lhs, rhs) => {
        let (lhs_val, rhs_val) =
          (self.eval_expression(lhs)?, self.eval_expression(rhs)?);

        let (lhs_num, rhs_num) =
          (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

        Ok(Value::Number(lhs_num.powf(rhs_num)))
      }
      Expression::BinaryOp(BinaryOp::Subtract, lhs, rhs) => Ok(Value::Number(
        self.eval_expression(lhs)?.number(lhs.1)?
          - self.eval_expression(rhs)?.number(rhs.1)?,
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
      Expression::Identifier(name) => match self.environment.get_variable(name)
      {
        Some(value) => Ok(value.clone()),
        None => {
          Err(Error::new(*span, format!("Undefined variable `{}`", name)))
        }
      },
      Expression::List(list) => {
        let mut evaluated_list = Vec::with_capacity(list.len());

        for item in list {
          evaluated_list.push(self.eval_expression(item)?);
        }

        Ok(Value::List(evaluated_list))
      }
      Expression::ListAccess(list, index) => {
        let list_value = self.eval_expression(list)?;

        let index_value = self.eval_expression(index)?;

        let list_items = match &list_value {
          Value::List(items) => items.clone(),
          _ => {
            return Err(Error::new(
              list.1,
              format!("'{}' is not a list", list_value),
            ));
          }
        };

        let idx = index_value.number(index.1)? as usize;

        if idx >= list_items.len() {
          return Err(Error::new(
            *span,
            format!(
              "Index {} out of bounds for list of length {}",
              idx,
              list_items.len()
            ),
          ));
        }

        Ok(list_items[idx].clone())
      }
      Expression::Number(number) => Ok(Value::Number(*number)),
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
