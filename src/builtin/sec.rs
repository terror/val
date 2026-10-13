use super::*;

builtin! {
  Sec {
    name: "sec",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let argument = payload.number(0)?;

      if argument.cos(payload.config).is_zero() {
        return Err(Error::new(payload.span, "Cannot compute sec of π/2 + nπ"));
      }

      Ok(Value::Number(argument.sec(payload.config)))
    }
  }
}
