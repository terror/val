use super::*;

builtin! {
  Int {
    name: "int",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let value = &payload.arguments[0];

      match value {
        Value::Number(number) => Ok(Value::Number(number.floor())),
        Value::String(s) => Number::try_from(s.as_str())
          .map(|number| Value::Number(number.floor()))
          .map_err(|_| {
            Error::new(payload.span, format!("Cannot convert '{s}' to int"))
          }),
        Value::Boolean(b) => Ok(Value::Number(Number::from(*b))),
        _ => Err(Error::new(
          payload.span,
          format!("Cannot convert {} to int", value.type_name()),
        )),
      }
    }
  }
}
