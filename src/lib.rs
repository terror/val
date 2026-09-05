use {
  ariadne::{Color, IndexType, Label, Report, ReportKind},
  ast::{AssignmentTarget, BinaryOp, Expression, Program, Statement, UnaryOp},
  builtins::BUILTINS,
  chumsky::prelude::*,
  context::Context,
  decimal::Decimal,
  frame::Frame,
  rug::{
    Complete, Float, Integer, Rational,
    float::{Constant, Round},
    integer::MiniInteger,
    ops::{Pow, PowAssignRound},
  },
  std::{
    borrow::Cow,
    cell::RefCell,
    cmp::Ordering,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    num::NonZeroUsize,
    ops::Range,
    rc::Rc,
    str::FromStr,
  },
  symbol::Symbol,
};

pub use crate::{
  builtin::Builtin, builtin_arity::BuiltinArity,
  builtin_function::BuiltinFunction,
  builtin_function_payload::BuiltinFunctionPayload, completion::Completion,
  config::Config, environment::Environment, error::Error,
  evaluation::Evaluation, evaluator::Evaluator, function::Function,
  number::Number, parser::parse, rounding_mode::RoundingMode, value::Value,
};

pub type Span = SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

pub mod ast;
mod builtin;
mod builtin_arity;
mod builtin_function;
mod builtin_function_payload;
mod builtins;
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
mod symbol;
mod value;
