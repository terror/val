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

    let environment = Environment::new();

    match parse(content.trim()) {
      Ok(ast) => match eval(&ast, &environment) {
        Ok(value) => {
          println!("{value}");
          Ok(())
        }
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

      let environment = Environment::new();

      match parse(input) {
        Ok(ast) => match eval(&ast, &environment) {
          Ok(value) => println!("{}", value),
          Err(error) => error
            .report("<input>")
            .eprint(("<input>", Source::from(input)))?,
        },
        Err(errors) => {
          for error in errors {
            error
              .report("<input>")
              .eprint(("<input>", Source::from(input)))?;
          }
        }
      }
    }

    Ok(())
  }
}
