use super::*;

builtin! {
  Numerator {
    name: "numerator",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(Number::from(
        payload.rational(0)?.numer().clone(),
      )))
    }
  }
}
