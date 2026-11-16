use super::*;

builtin! {
  Input {
    name: "input",
    arity: BuiltinArity::Range(0, 1),
    call(payload) {
      if payload.arguments.len() == 1 {
        let mut output = payload.environment.output.borrow_mut();

        output
          .write_all(payload.arguments[0].string(payload.span)?.as_bytes())
          .and_then(|()| output.flush())
          .map_err(|error| Error::new(payload.span, error.to_string()))?;
      }

      let mut input = String::new();

      match payload.environment.input.borrow_mut().read_line(&mut input) {
        Ok(_) => {
          if input.ends_with('\n') {
            input.pop();

            if input.ends_with('\r') {
              input.pop();
            }
          }

          Ok(Value::String(input))
        }
        Err(e) => Err(Error::new(
          payload.span,
          format!("Failed to read input: {e}"),
        )),
      }
    }
  }
}
