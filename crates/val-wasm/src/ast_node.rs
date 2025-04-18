use super::*;

#[derive(Clone)]
#[wasm_bindgen(getter_with_clone)]
pub struct AstNode {
  pub kind: String,
  pub range: Range,
  pub children: Vec<AstNode>,
}

impl From<(&Program<'_>, &Span)> for AstNode {
  fn from(value: (&Program<'_>, &Span)) -> Self {
    let (program, span) = value;

    let range = Range::from(span);

    let mut children = Vec::new();

    match program {
      Program::Statements(statements) => {
        for (statement, span) in statements {
          children.push(Self::from((statement, span)));
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

impl From<(&Statement<'_>, &Span)> for AstNode {
  fn from(value: (&Statement<'_>, &Span)) -> Self {
    let (statement, span) = value;

    let range = Range::from(span);

    let mut children = Vec::new();

    match statement {
      Statement::Assignment(_, rhs) => {
        children.push(Self::from((&rhs.0, &rhs.1)));

        Self {
          kind: statement.kind(),
          range,
          children,
        }
      }
      Statement::Block(statements) => {
        for (statement, span) in statements {
          children.push(Self::from((statement, span)));
        }

        Self {
          kind: statement.kind(),
          range,
          children,
        }
      }
      Statement::Break => Self {
        kind: statement.kind(),
        range,
        children,
      },
      Statement::Continue => Self {
        kind: statement.kind(),
        range,
        children,
      },
      Statement::Expression(expression) => {
        children.push(Self::from((&expression.0, &expression.1)));

        Self {
          kind: statement.kind(),
          range,
          children,
        }
      }
      Statement::Function(_, _, body) => {
        for (statement, span) in body {
          children.push(Self::from((statement, span)));
        }

        Self {
          kind: statement.kind(),
          range,
          children,
        }
      }
      Statement::If(condition, then_branch, else_branch) => {
        children.push(Self::from((&condition.0, &condition.1)));

        for (statement, span) in then_branch {
          children.push(Self::from((statement, span)));
        }

        if let Some(else_statements) = else_branch {
          for (statement, span) in else_statements {
            children.push(Self::from((statement, span)));
          }
        }

        Self {
          kind: statement.kind(),
          range,
          children,
        }
      }
      Statement::Loop(body) => {
        for (statement, span) in body {
          children.push(Self::from((statement, span)));
        }

        Self {
          kind: statement.kind(),
          range,
          children,
        }
      }
      Statement::Return(expression) => {
        if let Some(expression) = expression {
          children.push(Self::from((&expression.0, &expression.1)));
        }

        Self {
          kind: statement.kind(),
          range,
          children,
        }
      }
      Statement::While(condition, body) => {
        children.push(Self::from((&condition.0, &condition.1)));

        for (statement, span) in body {
          children.push(Self::from((statement, span)));
        }

        Self {
          kind: statement.kind(),
          range,
          children,
        }
      }
    }
  }
}

impl From<(&Expression<'_>, &Span)> for AstNode {
  fn from(value: (&Expression<'_>, &Span)) -> Self {
    let (expression, span) = value;

    let range = Range::from(span);

    let mut children = Vec::new();

    match expression {
      Expression::BinaryOp(_, lhs, rhs) => {
        children.push(Self::from((&lhs.0, &lhs.1)));
        children.push(Self::from((&rhs.0, &rhs.1)));

        Self {
          kind: expression.kind(),
          range,
          children,
        }
      }
      Expression::Boolean(_) => Self {
        kind: expression.kind(),
        range,
        children,
      },
      Expression::FunctionCall(_, arguments) => {
        for (ast, span) in arguments {
          children.push(Self::from((ast, span)));
        }

        Self {
          kind: expression.kind(),
          range,
          children,
        }
      }
      Expression::Identifier(_) => Self {
        kind: expression.kind(),
        range,
        children,
      },
      Expression::List(items) => {
        for (item, span) in items {
          children.push(Self::from((item, span)));
        }

        Self {
          kind: expression.kind(),
          range,
          children,
        }
      }
      Expression::ListAccess(list, index) => {
        children.push(Self::from((&list.0, &list.1)));
        children.push(Self::from((&index.0, &index.1)));

        Self {
          kind: expression.kind(),
          range,
          children,
        }
      }
      Expression::Null => Self {
        kind: expression.kind(),
        range,
        children,
      },
      Expression::Number(_) => Self {
        kind: expression.kind(),
        range,
        children,
      },
      Expression::String(_) => Self {
        kind: expression.kind(),
        range,
        children,
      },
      Expression::UnaryOp(_, rhs) => {
        children.push(Self::from((&rhs.0, &rhs.1)));

        Self {
          kind: expression.kind(),
          range,
          children,
        }
      }
    }
  }
}
