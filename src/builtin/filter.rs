use super::*;

builtin! {
  Filter {
    name: "filter",
    arity: BuiltinArity::Exact(2),
    call(payload) {
      let list = payload.arguments[0].list(payload.span)?;
      let function = payload.arguments[1].function(payload.span)?;

      function.check_arity(1, payload.span)?;

      let mut filtered = Vec::new();

      for value in list {
        if function.call(
          vec![value.clone()],
          payload.config,
          payload.span,
        )?.boolean(payload.span)? {
          filtered.push(value.clone());
        }
      }

      Ok(Value::List(filtered))
    }
  }
}
