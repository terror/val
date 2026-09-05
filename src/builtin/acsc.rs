use super::*;

builtin! {
  Acsc {
    name: "acsc",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let argument = payload.arguments[0].number(payload.span)?;

      if argument.abs() < Number::from(1_i64) {
        return Err(Error::new(
          payload.span,
          "acsc argument must have absolute value at least 1",
        ));
      }

      let reciprocal = Number::from(1_i64)
        .div(argument, payload.config)
        .map_err(|error| error.with_span(payload.span))?;

      Ok(Value::Number(reciprocal.asin(payload.config)))
    }
  }
}
