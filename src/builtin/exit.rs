use super::*;

builtin! {
  Exit {
    name: "exit",
    aliases: ["quit"],
    arity: BuiltinArity::Range(0, 1),
    call(payload) {
      let name = payload.name;

      let code = if payload.arguments.is_empty() {
        0
      } else {
        let Some(code) = payload.number(0)?.to_non_negative_usize() else {
          return Err(Error::new(
            payload.span,
            format!("Argument to `{name}` must be a non-negative finite number"),
          ));
        };

        let Ok(code) = i32::try_from(code) else {
          return Err(Error::new(
            payload.span,
            format!("Argument to `{name}` must fit in a 32-bit signed integer"),
          ));
        };

        code
      };

      Err(Error::Exit {
        code,
        span: payload.span,
      })
    }
  }
}
