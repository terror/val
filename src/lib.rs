use {
  ariadne::{Color, Label, Report, ReportKind},
  ast::{AssignmentTarget, BinaryOp, Expression, Program, Statement, UnaryOp},
  builtins::BUILTINS,
  chumsky::prelude::*,
  context::Context,
  decimal::Decimal,
  rug::{
    Complete, Float, Integer, Rational,
    float::{Constant, Round},
    ops::Pow,
  },
  std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    num::NonZeroUsize,
    ops::Range,
    process,
    str::FromStr,
  },
  symbol::Symbol,
};

pub use crate::{
  builtin::{Builtin, BuiltinFunction, BuiltinFunctionPayload},
  completion::Completion,
  config::Config,
  environment::Environment,
  error::Error,
  evaluator::Evaluator,
  function::Function,
  number::Number,
  parser::parse,
  rounding_mode::RoundingMode,
  value::Value,
};

pub type Span = SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

pub mod ast;
mod builtin;
mod builtins;
mod completion;
mod config;
mod context;
mod decimal;
mod environment;
mod error;
mod evaluator;
mod function;
mod number;
mod parser;
mod rounding_mode;
mod symbol;
mod value;
