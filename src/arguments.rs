use super::*;

#[derive(Clap, Debug)]
#[clap(
  about,
  author,
  version,
  help_template = "\
{before-help}{name} {version}
{author}
{about}

\x1b[1;4mUsage\x1b[0m: {usage}

{all-args}{after-help}
"
)]
pub struct Arguments {
  #[clap(
    short,
    long,
    conflicts_with = "filename",
    help = "Expression to evaluate"
  )]
  expression: Option<String>,

  #[clap(conflicts_with = "expression", help = "File to evaluate")]
  filename: Option<PathBuf>,

  #[clap(
    short,
    long,
    conflicts_with = "filename",
    help = "Load files before entering the REPL"
  )]
  load: Option<Vec<PathBuf>>,

  #[clap(
    short,
    long,
    default_value = "1024",
    help = "Decimal precision to use for calculations"
  )]
  precision: usize,

  #[clap(
    short,
    long,
    value_parser = clap::value_parser!(RoundingMode),
    default_value = "to-even",
    help = "Rounding mode to use for calculations",
  )]
  rounding_mode: RoundingMode,

  #[clap(
    long,
    default_value = "128",
    help = "Stack size in MB for evaluations"
  )]
  pub stack_size: usize,
}
impl Arguments {
  pub fn run(self) -> Result {
    match (&self.filename, &self.expression) {
      (Some(filename), _) => self.eval(filename.clone()),
      (_, Some(expression)) => self.eval_expression(expression.clone()),
      _ => {
        #[cfg(not(target_family = "wasm"))]
        {
          self.read()
        }
        #[cfg(target_family = "wasm")]
        {
          Err(anyhow::anyhow!("Interactive mode not supported in WASM"))
        }
      }
    }
  }

  fn eval(&self, filename: PathBuf) -> Result {
    let content = fs::read_to_string(&filename)?;

    let filename = filename.to_string_lossy().to_string();

    match parse(&content) {
      Ok(ast) => {
        let environment = Environment::new(Config {
          precision: self.precision,
          rounding_mode: self.rounding_mode.into(),
        });

        let mut analyzer = Analyzer::new(environment.clone());

        let analysis_errors = analyzer.analyze(&ast);

        if !analysis_errors.is_empty() {
          for error in analysis_errors {
            error
              .report(&filename)
              .eprint((filename.as_str(), Source::from(&content)))?;
          }

          process::exit(1);
        }

        let mut evaluator = Evaluator::from(environment);

        match evaluator.eval(&ast) {
          Ok(_) => Ok(()),
          Err(error) => {
            error
              .report(&filename)
              .eprint((filename.as_str(), Source::from(content)))?;

            process::exit(1);
          }
        }
      }
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

  fn eval_expression(&self, value: String) -> Result {
    let mut evaluator = Evaluator::from(Environment::new(Config {
      precision: self.precision,
      rounding_mode: self.rounding_mode.into(),
    }));

    match parse(&value) {
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
            .eprint(("<expression>", Source::from(value)))?;

          process::exit(1);
        }
      },
      Err(errors) => {
        for error in errors {
          error
            .report("<expression>")
            .eprint(("<expression>", Source::from(&value)))?;
        }

        process::exit(1);
      }
    }
  }

  #[cfg(not(target_family = "wasm"))]
  fn read(&self) -> Result {
    let history = dirs::home_dir().unwrap_or_default().join(".val_history");

    let editor_config = Builder::new()
      .color_mode(ColorMode::Enabled)
      .edit_mode(EditMode::Emacs)
      .history_ignore_space(true)
      .completion_type(CompletionType::Circular)
      .max_history_size(1000)?
      .build();

    let mut editor =
      Editor::<Highlighter, DefaultHistory>::with_config(editor_config)?;

    editor.set_helper(Some(Highlighter::new()));
    editor.load_history(&history).ok();

    let mut evaluator = Evaluator::from(Environment::new(Config {
      precision: self.precision,
      rounding_mode: self.rounding_mode.into(),
    }));

    if let Some(filenames) = &self.load {
      for filename in filenames {
        let content: &'static str =
          Box::leak(fs::read_to_string(filename)?.into_boxed_str());

        let filename = filename.to_string_lossy().to_string();

        match parse(content) {
          Ok(ast) => match evaluator.eval(&ast) {
            Ok(_) => {}
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
    }

    loop {
      let line = editor.readline("> ")?;

      editor.add_history_entry(&line)?;
      editor.save_history(&history)?;

      let line: &'static str = Box::leak(line.into_boxed_str());

      match parse(line) {
        Ok(ast) => match evaluator.eval(&ast) {
          Ok(value) if !matches!(value, Value::Null) => println!("{value}"),
          Ok(_) => {}
          Err(error) => error
            .report("<input>")
            .eprint(("<input>", Source::from(line)))?,
        },
        Err(errors) => {
          for error in errors {
            error
              .report("<input>")
              .eprint(("<input>", Source::from(line)))?;
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

  #[test]
  fn load_conflicts_with_filename() {
    let result = Arguments::try_parse_from(vec![
      "program",
      "file.txt",
      "--load",
      "prelude.val",
    ]);

    assert!(result.is_err(), "Parser should reject filename + --load");

    let error = result.unwrap_err().to_string();

    assert!(
      error.contains("cannot be used with"),
      "Error should mention conflicts: {}",
      error
    );
  }
}
