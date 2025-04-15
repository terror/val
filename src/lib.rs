pub(crate) use {
  ariadne::{Color, Label, Report, ReportKind, Source},
  chumsky::prelude::*,
  clap::Parser as Clap,
  environment::Environment,
  rustyline::DefaultEditor,
  std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs,
    ops::Range,
    path::PathBuf,
    process,
  },
};

pub use crate::{
  ast::{BinaryOp, Expression, Program, Statement, UnaryOp},
  error::Error,
  evaluator::Evaluator,
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
mod evaluator;
mod parser;
mod value;
