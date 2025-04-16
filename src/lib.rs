pub(crate) use {
  ariadne::{Color, Label, Report, ReportKind, Source},
  chumsky::prelude::*,
  clap::Parser as Clap,
  environment::Environment,
  std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs,
    ops::Range,
    path::PathBuf,
    process,
  },
};

#[cfg(not(target_family = "wasm"))]
pub(crate) use rustyline::DefaultEditor;

pub use crate::{
  ast::{BinaryOp, Expression, Program, Statement, UnaryOp},
  error::Error,
  eval_result::EvalResult,
  evaluator::Evaluator,
  function::{BuiltinFunction, Function},
  parser::parse,
  value::Value,
};

pub type Span = SimpleSpan<usize>;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
type Spanned<T> = (T, Span);

#[doc(hidden)]
pub mod arguments;

mod ast;
mod environment;
mod error;
mod eval_result;
mod evaluator;
mod function;
mod parser;
mod value;
