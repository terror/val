use super::*;

builtin! {
  ConstantE {
    name: "e",
    constant(config) {
      Number::e(config)
    }
  }
}
