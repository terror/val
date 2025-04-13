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

impl From<Span> for Range {
  fn from(span: val::Span) -> Self {
    let range = span.into_range();

    Range {
      start: range.start as u32,
      end: range.end as u32,
    }
  }
}

impl From<&Span> for Range {
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

impl From<(&Ast<'_>, &Span)> for AstNode {
  fn from(value: (&Ast<'_>, &Span)) -> Self {
    let (ast, span) = value;

    let range = Range::from(span);

    let mut children = Vec::new();

    match ast {
      Ast::Number(_) => Self {
        kind: ast.kind(),
        range,
        children,
      },
      Ast::Identifier(_) => Self {
        kind: ast.kind(),
        range,
        children,
      },
      Ast::UnaryOp(_, rhs) => {
        children.push(Self::from((&rhs.0, &rhs.1)));

        Self {
          kind: ast.kind(),
          range,
          children,
        }
      }
      Ast::BinaryOp(_, lhs, rhs) => {
        children.push(Self::from((&lhs.0, &lhs.1)));
        children.push(Self::from((&rhs.0, &rhs.1)));

        Self {
          kind: ast.kind(),
          range,
          children,
        }
      }
      Ast::FunctionCall(_, args) => {
        for (ast, span) in args {
          children.push(Self::from((ast, span)));
        }

        Self {
          kind: ast.kind(),
          range,
          children,
        }
      }
    }
  }
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<JsValue, JsValue> {
  match val::parse(input) {
    Ok((ast, span)) => {
      match serde_wasm_bindgen::to_value(&AstNode::from((&ast, &span))) {
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
        Ok(value) => Err(value),
        Err(error) => Err(JsValue::from_str(&error.to_string())),
      }
    }
  }
}
