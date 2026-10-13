use {
  crate::{
    ast_node::AstNode,
    error::{ErrorKind, ValError},
    execution::Execution,
    range::Range,
    range_converter::RangeConverter,
    runtime::Runtime,
  },
  serde::Serialize,
  serde_wasm_bindgen::to_value,
  std::{
    cell::RefCell,
    io::{BufRead, Cursor},
    num::NonZeroUsize,
  },
  val::{
    BuiltinArity, BuiltinFunction, BuiltinFunctionPayload, Config, Environment,
    Evaluation, Evaluator, Function, RoundingMode, Span, Spanned, Value,
    ast::{AssignmentTarget, Expression, Program, Statement},
  },
  wasm_bindgen::prelude::*,
};

mod ast_node;
mod error;
mod execution;
mod range;
mod range_converter;
mod runtime;

#[wasm_bindgen(start)]
fn start() {
  val_wasm_libc_shim::link();
  console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn execute(source: &str, input: &str) -> JsValue {
  to_value(&Execution::run(source, input)).unwrap()
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
  match val::parse(input) {
    Ok(ast) => {
      let ast = AstNode::from_program(&ast);

      Ok(to_value(&ast).unwrap())
    }
    Err(errors) => Err(
      to_value(
        &errors
          .into_iter()
          .map(|error| ValError {
            kind: ErrorKind::Parser,
            message: error.to_string(),
            range: RangeConverter::convert_span(error.span()),
          })
          .collect::<Vec<ValError>>(),
      )
      .unwrap(),
    ),
  }
}

#[wasm_bindgen]
pub fn evaluate(input: &str) -> Result<JsValue, JsValue> {
  match val::parse(input) {
    Ok(ast) => {
      let mut evaluator = Evaluator::from(Environment::new(val::Config {
        digits: NonZeroUsize::new(16).unwrap(),
        precision: 53,
        rounding_mode: RoundingMode::FromZero.into(),
      }));

      match evaluator.evaluate(&ast) {
        Ok(Evaluation::Exit { code, span }) => Err(
          to_value(&[ValError {
            kind: ErrorKind::Exit,
            message: format!("exit requested with code {code}"),
            range: RangeConverter::convert_span(Some(&span)),
          }])
          .unwrap(),
        ),
        Ok(Evaluation::Value(value)) => {
          Ok(to_value(&value.to_string()).unwrap())
        }
        Err(error) => Err(
          to_value(&[ValError {
            kind: ErrorKind::Evaluator,
            message: error.to_string(),
            range: RangeConverter::convert_span(error.span()),
          }])
          .unwrap(),
        ),
      }
    }
    Err(errors) => Err(
      to_value(
        &errors
          .into_iter()
          .map(|error| ValError {
            kind: ErrorKind::Parser,
            message: error.to_string(),
            range: RangeConverter::convert_span(error.span()),
          })
          .collect::<Vec<ValError>>(),
      )
      .unwrap(),
    ),
  }
}
