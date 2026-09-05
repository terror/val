use super::*;

builtin! {
  Log10 {
    name: "log10",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let number = payload.arguments[0].number(payload.span)?;

      if number.is_zero() || number.is_negative() {
        return Err(Error::new(
          payload.span,
          "Cannot take logarithm of zero or negative number",
        ));
      }

      Ok(Value::Number(number.log10(payload.config)))
    }
  }
}
