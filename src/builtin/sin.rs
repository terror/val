use super::*;

builtin! {
  Sin {
    name: "sin",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.sin(payload.config)))
    }
  }
}
