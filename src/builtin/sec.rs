use super::*;

builtin! {
  Sec {
    name: "sec",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.sec(payload.config)))
    }
  }
}
