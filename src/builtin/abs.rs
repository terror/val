use super::*;

builtin! {
  Abs {
    name: "abs",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(
        payload.arguments[0].number(payload.span)?.abs(),
      ))
    }
  }
}
