use super::*;

builtin! {
  Floor {
    name: "floor",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(
        payload.arguments[0].number(payload.span)?.floor(),
      ))
    }
  }
}
