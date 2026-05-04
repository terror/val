mod consts;

pub(crate) use {
  crate::consts::with_consts,
  ariadne::{Color, Label, Report, ReportKind, Source},
  astro_float::{BigFloat as Float, Consts, Radix, Sign},
  chumsky::prelude::*,
  clap::Parser as Clap,
  std::{
    cell::RefCell,
    collections::HashMap,
    f64,
    fmt::{self, Display, Formatter},
    fs,
    ops::Range,
    path::PathBuf,
    process,
  },
};

#[cfg(not(target_family = "wasm"))]
pub(crate) use {
  crate::highlighter::Highlighter,
  regex::Regex,
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
  std::borrow::Cow::{self, Owned},
};

pub use crate::{
  arguments::Arguments,
  ast::{BinaryOp, Expression, Program, Statement, UnaryOp},
  config::Config,
  environment::Environment,
  error::Error,
  eval_result::EvalResult,
  evaluator::Evaluator,
  float_ext::FloatExt,
  function::{BuiltinFunction, BuiltinFunctionPayload, Function},
  parser::parse,
  rounding_mode::RoundingMode,
  value::Value,
};

pub type Span = SimpleSpan<usize>;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
type Spanned<T> = (T, Span);

#[doc(hidden)]
pub mod arguments;

#[cfg(not(target_family = "wasm"))]
mod highlighter;

mod ast;
mod config;
mod environment;
mod error;
mod eval_result;
mod evaluator;
mod float_ext;
mod function;
mod parser;
mod rounding_mode;
mod value;
