use super::*;

builtin! {
  Gcd {
    name: "gcd",
    arity: BuiltinArity::Exact(2),
    call(payload) {
      let (a, b) = (payload.integer(0)?.abs(), payload.integer(1)?.abs());

      Ok(Value::Number(Number::from(a.gcd(&b))))
    }
  }
}
