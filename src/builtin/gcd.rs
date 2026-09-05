use super::*;

builtin! {
  Gcd {
    name: "gcd",
    arity: BuiltinArity::Exact(2),
    call(payload) {
      let name = payload.name;

      let integer = |argument: &Value| {
        argument.number(payload.span)?.to_integer().ok_or_else(|| {
          Error::new(
            payload.span,
            format!("Arguments to `{name}` must be finite integers"),
          )
        })
      };

      let a = integer(&payload.arguments[0])?.abs();
      let b = integer(&payload.arguments[1])?.abs();

      Ok(Value::Number(Number::from(a.gcd(&b))))
    }
  }
}
