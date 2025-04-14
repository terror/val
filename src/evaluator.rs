use super::*;

pub fn eval<'a>(
  ast: &Spanned<Ast<'a>>,
  env: &Environment<'a>,
) -> Result<Value<'a>, Error> {
  let (node, span) = ast;

  match node {
    Ast::BinaryOp(BinaryOp::Add, lhs, rhs) => Ok(Value::Number(
      eval(lhs, env)?.number(lhs.1)? + eval(rhs, env)?.number(rhs.1)?,
    )),
    Ast::BinaryOp(BinaryOp::Divide, lhs, rhs) => {
      let (lhs_val, rhs_val) = (eval(lhs, env)?, eval(rhs, env)?);

      let (lhs_num, rhs_num) = (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

      if rhs_num == 0.0 {
        return Err(Error::new(rhs.1, "Division by zero"));
      }

      Ok(Value::Number(lhs_num / rhs_num))
    }
    Ast::BinaryOp(BinaryOp::Equal, lhs, rhs) => {
      Ok(Value::Boolean(eval(lhs, env)? == eval(rhs, env)?))
    }
    Ast::BinaryOp(BinaryOp::GreaterThan, lhs, rhs) => {
      Ok(Value::Boolean(eval(lhs, env)? > eval(rhs, env)?))
    }
    Ast::BinaryOp(BinaryOp::GreaterThanEqual, lhs, rhs) => {
      Ok(Value::Boolean(eval(lhs, env)? >= eval(rhs, env)?))
    }
    Ast::BinaryOp(BinaryOp::LessThan, lhs, rhs) => {
      Ok(Value::Boolean(eval(lhs, env)? < eval(rhs, env)?))
    }
    Ast::BinaryOp(BinaryOp::LessThanEqual, lhs, rhs) => {
      Ok(Value::Boolean(eval(lhs, env)? <= eval(rhs, env)?))
    }
    Ast::BinaryOp(BinaryOp::Modulo, lhs, rhs) => {
      let (lhs_val, rhs_val) = (eval(lhs, env)?, eval(rhs, env)?);

      let (lhs_num, rhs_num) = (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

      if rhs_num == 0.0 {
        return Err(Error::new(rhs.1, "Modulo by zero"));
      }

      Ok(Value::Number(lhs_num % rhs_num))
    }
    Ast::BinaryOp(BinaryOp::Multiply, lhs, rhs) => Ok(Value::Number(
      eval(lhs, env)?.number(lhs.1)? * eval(rhs, env)?.number(rhs.1)?,
    )),
    Ast::BinaryOp(BinaryOp::NotEqual, lhs, rhs) => {
      Ok(Value::Boolean(eval(lhs, env)? != eval(rhs, env)?))
    }
    Ast::BinaryOp(BinaryOp::Power, lhs, rhs) => {
      let (lhs_val, rhs_val) = (eval(lhs, env)?, eval(rhs, env)?);

      let (lhs_num, rhs_num) = (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

      Ok(Value::Number(lhs_num.powf(rhs_num)))
    }
    Ast::BinaryOp(BinaryOp::Subtract, lhs, rhs) => Ok(Value::Number(
      eval(lhs, env)?.number(lhs.1)? - eval(rhs, env)?.number(rhs.1)?,
    )),
    Ast::Boolean(boolean) => Ok(Value::Boolean(*boolean)),
    Ast::FunctionCall(name, arguments) => {
      let mut evaluated_arguments = Vec::with_capacity(arguments.len());

      for argument in arguments {
        evaluated_arguments.push(eval(argument, env)?);
      }

      env.call_function(name, evaluated_arguments, *span)
    }
    Ast::Identifier(name) => match env.get_variable(name) {
      Some(value) => Ok(value.clone()),
      None => Err(Error::new(*span, format!("Undefined variable `{}`", name))),
    },
    Ast::List(list) => {
      let mut evaluated_list = Vec::with_capacity(list.len());

      for item in list {
        evaluated_list.push(eval(item, env)?);
      }

      Ok(Value::List(evaluated_list))
    }
    Ast::Number(number) => Ok(Value::Number(*number)),
    Ast::String(string) => Ok(Value::String(string)),
    Ast::UnaryOp(UnaryOp::Negate, rhs) => {
      Ok(Value::Number(-eval(rhs, env)?.number(rhs.1)?))
    }
    Ast::UnaryOp(UnaryOp::Not, rhs) => {
      Ok(Value::Boolean(!eval(rhs, env)?.boolean(rhs.1)?))
    }
  }
}
