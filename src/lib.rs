use {
  crate::{builtins::BUILTINS, consts::with_consts},
  ariadne::{Color, Label, Report, ReportKind},
  astro_float::{BigFloat as Float, Consts, Radix, Sign},
  chumsky::prelude::*,
  std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    ops::Range,
    process,
    str::FromStr,
  },
};

pub use crate::{
  ast::{AssignmentTarget, BinaryOp, Expression, Program, Statement, UnaryOp},
  builtin::{Builtin, BuiltinFunction, BuiltinFunctionPayload},
  completion::Completion,
  config::Config,
  environment::Environment,
  error::Error,
  evaluator::Evaluator,
  float_ext::FloatExt,
  function::Function,
  parser::parse,
  rounding_mode::RoundingMode,
  value::Value,
};

pub type Span = SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

mod ast;
mod builtin;
mod builtins;
mod completion;
mod config;
mod consts;
mod context;
mod environment;
mod error;
mod evaluator;
mod float_ext;
mod function;
mod parser;
mod rounding_mode;
mod value;
