use super::*;

builtin! {
  Tanh {
    name: "tanh",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(
        payload.arguments[0]
          .number(payload.span)?
          .tanh(payload.config),
      ))
    }
  }
}
