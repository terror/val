use super::*;

builtin! {
  Map {
    name: "map",
    arity: BuiltinArity::Exact(2),
    call(payload) {
      let list = payload.arguments[0].list(payload.span)?;
      let function = payload.arguments[1].function(payload.span)?;

      function.check_arity(1, payload.span)?;

      let mut mapped = Vec::with_capacity(list.len());

      for value in list {
        mapped.push(function.call(
          vec![value.clone()],
          payload.config,
          payload.span,
        )?);
      }

      Ok(Value::List(mapped))
    }
  }
}
