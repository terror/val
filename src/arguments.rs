use super::*;

#[derive(Clap, Debug)]
#[clap(author, version)]
pub struct Arguments {
  #[clap(conflicts_with = "expression")]
  filename: Option<PathBuf>,
  #[clap(short, long, conflicts_with = "filename")]
  expression: Option<String>,
}

impl Arguments {
  pub fn run(self) -> Result {
    match (self.filename, self.expression) {
      (Some(filename), _) => Self::eval(filename),
      (_, Some(expression)) => Self::eval_expression(expression),
      _ => {
        #[cfg(not(target_family = "wasm"))]
        {
          Self::read()
        }
        #[cfg(target_family = "wasm")]
        {
          Err(anyhow::anyhow!("Interactive mode not supported in WASM"))
        }
      }
    }
  }

  fn eval(filename: PathBuf) -> Result {
    let content = fs::read_to_string(&filename)?;

    let filename = filename.to_string_lossy().to_string();

    let mut evaluator = Evaluator::new();

    match parse(&content) {
      Ok(ast) => match evaluator.eval(&ast) {
        Ok(_) => Ok(()),
        Err(error) => {
          error
            .report(&filename)
            .eprint((filename.as_str(), Source::from(content)))?;

          process::exit(1);
        }
      },
      Err(errors) => {
        for error in errors {
          error
            .report(&filename)
            .eprint((filename.as_str(), Source::from(&content)))?;
        }

        process::exit(1);
      }
    }
  }

  fn eval_expression(expr: String) -> Result {
    let mut evaluator = Evaluator::new();

    match parse(&expr) {
      Ok(ast) => match evaluator.eval(&ast) {
        Ok(value) => {
          if let Value::Null = value {
            return Ok(());
          }

          println!("{}", value);

          Ok(())
        }
        Err(error) => {
          error
            .report("<expression>")
            .eprint(("<expression>", Source::from(expr)))?;

          process::exit(1);
        }
      },
      Err(errors) => {
        for error in errors {
          error
            .report("<expression>")
            .eprint(("<expression>", Source::from(&expr)))?;
        }

        process::exit(1);
      }
    }
  }

  #[cfg(not(target_family = "wasm"))]
  fn read() -> Result {
    let history = dirs::home_dir().unwrap_or_default().join(".val_history");

    let config = Builder::new()
      .color_mode(ColorMode::Enabled)
      .edit_mode(EditMode::Emacs)
      .history_ignore_space(true)
      .completion_type(CompletionType::Circular)
      .max_history_size(1000)?
      .build();

    let mut editor =
      Editor::<Highlighter, DefaultHistory>::with_config(config)?;

    editor.set_helper(Some(Highlighter::new()));

    editor.load_history(&history).ok();

    loop {
      let line = editor.readline("> ")?;

      editor.add_history_entry(line.as_str())?;
      editor.save_history(&history)?;

      let mut evaluator = Evaluator::new();

      match parse(&line) {
        Ok(ast) => match evaluator.eval(&ast) {
          Ok(value) => {
            if let Value::Null = value {
              continue;
            }

            println!("{}", value);
          }
          Err(error) => error
            .report("<input>")
            .eprint(("<input>", Source::from(&line)))?,
        },
        Err(errors) => {
          for error in errors {
            error
              .report("<input>")
              .eprint(("<input>", Source::from(&line)))?;
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use {super::*, clap::Parser, std::path::PathBuf};

  #[test]
  fn filename_only() {
    let arguments = Arguments::parse_from(vec!["program", "file.txt"]);

    assert!(arguments.filename.is_some());
    assert!(arguments.expression.is_none());

    assert_eq!(arguments.filename.unwrap(), PathBuf::from("file.txt"));
  }

  #[test]
  fn expression_only() {
    let arguments =
      Arguments::parse_from(vec!["program", "--expression", "1 + 2"]);

    assert!(arguments.filename.is_none());
    assert!(arguments.expression.is_some());

    assert_eq!(arguments.expression.unwrap(), "1 + 2");
  }

  #[test]
  fn expression_short_form() {
    let arguments = Arguments::parse_from(vec!["program", "-e", "1 + 2"]);

    assert!(arguments.filename.is_none());
    assert!(arguments.expression.is_some());

    assert_eq!(arguments.expression.unwrap(), "1 + 2");
  }

  #[test]
  fn both_should_fail() {
    assert!(
      Arguments::try_parse_from(vec![
        "program",
        "file.txt",
        "--expression",
        "1 + 2"
      ])
      .is_err()
    );
  }

  #[test]
  fn neither_provided() {
    let arguments = Arguments::parse_from(vec!["program"]);

    assert!(arguments.filename.is_none());
    assert!(arguments.expression.is_none());
  }

  #[test]
  fn conflict_error_message() {
    let result = Arguments::try_parse_from(vec![
      "program",
      "file.txt",
      "--expression",
      "1 + 2",
    ]);

    assert!(result.is_err());

    let error = result.unwrap_err().to_string();

    assert!(
      error.contains("cannot be used with"),
      "Error should mention conflicts: {}",
      error
    );
  }
}
