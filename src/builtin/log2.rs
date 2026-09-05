use super::*;

builtin! {
  Log2 {
    name: "log2",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let number = payload.logarithm_argument(0)?;

      Ok(Value::Number(number.log2(payload.config)))
    }
  }
}
