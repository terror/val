use super::*;

builtin! {
  Acot {
    name: "acot",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let argument = payload.arguments[0].number(payload.span)?;

      let pi_div_2 = Number::Approx(
        Float::with_val_round(
          payload.config.precision(),
          Constant::Pi,
          payload.config.rounding_mode,
        )
        .0,
      )
      .div(&Number::from(2_i64), payload.config)
      .unwrap();

      Ok(Value::Number(
        pi_div_2.sub(&argument.atan(payload.config), payload.config),
      ))
    }
  }
}
