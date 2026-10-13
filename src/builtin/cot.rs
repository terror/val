use super::*;

builtin! {
  Cot {
    name: "cot",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let argument = payload.number(0)?;

      if argument.tan(payload.config).is_zero() {
        return Err(Error::new(
          payload.span,
          "Cannot compute cot of multiple of π",
        ));
      }

      Ok(Value::Number(argument.cot(payload.config)))
    }
  }
}
