use super::*;

builtin! {
  Csc {
    name: "csc",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let argument = payload.number(0)?;

      if argument.sin(payload.config).is_zero() {
        return Err(Error::new(
          payload.span,
          "Cannot compute csc of multiple of π",
        ));
      }

      Ok(Value::Number(argument.csc(payload.config)))
    }
  }
}
