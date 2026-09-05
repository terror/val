use super::*;

builtin! {
  ConstantTau {
    name: "tau",
    constant(config) {
      Number::tau(config)
    }
  }
}
