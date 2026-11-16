use super::*;

builtin! {
  Print {
    name: "print",
    arity: BuiltinArity::Any,
    call(payload) {
      payload
        .environment
        .output
        .borrow_mut()
        .write_all(payload.format_arguments().as_bytes())
        .map_err(|error| Error::new(payload.span, error.to_string()))?;

      Ok(Value::Null)
    }
  }
}
