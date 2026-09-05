use super::*;

#[derive(Clone, Serialize)]
pub struct AstNode {
  pub kind: String,
  pub range: Range,
  pub children: Vec<AstNode>,
}

impl AstNode {
  pub(crate) fn convert_ranges(&mut self, converter: &RangeConverter) {
    self.range = converter.convert(self.range);

    for child in &mut self.children {
      child.convert_ranges(converter);
    }
  }
}

impl From<(&Program, &Span)> for AstNode {
  fn from(value: (&Program, &Span)) -> Self {
    let (program, span) = value;

    let range = Range::from(span);

    let mut children = Vec::new();

    match program {
      Program::Statements(statements) => {
        for (statement, span) in statements {
          children.push(Self::from((statement, span)));
        }
      }
    }

    Self {
      kind: program.kind(),
      range,
      children,
    }
  }
}

impl From<(&Statement, &Span)> for AstNode {
  fn from(value: (&Statement, &Span)) -> Self {
    let (statement, span) = value;

    let range = Range::from(span);

    let mut children = Vec::new();

    match statement {
      Statement::Assignment(lhs, rhs) => {
        children.push(Self::from((&lhs.0, &lhs.1)));
        children.push(Self::from((&rhs.0, &rhs.1)));
      }
      Statement::Block(statements)
      | Statement::Function(_, _, statements)
      | Statement::Loop(statements) => {
        for (statement, span) in statements {
          children.push(Self::from((statement, span)));
        }
      }
      Statement::Break | Statement::Continue => {}
      Statement::Expression(expression) => {
        children.push(Self::from((&expression.0, &expression.1)));
      }
      Statement::For(_, iterable, body) => {
        children.push(Self::from((&iterable.0, &iterable.1)));

        for (statement, span) in body {
          children.push(Self::from((statement, span)));
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
      }
      Statement::Return(expression) => {
        if let Some(expression) = expression {
          children.push(Self::from((&expression.0, &expression.1)));
        }
      }
      Statement::While(condition, body) => {
        children.push(Self::from((&condition.0, &condition.1)));

        for (statement, span) in body {
          children.push(Self::from((statement, span)));
        }
      }
    }

    Self {
      kind: statement.kind(),
      range,
      children,
    }
  }
}

impl From<(&AssignmentTarget, &Span)> for AstNode {
  fn from(value: (&AssignmentTarget, &Span)) -> Self {
    let (target, span) = value;

    let range = Range::from(span);

    let mut children = Vec::new();

    match target {
      AssignmentTarget::Identifier(_) => {}
      AssignmentTarget::ListAccess(list, index) => {
        children.push(Self::from((&list.0, &list.1)));
        children.push(Self::from((&index.0, &index.1)));
      }
    }

    Self {
      kind: target.kind(),
      range,
      children,
    }
  }
}

impl From<(&Expression, &Span)> for AstNode {
  fn from(value: (&Expression, &Span)) -> Self {
    let (expression, span) = value;

    let range = Range::from(span);

    let mut children = Vec::new();

    match expression {
      Expression::BinaryOp(_, lhs, rhs) => {
        children.push(Self::from((&lhs.0, &lhs.1)));
        children.push(Self::from((&rhs.0, &rhs.1)));
      }
      Expression::Boolean(_)
      | Expression::Identifier(_)
      | Expression::Null
      | Expression::Number(_)
      | Expression::String(_) => {}
      Expression::Function(_, body) => {
        for (statement, span) in body {
          children.push(Self::from((statement, span)));
        }
      }
      Expression::FunctionCall(function, arguments) => {
        children.push(Self::from((&function.0, &function.1)));

        for (ast, span) in arguments {
          children.push(Self::from((ast, span)));
        }
      }
      Expression::List(items) => {
        for (item, span) in items {
          children.push(Self::from((item, span)));
        }
      }
      Expression::ListAccess(list, index) => {
        children.push(Self::from((&list.0, &list.1)));
        children.push(Self::from((&index.0, &index.1)));
      }
      Expression::UnaryOp(_, rhs) => {
        children.push(Self::from((&rhs.0, &rhs.1)));
      }
    }

    Self {
      kind: expression.kind(),
      range,
      children,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn converts_nested_ranges() {
    let converter = RangeConverter::new("é 😀");

    let mut node = AstNode {
      kind: "foo".into(),
      range: Range { start: 0, end: 7 },
      children: vec![AstNode {
        kind: "bar".into(),
        range: Range { start: 3, end: 7 },
        children: Vec::new(),
      }],
    };

    node.convert_ranges(&converter);

    assert_eq!(node.range, Range { start: 0, end: 4 });
    assert_eq!(node.children[0].range, Range { start: 2, end: 4 });
  }
}
