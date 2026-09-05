use super::*;

builtin! {
  ConstantPhi {
    name: "phi",
    constant(config) {
      Number::from(1_i64)
        .add(&Number::from(5_i64).sqrt(config), config)
        .div(&Number::from(2_i64), config)
        .unwrap()
    }
  }
}
