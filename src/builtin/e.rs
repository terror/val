use super::*;

builtin! {
  E {
    name: "e",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(
        payload.arguments[0]
          .number(payload.span)?
          .exp(payload.config),
      ))
    }
  }
}
