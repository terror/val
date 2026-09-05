use super::*;

builtin! {
  Float {
    name: "float",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let value = &payload.arguments[0];

      match value {
        Value::Number(number) => {
          Ok(Value::Number(number.to_approx(payload.config)))
        }
        Value::String(s) => Number::try_from(s.as_str())
          .map(|number| Value::Number(number.to_approx(payload.config)))
          .map_err(|_| {
            Error::new(payload.span, format!("Cannot convert '{s}' to float"))
          }),
        Value::Boolean(b) => {
          Ok(Value::Number(Number::from(*b).to_approx(payload.config)))
        }
        _ => Err(Error::new(
          payload.span,
          format!("Cannot convert {} to float", value.type_name()),
        )),
      }
    }
  }
}
