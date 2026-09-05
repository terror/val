use super::*;

builtin! {
  Cosh {
    name: "cosh",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Number(
        payload.arguments[0]
          .number(payload.span)?
          .cosh(payload.config),
      ))
    }
  }
}
