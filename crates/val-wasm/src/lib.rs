use {
  crate::{
    ast_node::AstNode,
    error::{ErrorKind, ValError},
    execution::Execution,
    highlight::Highlights,
    output::Output,
    parse_result::ParseResult,
    range::Range,
    range_converter::RangeConverter,
  },
  serde::Serialize,
  std::{
    cell::RefCell,
    io::{self, Cursor, Write},
    rc::Rc,
    str,
  },
  tsify::{Ts, Tsify},
  val::{
    Config, Environment, Evaluation, Evaluator, Source, Span, Spanned, Token,
    Value,
    ast::{AssignmentTarget, Expression, Program, Statement},
  },
  wasm_bindgen::prelude::*,
};

mod ast_node;
mod error;
mod execution;
mod highlight;
mod output;
mod parse_result;
mod range;
mod range_converter;

#[wasm_bindgen(start)]
fn start() {
  val_wasm_libc_shim::link();
  console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn execute(source: &str, input: &str) -> Result<Ts<Execution>, JsError> {
  Ok(Execution::run(source, input).into_ts()?)
}

#[wasm_bindgen]
pub fn highlight(source: &str) -> Result<Ts<Highlights>, JsError> {
  Ok(Highlights::new(source).into_ts()?)
}

#[wasm_bindgen]
pub fn parse(source: &str) -> Result<Ts<ParseResult>, JsError> {
  let result = match val::parse(source) {
    Ok(ast) => ParseResult::Ok {
      ast: AstNode::from_program(&ast),
    },
    Err(errors) => ParseResult::Error {
      errors: errors
        .into_iter()
        .map(|error| ValError {
          kind: ErrorKind::Parser,
          message: error.to_string(),
          range: RangeConverter::convert_span(error.span()),
        })
        .collect(),
    },
  };

  Ok(result.into_ts()?)
}
