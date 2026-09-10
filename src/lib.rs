use {
  ariadne::{Color, IndexType, Label, Report, ReportKind},
  ast::{AssignmentTarget, BinaryOp, Expression, Program, Statement, UnaryOp},
  chumsky::prelude::*,
  completion::Completion,
  context::Context,
  decimal::Decimal,
  frame::Frame,
  gc::{Finalize, GcCell, Trace},
  rug::{
    Complete, Float, Integer, Rational,
    float::{Constant, Round},
    integer::MiniInteger,
    ops::{AssignRound, Pow, PowAssignRound},
  },
  std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{self, Debug, Display, Formatter},
    iter::once,
    num::NonZeroUsize,
    ops::Range,
    str::FromStr,
  },
};

pub use crate::{
  builtin::Builtin, builtin_arity::BuiltinArity,
  builtin_function::BuiltinFunction,
  builtin_function_payload::BuiltinFunctionPayload, config::Config,
  environment::Environment, error::Error, evaluation::Evaluation,
  evaluator::Evaluator, function::Function, number::Number, parser::parse,
  rounding_mode::RoundingMode, user_function::UserFunction, value::Value,
};

pub use gc::{Gc, force_collect};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
pub type Span = SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

pub mod ast;
mod builtin;
mod builtin_arity;
mod builtin_function;
mod builtin_function_payload;
mod completion;
mod config;
mod context;
mod decimal;
mod environment;
mod error;
mod evaluation;
mod evaluator;
mod frame;
mod function;
mod number;
mod parser;
mod rounding_mode;
mod user_function;
mod value;
