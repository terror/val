use super::*;

builtin! {
  Tanh {
    name: "tanh",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.tanh(payload.config)))
    }
  }
}
