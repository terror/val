use {
  serde::Serialize,
  typeshare::typeshare,
  val::{Ast, Span},
  wasm_bindgen::prelude::*,
};

#[wasm_bindgen(start)]
pub fn start() {
  console_error_panic_hook::set_once();
}

#[derive(Serialize)]
#[typeshare]
struct Error {
  message: String,
  range: Range,
}

#[derive(Serialize)]
#[typeshare]
struct Range {
  start: u32,
  end: u32,
}

impl From<val::Span> for Range {
  fn from(span: val::Span) -> Self {
    let range = span.into_range();

    Range {
      start: range.start as u32,
      end: range.end as u32,
    }
  }
}

impl From<&val::Span> for Range {
  fn from(span: &val::Span) -> Self {
    let range = span.into_range();

    Range {
      start: range.start as u32,
      end: range.end as u32,
    }
  }
}

#[derive(Serialize)]
#[typeshare]
struct AstNode {
  kind: String,
  range: Range,
  children: Vec<AstNode>,
}

fn convert_ast(ast: &Ast, span: &Span) -> AstNode {
  let range = Range::from(span);

  let mut children = Vec::new();

  match ast {
    Ast::Number(_) => AstNode {
      kind: ast.kind(),
      range,
      children,
    },
    Ast::Identifier(_) => AstNode {
      kind: ast.kind(),
      range,
      children,
    },
    Ast::UnaryOp(_, rhs) => {
      children.push(convert_ast(&rhs.0, &rhs.1));

      AstNode {
        kind: ast.kind(),
        range,
        children,
      }
    }
    Ast::BinaryOp(_, lhs, rhs) => {
      children.push(convert_ast(&lhs.0, &lhs.1));
      children.push(convert_ast(&rhs.0, &rhs.1));

      AstNode {
        kind: ast.kind(),
        range,
        children,
      }
    }
    Ast::FunctionCall(_, args) => {
      for (ast, span) in args {
        children.push(convert_ast(ast, span));
      }

      AstNode {
        kind: ast.kind(),
        range,
        children,
      }
    }
  }
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
  match val::parse(input) {
    Ok((ast, span)) => {
      match serde_wasm_bindgen::to_value(&convert_ast(&ast, &span)) {
        Ok(value) => Ok(value),
        Err(error) => Err(JsValue::from_str(&error.to_string())),
      }
    }
    Err(errors) => {
      let errors = errors
        .into_iter()
        .map(|error| Error {
          message: error.message,
          range: Range::from(error.span),
        })
        .collect::<Vec<Error>>();

      match serde_wasm_bindgen::to_value(&errors) {
        Ok(js_value) => Err(js_value),
        Err(error) => Err(JsValue::from_str(&error.to_string())),
      }
    }
  }
}
