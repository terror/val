use {
  clap::Parser,
  rustyline::error::ReadlineError,
  std::{backtrace::BacktraceStatus, process},
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

    for (i, error) in error.chain().skip(1).enumerate() {
      if i == 0 {
        eprintln!();
        eprintln!("because:");
      }

      eprintln!("- {error}");
    }

    let backtrace = error.backtrace();

    if backtrace.status() == BacktraceStatus::Captured {
      eprintln!("backtrace:");
      eprintln!("{backtrace}");
    }

    process::exit(1);
  }
}
