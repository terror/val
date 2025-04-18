use {
  crate::{
    ast_node::AstNode,
    error::{ErrorKind, ValError},
    range::Range,
  },
  val::{Environment, Evaluator, Expression, Program, Span, Statement},
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
pub fn parse(input: &str) -> Result<AstNode, Vec<ValError>> {
  match val::parse(input) {
    Ok((ast, span)) => Ok(AstNode::from((&ast, &span))),
    Err(errors) => Err(
      errors
        .into_iter()
        .map(|error| ValError {
          kind: ErrorKind::Parser,
          message: error.message,
          range: Range::from(error.span),
        })
        .collect(),
    ),
  }
}

#[wasm_bindgen]
pub fn eval(input: &str) -> Result<String, Vec<ValError>> {
  match val::parse(input) {
    Ok(ast) => {
      let mut evaluator =
        Evaluator::from(Environment::new(val::Config::default()));

      match evaluator.eval(&ast) {
        Ok(value) => Ok(value.to_string()),
        Err(error) => Err(vec![ValError {
          kind: ErrorKind::Evaluator,
          message: error.message,
          range: Range::from(error.span),
        }]),
      }
    }
    Err(errors) => Err(
      errors
        .into_iter()
        .map(|error| ValError {
          kind: ErrorKind::Parser,
          message: error.message,
          range: Range::from(error.span),
        })
        .collect(),
    ),
  }
}
