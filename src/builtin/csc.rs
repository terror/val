use super::*;

builtin! {
  Csc {
    name: "csc",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.csc(payload.config)))
    }
  }
}
