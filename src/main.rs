use {
  arguments::Arguments,
  ariadne::Source,
  clap::Parser,
  highlight_kind::HighlightKind,
  highlight_span::HighlightSpan,
  highlighter::Highlighter,
  prompt::Prompt,
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
    env,
    fmt::{self, Display, Formatter},
    fs,
    io::{self, IsTerminal},
    num::{NonZeroU32, NonZeroUsize},
    path::PathBuf,
    process,
    str::FromStr,
    thread,
  },
  val::{Config, Environment, Evaluator, Value, parse},
};

mod arguments;
mod highlight_kind;
mod highlight_span;
mod highlighter;
mod prompt;
mod rounding_mode;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  let arguments = Arguments::parse();

  let stack_size = arguments.stack_size.get().checked_mul(1024 * 1024);

  let result = match stack_size {
    Some(stack_size) => thread::Builder::new()
      .stack_size(stack_size)
      .spawn(move || arguments.run())
      .map_err(anyhow::Error::from)
      .and_then(|thread| {
        thread
          .join()
          .unwrap_or_else(|_| Err(anyhow::anyhow!("Thread panicked")))
      }),
    None => Err(anyhow::anyhow!("Stack size is too large")),
  };

  if let Err(error) = result {
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

fn color_enabled(terminal: bool) -> bool {
  color_enabled_with_no_color(terminal, no_color())
}

fn color_enabled_with_no_color(terminal: bool, no_color: bool) -> bool {
  terminal && !no_color
}

fn no_color() -> bool {
  env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn no_color() {
    assert!(color_enabled_with_no_color(true, false));
    assert!(!color_enabled_with_no_color(false, false));
    assert!(!color_enabled_with_no_color(true, true));
  }
}
