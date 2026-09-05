use super::*;

builtin! {
  Arc {
    name: "arc",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(
        payload.arguments[0]
          .number(payload.span)?
          .atan(payload.config),
      ))
    }
  }
}
