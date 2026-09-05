use super::*;

builtin! {
  Sqrt {
    name: "sqrt",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let number = payload.arguments[0].number(payload.span)?;

      if number.is_negative() {
        return Err(Error::new(
          payload.span,
          "Cannot take square root of negative number",
        ));
      }

      Ok(Value::Number(number.sqrt(payload.config)))
    }
  }
}
