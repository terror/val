use super::*;

builtin! {
  Fraction {
    name: "fraction",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::String(payload.rational(0)?.to_string()))
    }
  }
}
