use super::*;

builtin! {
  Tan {
    name: "tan",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.tan(payload.config)))
    }
  }
}
