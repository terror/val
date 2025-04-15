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
      None => Self::read(),
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

  fn read() -> Result {
    let history = dirs::home_dir().unwrap_or_default().join(".val_history");

    let mut editor = DefaultEditor::new()?;
    editor.load_history(&history).ok();

    loop {
      let line = editor.readline("> ")?;

      editor.add_history_entry(line.as_str())?;
      editor.save_history(&history)?;

      let mut evaluator = Evaluator::new();

      match parse(&line) {
        Ok(ast) => match evaluator.eval(&ast) {
          Ok(value) => println!("{}", value),
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
