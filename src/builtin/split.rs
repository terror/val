use super::*;

builtin! {
  Split {
    name: "split",
    arity: BuiltinArity::Exact(2),
    call(payload) {
      let string = payload.arguments[0].string(payload.span)?;

      let delimiter = payload.arguments[1].string(payload.span)?;

      Ok(Value::List(if delimiter.is_empty() {
        string
          .chars()
          .map(|character| Value::String(character.to_string()))
          .collect()
      } else {
        string
          .split(delimiter)
          .map(|part| Value::String(part.to_string()))
          .collect()
      }))
    }
  }
}
