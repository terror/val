use super::*;

builtin! {
  Reduce {
    name: "reduce",
    arity: BuiltinArity::Exact(3),
    call(payload) {
      let list = payload.arguments[0].list(payload.span)?;
      let function = payload.arguments[1].function(payload.span)?;

      function.check_arity(2, payload.span)?;

      let mut result = payload.arguments[2].clone();

      for value in list {
        result = function.call(
          vec![result, value.clone()],
          payload.config,
          payload.span,
        )?;
      }

      Ok(result)
    }
  }
}
