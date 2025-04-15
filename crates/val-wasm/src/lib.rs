use {
  crate::{ast_node::AstNode, range::Range},
  val::{Expression, Program, Span, Statement},
  wasm_bindgen::prelude::*,
};

mod ast_node;
mod range;

#[wasm_bindgen(start)]
fn start() {
  console_error_panic_hook::set_once();
}

#[wasm_bindgen(getter_with_clone)]
pub struct ParseError {
  pub message: String,
  pub range: Range,
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<AstNode, Vec<ParseError>> {
  match val::parse(input) {
    Ok((ast, span)) => Ok(AstNode::from((&ast, &span))),
    Err(errors) => Err(
      errors
        .into_iter()
        .map(|error| ParseError {
          message: error.message,
          range: Range::from(error.span),
        })
        .collect(),
    ),
  }
}
