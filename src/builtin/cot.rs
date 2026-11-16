use super::*;

builtin! {
  Cot {
    name: "cot",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.cot(payload.config)))
    }
  }
}
