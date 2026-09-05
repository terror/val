use super::*;

builtin! {
  Cos {
    name: "cos",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(
        payload.arguments[0]
          .number(payload.span)?
          .cos(payload.config),
      ))
    }
  }
}
