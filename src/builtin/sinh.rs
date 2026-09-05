use super::*;

builtin! {
  Sinh {
    name: "sinh",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(
        payload.arguments[0]
          .number(payload.span)?
          .sinh(payload.config),
      ))
    }
  }
}
