use super::*;

builtin! {
  Println {
    name: "println",
    arity: BuiltinArity::Any,
    call(payload) {
      use std::io::Write;

      writeln!(std::io::stdout(), "{}", payload.format_arguments())
        .map_err(|error| Error::new(payload.span, error.to_string()))?;

      Ok(Value::Null)
    }
  }
}
