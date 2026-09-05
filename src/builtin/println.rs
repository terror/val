use super::*;

builtin! {
  Println {
    name: "println",
    arity: BuiltinArity::Any,
    call(payload) {
      use std::io::Write;

      let mut output_strings = Vec::with_capacity(payload.arguments.len());

      for argument in &payload.arguments {
        output_strings.push(argument.display(payload.config));
      }

      writeln!(std::io::stdout(), "{}", output_strings.join(" "))
        .map_err(|error| Error::new(payload.span, error.to_string()))?;

      Ok(Value::Null)
    }
  }
}
