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
    Ast::BinaryOp(BinaryOp::Div, lhs, rhs) => {
      let (lhs_val, rhs_val) = (eval(lhs, env)?, eval(rhs, env)?);

      let (lhs_num, rhs_num) = (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

      if rhs_num == 0.0 {
        return Err(Error::new(rhs.1, "Division by zero"));
      }

      Ok(Value::Number(lhs_num / rhs_num))
    }
    Ast::BinaryOp(BinaryOp::Mod, lhs, rhs) => {
      let (lhs_val, rhs_val) = (eval(lhs, env)?, eval(rhs, env)?);

      let (lhs_num, rhs_num) = (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

      if rhs_num == 0.0 {
        return Err(Error::new(rhs.1, "Modulo by zero"));
      }

      Ok(Value::Number(lhs_num % rhs_num))
    }
    Ast::BinaryOp(BinaryOp::Mul, lhs, rhs) => Ok(Value::Number(
      eval(lhs, env)?.number(lhs.1)? * eval(rhs, env)?.number(rhs.1)?,
    )),
    Ast::BinaryOp(BinaryOp::Pow, lhs, rhs) => {
      let (lhs_val, rhs_val) = (eval(lhs, env)?, eval(rhs, env)?);

      let (lhs_num, rhs_num) = (lhs_val.number(lhs.1)?, rhs_val.number(rhs.1)?);

      Ok(Value::Number(lhs_num.powf(rhs_num)))
    }
    Ast::BinaryOp(BinaryOp::Sub, lhs, rhs) => Ok(Value::Number(
      eval(lhs, env)?.number(lhs.1)? - eval(rhs, env)?.number(rhs.1)?,
    )),
    Ast::FunctionCall(name, arguments) => {
      let mut evaluated_arguments = Vec::with_capacity(arguments.len());

      for argument in arguments {
        evaluated_arguments.push(eval(argument, env)?);
      }

      env.call_function(name, evaluated_arguments, *span)
    }
    Ast::Identifier(name) => match env.get_variable(name) {
      Some(value) => Ok(value.clone()),
      None => Err(Error::new(*span, format!("Undefined variable '{}'", name))),
    },
    Ast::Number(n) => Ok(Value::Number(*n)),
    Ast::UnaryOp(UnaryOp::Neg, rhs) => {
      Ok(Value::Number(-eval(rhs, env)?.number(rhs.1)?))
    }
  }
}
