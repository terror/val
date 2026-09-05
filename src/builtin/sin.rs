use super::*;

builtin! {
  Sin {
    name: "sin",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(
        payload.arguments[0]
          .number(payload.span)?
          .sin(payload.config),
      ))
    }
  }
}
