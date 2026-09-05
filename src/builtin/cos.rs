use super::*;

builtin! {
  Cos {
    name: "cos",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.cos(payload.config)))
    }
  }
}
