use super::*;

pub(crate) fn eval<'a>(ast: &Spanned<Ast<'a>>) -> Result<Value<'a>, Error> {
  let (node, span) = ast;

  match node {
    Ast::BinaryOp(BinaryOp::Add, lhs, rhs) => {
      Ok(Value::Num(eval(lhs)?.num(lhs.1)? + eval(rhs)?.num(rhs.1)?))
    }
    Ast::BinaryOp(BinaryOp::Div, lhs, rhs) => {
      let (lhs_val, rhs_val) = (eval(lhs)?, eval(rhs)?);

      let (lhs_num, rhs_num) = (lhs_val.num(lhs.1)?, rhs_val.num(rhs.1)?);

      if rhs_num == 0.0 {
        return Err(Error::new(rhs.1, "Division by zero"));
      }

      Ok(Value::Num(lhs_num / rhs_num))
    }
    Ast::BinaryOp(BinaryOp::Mod, lhs, rhs) => {
      let (lhs_val, rhs_val) = (eval(lhs)?, eval(rhs)?);

      let (lhs_num, rhs_num) = (lhs_val.num(lhs.1)?, rhs_val.num(rhs.1)?);

      if rhs_num == 0.0 {
        return Err(Error::new(rhs.1, "Modulo by zero"));
      }

      Ok(Value::Num(lhs_num / rhs_num))
    }
    Ast::BinaryOp(BinaryOp::Mul, lhs, rhs) => {
      Ok(Value::Num(eval(lhs)?.num(lhs.1)? * eval(rhs)?.num(rhs.1)?))
    }
    Ast::BinaryOp(BinaryOp::Sub, lhs, rhs) => {
      Ok(Value::Num(eval(lhs)?.num(lhs.1)? - eval(rhs)?.num(rhs.1)?))
    }
    Ast::Call(func_name, args) => match *func_name {
      "cos" => {
        if args.len() != 1 {
          return Err(Error::new(
            *span,
            format!("Function 'cos' expects 1 argument, got {}", args.len()),
          ));
        }
        let arg_val = eval(&args[0])?;
        Ok(Value::Num(arg_val.num(args[0].1)?.cos()))
      }
      "sin" => {
        if args.len() != 1 {
          return Err(Error::new(
            *span,
            format!("Function 'sin' expects 1 argument, got {}", args.len()),
          ));
        }
        let arg_val = eval(&args[0])?;
        Ok(Value::Num(arg_val.num(args[0].1)?.sin()))
      }
      _ => Err(Error::new(
        *span,
        format!("Function '{}' is not implemented", func_name),
      )),
    },
    Ast::Identifier(name) => {
      Err(Error::new(*span, format!("Undefined variable '{}'", name)))
    }
    Ast::Number(n) => Ok(Value::Num(*n)),
    Ast::UnaryOp(UnaryOp::Neg, rhs) => Ok(Value::Num(-eval(rhs)?.num(rhs.1)?)),
  }
}
