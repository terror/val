use super::*;

builtin! {
  Cot {
    name: "cot",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let tan = payload.arguments[0]
        .number(payload.span)?
        .tan(payload.config);

      if tan.is_zero() {
        return Err(Error::new(
          payload.span,
          "Cannot compute cot of multiple of π",
        ));
      }

      Number::from(1_i64)
        .div(&tan, payload.config)
        .map(Value::Number)
        .map_err(|error| error.with_span(payload.span))
    }
  }
}
