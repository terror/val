use super::*;

builtin! {
  Join {
    name: "join",
    arity: BuiltinArity::Exact(2),
    call(payload) {
      let list = payload.arguments[0].list(payload.span)?;

      let delimiter = payload.arguments[1].string(payload.span)?;

      let joined_string = list
        .iter()
        .map(|value| match value {
          Value::String(s) => s.to_string(),
          _ => value.display(payload.config),
        })
        .collect::<Vec<_>>()
        .join(delimiter);

      Ok(Value::String(Cow::Owned(joined_string)))
    }
  }
}
