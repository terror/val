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

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;
type Span = SimpleSpan<usize>;
type Spanned<T> = (T, Span);

#[doc(hidden)]
pub mod arguments;

mod ast;
mod environment;
mod error;
mod evaluator;
mod parser;
mod value;
