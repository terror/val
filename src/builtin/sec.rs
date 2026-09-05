use super::*;

builtin! {
  Sec {
    name: "sec",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let cos = payload.arguments[0]
        .number(payload.span)?
        .cos(payload.config);

      if cos.is_zero() {
        return Err(Error::new(payload.span, "Cannot compute sec of π/2 + nπ"));
      }

      Number::from(1_i64)
        .div(&cos, payload.config)
        .map(Value::Number)
        .map_err(|error| error.with_span(payload.span))
    }
  }
}
