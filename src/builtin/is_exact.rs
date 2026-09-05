use super::*;

builtin! {
  IsExact {
    name: "is_exact",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      Ok(Value::Boolean(matches!(payload.number(0)?, Number::Exact(_))))
    }
  }
}
