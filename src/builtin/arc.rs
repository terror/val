use super::*;

builtin! {
  Arc {
    name: "arc",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.atan(payload.config)))
    }
  }
}
