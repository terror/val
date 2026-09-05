use super::*;

builtin! {
  Bool {
    name: "bool",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let value = &payload.arguments[0];

      match value {
        Value::Boolean(b) => Ok(Value::Boolean(*b)),
        Value::Number(n) => Ok(Value::Boolean(!n.is_zero())),
        Value::String(s) => Ok(Value::Boolean(!s.is_empty())),
        Value::List(items) => Ok(Value::Boolean(!items.is_empty())),
        Value::Null => Ok(Value::Boolean(false)),
        Value::Function(_) => Err(Error::new(
          payload.span,
          format!("Cannot convert {} to bool", value.type_name()),
        )),
      }
    }
  }
}
