use {
  crate::{
    ast_node::AstNode,
    error::{ErrorKind, ValError},
    range::Range,
  },
  serde::Serialize,
  serde_wasm_bindgen::to_value,
  val::{
    Environment, Evaluator, Expression, Program, RoundingMode, Span, Statement,
  },
  wasm_bindgen::prelude::*,
};

mod ast_node;
mod error;
mod range;

#[wasm_bindgen(start)]
fn start() {
  console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
  match val::parse(input) {
    Ok((ast, span)) => Ok(to_value(&AstNode::from((&ast, &span))).unwrap()),
    Err(errors) => Err(
      to_value(
        &errors
          .into_iter()
          .map(|error| ValError {
            kind: ErrorKind::Parser,
            message: error.message,
            range: Range::from(error.span),
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
        precision: 53,
        rounding_mode: RoundingMode::FromZero.into(),
      }));

      match evaluator.eval(&ast) {
        Ok(value) => Ok(to_value(&value.to_string()).unwrap()),
        Err(error) => Err(
          to_value(&[ValError {
            kind: ErrorKind::Evaluator,
            message: error.message,
            range: Range::from(error.span),
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
            range: Range::from(error.span),
          })
          .collect::<Vec<ValError>>(),
      )
      .unwrap(),
    ),
  }
}
