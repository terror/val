use super::*;

builtin! {
  Sinh {
    name: "sinh",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.sinh(payload.config)))
    }
  }
}
