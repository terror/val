use super::*;

builtin! {
  Ceil {
    name: "ceil",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.ceil()))
    }
  }
}
