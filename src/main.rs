use {
  clap::Parser, rustyline::error::ReadlineError, std::process,
  val::arguments::Arguments,
};

fn main() {
  if let Err(error) = Arguments::parse().run() {
    if let Some(error) = error.downcast_ref::<ReadlineError>() {
      if matches!(error, ReadlineError::Eof | ReadlineError::Interrupted) {
        return;
      }
    }

    eprintln!("error: {error}");

    process::exit(1);
  }
}
