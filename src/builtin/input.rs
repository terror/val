use super::*;

builtin! {
  Input {
    name: "input",
    arity: BuiltinArity::Range(0, 1),
    call(payload) {
      use std::io::{self, BufRead, Write};

      if payload.arguments.len() == 1 {
        print!("{}", payload.arguments[0].string(payload.span)?);
        io::stdout().flush().unwrap();
      }

      let stdin = io::stdin();

      let mut input = String::new();

      match stdin.lock().read_line(&mut input) {
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
