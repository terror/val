use super::*;

builtin! {
  Log10 {
    name: "log10",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let number = payload.logarithm_argument(0)?;

      Ok(Value::Number(number.log10(payload.config)))
    }
  }
}
