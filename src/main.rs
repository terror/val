use {
  arguments::Arguments,
  ariadne::Source,
  clap::{ColorChoice, CommandFactory, FromArgMatches, Parser},
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
  let arguments = parse_arguments();

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

fn color_enabled(color: ColorChoice, terminal: bool) -> bool {
  color_enabled_with_no_color(color, terminal, no_color())
}

fn color_enabled_with_no_color(
  color: ColorChoice,
  terminal: bool,
  no_color: bool,
) -> bool {
  match color {
    ColorChoice::Auto => terminal && !no_color,
    ColorChoice::Always => true,
    ColorChoice::Never => false,
  }
}

fn no_color() -> bool {
  env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty())
}

fn parse_arguments() -> Arguments {
  let color = requested_color();
  let command = Arguments::command().color(color);

  Arguments::from_arg_matches(&command.get_matches())
    .unwrap_or_else(|error| error.exit())
}

fn requested_color() -> ColorChoice {
  let mut arguments = env::args_os().skip(1);
  let mut color = ColorChoice::Auto;

  while let Some(argument) = arguments.next() {
    if argument == "--" {
      break;
    }

    let value = if argument == "--color" {
      arguments.next()
    } else {
      argument
        .to_str()
        .and_then(|argument| argument.strip_prefix("--color="))
        .map(Into::into)
    };

    if let Some(value) = value.and_then(|value| value.to_str()?.parse().ok()) {
      color = value;
    }
  }

  color
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn color_modes() {
    assert!(color_enabled_with_no_color(ColorChoice::Auto, true, false));
    assert!(!color_enabled_with_no_color(
      ColorChoice::Auto,
      false,
      false
    ));
    assert!(!color_enabled_with_no_color(ColorChoice::Auto, true, true));
    assert!(color_enabled_with_no_color(
      ColorChoice::Always,
      false,
      true
    ));
    assert!(!color_enabled_with_no_color(
      ColorChoice::Never,
      true,
      false
    ));
  }
}
