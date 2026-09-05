use super::*;

builtin! {
  Asin {
    name: "asin",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let argument = payload.number(0)?;

      if argument < &Number::from(-1_i64) || argument > &Number::from(1_i64) {
        return Err(Error::new(
          payload.span,
          "asin argument must be between -1 and 1",
        ));
      }

      Ok(Value::Number(argument.asin(payload.config)))
    }
  }
}
