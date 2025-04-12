use {
  crate::{
    arguments::Arguments,
    ast::{Ast, BinaryOp, UnaryOp},
    error::Error,
    eval::eval,
    parser::parse,
    value::Value,
  },
  ariadne::{Color, Label, Report, ReportKind, Source},
  chumsky::prelude::*,
  clap::Parser as Clap,
  std::{
    fmt::{Display, Formatter},
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
    process,
  },
};

mod arguments;
mod ast;
mod error;
mod eval;
mod parser;
mod value;

type Span = SimpleSpan<usize>;
type Spanned<T> = (T, Span);

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
