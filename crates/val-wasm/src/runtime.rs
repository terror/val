use super::*;

thread_local! {
  static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::default());
}

#[derive(Default)]
pub(crate) struct Runtime {
  input: Cursor<Vec<u8>>,
  output: String,
}

impl Runtime {
  pub(crate) fn environment(config: Config, input: &str) -> Environment {
    RUNTIME.set(Self {
      input: Cursor::new(input.as_bytes().to_vec()),
      output: String::new(),
    });

    let environment = Environment::new(config);

    for builtin in [
      BuiltinFunction {
        arity: BuiltinArity::Range(0, 1),
        function: Self::input,
        name: "input",
      },
      BuiltinFunction {
        arity: BuiltinArity::Any,
        function: Self::print,
        name: "print",
      },
      BuiltinFunction {
        arity: BuiltinArity::Any,
        function: Self::print,
        name: "println",
      },
    ] {
      environment
        .add_symbol(builtin.name, Value::Function(Function::Builtin(builtin)));
    }

    environment
  }

  fn input(payload: BuiltinFunctionPayload<'_>) -> val::Result<Value> {
    if let Some(prompt) = payload.arguments.first() {
      let Value::String(prompt) = prompt else {
        return Err(val::Error::new(
          payload.span,
          format!("'{prompt}' is not a string"),
        ));
      };

      Self::write(prompt, payload.span)?;
    }

    let mut input = String::new();

    RUNTIME
      .with_borrow_mut(|runtime| runtime.input.read_line(&mut input))
      .map_err(|error| {
        val::Error::new(payload.span, format!("Failed to read input: {error}"))
      })?;

    if input.ends_with('\n') {
      input.pop();

      if input.ends_with('\r') {
        input.pop();
      }
    }

    Ok(Value::String(input))
  }

  fn print(payload: BuiltinFunctionPayload<'_>) -> val::Result<Value> {
    let output = payload
      .arguments
      .iter()
      .map(|argument| argument.display(payload.config))
      .collect::<Vec<_>>()
      .join(" ");

    let output = if payload.name == "println" {
      format!("{output}\n")
    } else {
      output
    };

    Self::write(&output, payload.span)?;

    Ok(Value::Null)
  }

  pub(crate) fn take_output() -> String {
    RUNTIME.take().output
  }

  fn write(output: &str, span: &Span) -> val::Result<()> {
    RUNTIME.with_borrow_mut(|runtime| {
      if runtime.output.len().saturating_add(output.len()) > 1024 * 1024 {
        return Err(val::Error::new(span, "Output exceeds 1 MiB"));
      }

      runtime.output.push_str(output);

      Ok(())
    })
  }
}
