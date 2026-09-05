use super::*;

builtin! {
  E {
    name: "e",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.exp(payload.config)))
    }
  }
}
