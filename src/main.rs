use {
  arguments::Arguments,
  ariadne::Source,
  clap::Parser,
  highlighter::Highlighter,
  regex::Regex,
  rounding_mode::RoundingMode,
  rug::float::Round,
  rustyline::{
    Context, Editor, Helper,
    completion::{Completer, FilenameCompleter, Pair},
    config::{Builder, ColorMode, CompletionType, EditMode},
    error::ReadlineError,
    highlight::{CmdKind, Highlighter as RustylineHighlighter},
    hint::{Hinter, HistoryHinter},
    history::DefaultHistory,
    validate::Validator,
  },
  std::{
    backtrace::BacktraceStatus,
    borrow::{Cow, Cow::Owned},
    fmt::{self, Display, Formatter},
    fs,
    path::PathBuf,
    process,
    str::FromStr,
    thread,
  },
  val::{
    Config, Environment, Evaluator, Spanned, Value,
    ast::{AssignmentTarget, Expression, Program, Statement},
    parse,
  },
};

mod arguments;
mod highlighter;
mod rounding_mode;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  let arguments = Arguments::parse();

  let stack_size = arguments.stack_size * 1024 * 1024;

  let result = thread::Builder::new()
    .stack_size(stack_size)
    .spawn(move || arguments.run())
    .unwrap()
    .join();

  if let Err(error) =
    result.unwrap_or_else(|_| Err(anyhow::anyhow!("Thread panicked")))
  {
    if let Some(&ReadlineError::Eof | &ReadlineError::Interrupted) =
      error.downcast_ref::<ReadlineError>()
    {
      return;
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
