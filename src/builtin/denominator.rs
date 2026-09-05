use super::*;

builtin! {
  Denominator {
    name: "denominator",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(Number::from(
        payload.rational(0)?.denom().clone(),
      )))
    }
  }
}
