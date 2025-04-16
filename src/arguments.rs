use super::*;
#[derive(Clap)]
#[clap(author, version)]
pub struct Arguments {
  filename: Option<PathBuf>,
}
impl Arguments {
  pub fn run(self) -> Result {
    match self.filename {
      Some(filename) => Self::eval(filename),
      #[cfg(not(target_family = "wasm"))]
      None => Self::read(),
      #[cfg(target_family = "wasm")]
      None => Err(anyhow::anyhow!("Interactive mode not supported in WASM")),
    }
  }
  fn eval(filename: PathBuf) -> Result {
    let content = fs::read_to_string(&filename)?;

    let filename = filename.to_string_lossy().to_string();

    let mut evaluator = Evaluator::new();

    match parse(&content) {
      Ok(ast) => {
        let mut analyzer = Analyzer::new();

        let analysis_errors = analyzer.analyze(&ast);

        if !analysis_errors.is_empty() {
          for error in analysis_errors {
            error
              .report(&filename)
              .eprint((filename.as_str(), Source::from(&content)))?;
          }

          process::exit(1);
        }

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
        Ok(ast) => {
          let mut analyzer = Analyzer::new();

          let analysis_errors = analyzer.analyze(&ast);

          if !analysis_errors.is_empty() {
            for error in analysis_errors {
              error
                .report("<input>")
                .eprint(("<input>", Source::from(&line)))?;
            }

            continue;
          }

          match evaluator.eval(&ast) {
            Ok(value) => {
              if let Value::Null = value {
                continue;
              }

              println!("{}", value);
            }
            Err(error) => error
              .report("<input>")
              .eprint(("<input>", Source::from(&line)))?,
          }
        }
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
