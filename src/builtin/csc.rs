use super::*;

builtin! {
  Csc {
    name: "csc",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let sin = payload.number(0)?.sin(payload.config);

      if sin.is_zero() {
        return Err(Error::new(
          payload.span,
          "Cannot compute csc of multiple of π",
        ));
      }

      Number::from(1_i64)
        .div(&sin, payload.config)
        .map(Value::Number)
        .map_err(|error| error.with_span(payload.span))
    }
  }
}
