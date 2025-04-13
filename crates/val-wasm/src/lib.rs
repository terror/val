use {
  serde::Serialize,
  std::collections::HashMap,
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
  attributes: HashMap<String, String>,
}

fn convert_ast(ast: &Ast, span: &Span) -> AstNode {
  let range = Range::from(span);

  let mut attributes = HashMap::new();
  let mut children = Vec::new();

  match ast {
    Ast::Number(n) => {
      attributes.insert("value".into(), n.to_string());

      AstNode {
        kind: "Number".to_string(),
        range,
        children,
        attributes,
      }
    }
    Ast::Identifier(id) => {
      attributes.insert("name".into(), id.to_string());

      AstNode {
        kind: "Identifier".to_string(),
        range,
        children,
        attributes,
      }
    }
    Ast::UnaryOp(op, rhs) => {
      attributes.insert("operator".to_string(), op.to_string());

      children.push(convert_ast(&rhs.0, &rhs.1));

      AstNode {
        kind: "UnaryOp".to_string(),
        range,
        children,
        attributes,
      }
    }
    Ast::BinaryOp(op, lhs, rhs) => {
      attributes.insert("operator".to_string(), op.to_string());

      children.push(convert_ast(&lhs.0, &lhs.1));
      children.push(convert_ast(&rhs.0, &rhs.1));

      AstNode {
        kind: "BinaryOp".to_string(),
        range,
        children,
        attributes,
      }
    }
    Ast::Call(func, args) => {
      attributes.insert("name".to_string(), func.to_string());

      for (ast, span) in args {
        children.push(convert_ast(ast, span));
      }

      AstNode {
        kind: "Call".to_string(),
        range,
        children,
        attributes,
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
