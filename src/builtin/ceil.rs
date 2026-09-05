use super::*;

builtin! {
  Ceil {
    name: "ceil",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(
        payload.arguments[0].number(payload.span)?.ceil(),
      ))
    }
  }
}
