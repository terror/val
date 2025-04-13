pub(crate) use {
  ariadne::{Color, Label, Report, ReportKind, Source},
  chumsky::prelude::*,
  clap::Parser as Clap,
  environment::Environment,
  std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    fs,
    io::{self, BufRead, Write},
    ops::Range,
    path::PathBuf,
    process,
  },
};

pub use crate::{
  ast::{Ast, BinaryOp, UnaryOp},
  error::Error,
  evaluator::eval,
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
