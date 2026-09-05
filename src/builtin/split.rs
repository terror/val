use super::*;

builtin! {
  Split {
    name: "split",
    arity: BuiltinArity::Exact(2),
    call(payload) {
      let string = payload.arguments[0].string(payload.span)?;

      let delimiter = payload.arguments[1].string(payload.span)?;

      Ok(Value::List(
        string
          .split(delimiter)
          .filter(|part| !part.is_empty())
          .map(|part| Value::String(Cow::Owned(part.to_string())))
          .collect(),
      ))
    }
  }
}
