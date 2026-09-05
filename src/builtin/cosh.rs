use super::*;

builtin! {
  Cosh {
    name: "cosh",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.cosh(payload.config)))
    }
  }
}
