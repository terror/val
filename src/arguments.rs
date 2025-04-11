use super::*;

#[derive(Clap)]
#[clap(author, version)]
pub(crate) struct Arguments {
  pub(crate) filename: Option<PathBuf>,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    match self.filename {
      Some(filename) => Self::eval_file(filename),
      None => Self::start_repl(),
    }
  }

  fn eval_file(filename: PathBuf) -> Result {
    let content = fs::read_to_string(&filename)?;

    let filename = filename.to_string_lossy().to_string();

    let result = parser().parse(content.trim());

    match result.into_output_errors() {
      (Some(ast), errors) if errors.is_empty() => match eval(&ast) {
        Ok(value) => {
          println!("{value}");
          Ok(())
        }
        Err(error) => error.report(&filename, &content),
      },
      (_, errors) => report_parse_errors(&filename, &content, &errors),
    }
  }

  fn start_repl() -> Result {
    loop {
      let mut buffer = String::new();

      print!("> ");

      io::stdout().flush()?;

      if io::stdin().lock().read_line(&mut buffer)? == 0 {
        break;
      }

      let input = buffer.trim();

      if input.is_empty() {
        continue;
      }

      let result = parser().parse(input);

      match result.into_output_errors() {
        (Some(ast), errors) if errors.is_empty() => match eval(&ast) {
          Ok(value) => println!("{}", value),
          Err(error) => error.report("<input>", input)?,
        },
        (_, errors) => {
          report_parse_errors("<input>", input, &errors)?;
        }
      }
    }

    Ok(())
  }
}
