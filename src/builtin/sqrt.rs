use super::*;

builtin! {
  Sqrt {
    name: "sqrt",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let number = payload.number(0)?;

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
