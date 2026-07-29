use {
  crate::{
    ast_node::AstNode,
    error::{ErrorKind, ValError},
    range::Range,
    range_converter::RangeConverter,
  },
  serde::Serialize,
  serde_wasm_bindgen::to_value,
  std::num::NonZeroUsize,
  val::{
    Environment, Evaluator, RoundingMode, Span,
    ast::{AssignmentTarget, Expression, Program, Statement},
  },
  wasm_bindgen::prelude::*,
};

mod ast_node;
mod error;
mod range;
mod range_converter;

#[wasm_bindgen(start)]
fn start() {
  val_wasm_libc_shim::link();
  console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
  let converter = RangeConverter::new(input);

  match val::parse(input) {
    Ok((ast, span)) => {
      let mut ast = AstNode::from((&ast, &span));

      ast.convert_ranges(&converter);

      Ok(to_value(&ast).unwrap())
    }
    Err(errors) => Err(
      to_value(
        &errors
          .into_iter()
          .map(|error| ValError {
            kind: ErrorKind::Parser,
            message: error.message,
            range: converter.convert(Range::from(error.span)),
          })
          .collect::<Vec<ValError>>(),
      )
      .unwrap(),
    ),
  }
}

#[wasm_bindgen]
pub fn evaluate(input: &str) -> Result<JsValue, JsValue> {
  let converter = RangeConverter::new(input);

  match val::parse(input) {
    Ok(ast) => {
      let mut evaluator = Evaluator::from(Environment::new(val::Config {
        digits: NonZeroUsize::new(16).unwrap(),
        precision: 53,
        rounding_mode: RoundingMode::FromZero.into(),
      }));

      match evaluator.evaluate(&ast) {
        Ok(value) => Ok(to_value(&value.to_string()).unwrap()),
        Err(error) => Err(
          to_value(&[ValError {
            kind: ErrorKind::Evaluator,
            message: error.message,
            range: converter.convert(Range::from(error.span)),
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
            message: error.message,
            range: converter.convert(Range::from(error.span)),
          })
          .collect::<Vec<ValError>>(),
      )
      .unwrap(),
    ),
  }
}
