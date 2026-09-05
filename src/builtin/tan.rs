use super::*;

builtin! {
  Tan {
    name: "tan",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(
        payload.arguments[0]
          .number(payload.span)?
          .tan(payload.config),
      ))
    }
  }
}
