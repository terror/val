use super::*;

builtin! {
  Floor {
    name: "floor",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(payload.number(0)?.floor()))
    }
  }
}
