use {
  serde::Serialize,
  typeshare::typeshare,
  val::{Expression, Program, Span, Statement},
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

impl AstNode {
  fn from_expression(expr: &Expression<'_>, span: &Span) -> Self {
    let range = Range::from(span);

    let mut children = Vec::new();

    match expr {
      Expression::BinaryOp(_, lhs, rhs) => {
        children.push(Self::from_expression(&lhs.0, &lhs.1));
        children.push(Self::from_expression(&rhs.0, &rhs.1));

        Self {
          kind: expr.kind(),
          range,
          children,
        }
      }
      Expression::Boolean(_) => Self {
        kind: expr.kind(),
        range,
        children,
      },
      Expression::FunctionCall(_, arguments) => {
        for (ast, span) in arguments {
          children.push(Self::from_expression(ast, span));
        }

        Self {
          kind: expr.kind(),
          range,
          children,
        }
      }
      Expression::Identifier(_) => Self {
        kind: expr.kind(),
        range,
        children,
      },
      Expression::List(items) => {
        for (item, span) in items {
          children.push(Self::from_expression(item, span));
        }

        Self {
          kind: expr.kind(),
          range,
          children,
        }
      }
      Expression::ListAccess(list, index) => {
        children.push(Self::from_expression(&list.0, &list.1));
        children.push(Self::from_expression(&index.0, &index.1));

        Self {
          kind: expr.kind(),
          range,
          children,
        }
      }
      Expression::Number(_) => Self {
        kind: expr.kind(),
        range,
        children,
      },
      Expression::String(_) => Self {
        kind: expr.kind(),
        range,
        children,
      },
      Expression::UnaryOp(_, rhs) => {
        children.push(Self::from_expression(&rhs.0, &rhs.1));

        Self {
          kind: expr.kind(),
          range,
          children,
        }
      }
    }
  }

  fn from_statement(statement: &Statement<'_>, span: &Span) -> Self {
    let range = Range::from(span);

    let mut children = Vec::new();

    match statement {
      Statement::Assignment(_, rhs) => {
        children.push(Self::from_expression(&rhs.0, &rhs.1));

        Self {
          kind: statement.kind(),
          range,
          children,
        }
      }
      Statement::Block(statements) => {
        for (statement, span) in statements {
          children.push(Self::from_statement(statement, span));
        }

        Self {
          kind: statement.kind(),
          range,
          children,
        }
      }
      Statement::Expression(expression) => {
        children.push(Self::from_expression(&expression.0, &expression.1));

        Self {
          kind: statement.kind(),
          range,
          children,
        }
      }
      Statement::While(condition, body) => {
        children.push(Self::from_expression(&condition.0, &condition.1));

        for (statement, span) in body {
          children.push(Self::from_statement(statement, span));
        }

        Self {
          kind: statement.kind(),
          range,
          children,
        }
      }
    }
  }

  fn from_program(program: &Program<'_>, span: &Span) -> Self {
    let range = Range::from(span);

    let mut children = Vec::new();

    match program {
      Program::Statements(statements) => {
        for (statement, span) in statements {
          children.push(Self::from_statement(statement, span));
        }

        Self {
          kind: program.kind(),
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
      match serde_wasm_bindgen::to_value(&AstNode::from_program(&ast, &span)) {
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
