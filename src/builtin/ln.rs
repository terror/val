use super::*;

builtin! {
  Ln {
    name: "ln",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let number = payload.logarithm_argument(0)?;

      Ok(Value::Number(number.ln(payload.config)))
    }
  }
}
