use super::*;

builtin! {
  ConstantPi {
    name: "pi",
    constant(config) {
      Number::Approx(
        Float::with_val_round(
          config.precision(),
          Constant::Pi,
          config.rounding_mode,
        )
        .0,
      )
    }
  }
}
