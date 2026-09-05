use super::*;

builtin! {
  List {
    name: "list",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let value = &payload.arguments[0];

      Ok(match value {
        Value::List(items) => Value::List(items.clone()),
        Value::String(s) => Value::List(
          s.chars()
            .map(|c| Value::String(Cow::Owned(c.to_string())))
            .collect(),
        ),
        _ => Value::List(vec![value.clone()]),
      })
    }
  }
}
