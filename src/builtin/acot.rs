use super::*;

builtin! {
  Acot {
    name: "acot",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.acot(payload.config)))
    }
  }
}
