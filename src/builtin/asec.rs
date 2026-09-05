use super::*;

builtin! {
  Asec {
    name: "asec",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let argument = payload.number(0)?;

      if argument.abs() < Number::from(1_i64) {
        return Err(Error::new(
          payload.span,
          "asec argument must have absolute value at least 1",
        ));
      }

      let reciprocal = Number::from(1_i64)
        .div(argument, payload.config)
        .map_err(|error| error.with_span(payload.span))?;

      Ok(Value::Number(reciprocal.acos(payload.config)))
    }
  }
}
