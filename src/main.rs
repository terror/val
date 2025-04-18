use {
  clap::Parser,
  rustyline::error::ReadlineError,
  std::{backtrace::BacktraceStatus, process, thread},
  val::arguments::Arguments,
};

fn main() {
  let arguments = Arguments::parse();

  let stack_size = arguments.stack_size * 1024 * 1024;

  let result = thread::Builder::new()
    .stack_size(stack_size)
    .spawn(move || arguments.run())
    .unwrap()
    .join();

  if let Err(error) = result.unwrap_or_else(|_| Err(anyhow::anyhow!("Thread panicked"))) {
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
