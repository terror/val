use super::*;

builtin! {
  Len {
    name: "len",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let value = &payload.arguments[0];

      match value {
        Value::String(s) => Ok(Value::Number(Number::from(s.chars().count()))),
        Value::List(items) => Ok(Value::Number(Number::from(items.len()))),
        _ => Err(Error::new(
          payload.span,
          format!("Cannot get length of {}", value.type_name()),
        )),
      }
    }
  }
}
