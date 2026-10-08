use super::*;

#[derive(Clone, Serialize)]
pub struct AstNode {
  pub kind: &'static str,
  pub range: Range,
  pub children: Vec<AstNode>,
}

impl AstNode {
  fn from_assignment_target(
    target: &Spanned<AssignmentTarget>,
    converter: &RangeConverter,
  ) -> Self {
    let (target, span) = target;

    let range = converter.convert(span);

    let mut children = Vec::new();

    match target {
      AssignmentTarget::Identifier(_) => {}
      AssignmentTarget::ListAccess(list, index) => {
        children.push(Self::from_assignment_target(list, converter));
        children.push(Self::from_expression(index, converter));
      }
    }

    Self {
      kind: target.kind(),
      range,
      children,
    }
  }

  fn from_expression(
    expression: &Spanned<Expression>,
    converter: &RangeConverter,
  ) -> Self {
    let (expression, span) = expression;

    let range = converter.convert(span);

    let mut children = Vec::new();

    match expression {
      Expression::BinaryOp(_, lhs, rhs) => {
        children.push(Self::from_expression(lhs, converter));
        children.push(Self::from_expression(rhs, converter));
      }
      Expression::Boolean(_)
      | Expression::Identifier(_)
      | Expression::Null
      | Expression::Number(_)
      | Expression::String(_) => {}
      Expression::Function(_, body) => {
        for statement in body {
          children.push(Self::from_statement(statement, converter));
        }
      }
      Expression::FunctionCall(function, arguments) => {
        children.push(Self::from_expression(function, converter));

        for argument in arguments {
          children.push(Self::from_expression(argument, converter));
        }
      }
      Expression::List(items) => {
        for item in items {
          children.push(Self::from_expression(item, converter));
        }
      }
      Expression::ListAccess(list, index) => {
        children.push(Self::from_expression(list, converter));
        children.push(Self::from_expression(index, converter));
      }
      Expression::UnaryOp(_, rhs) => {
        children.push(Self::from_expression(rhs, converter));
      }
    }

    Self {
      kind: expression.kind(),
      range,
      children,
    }
  }

  pub(crate) fn from_program(program: &Spanned<Program>) -> Self {
    let (program, span) = program;

    let converter = RangeConverter::new(span.source().text());

    let range = converter.convert(span);

    let mut children = Vec::new();

    for statement in &program.statements {
      children.push(Self::from_statement(statement, &converter));
    }

    Self {
      kind: program.kind(),
      range,
      children,
    }
  }

  fn from_statement(
    statement: &Spanned<Statement>,
    converter: &RangeConverter,
  ) -> Self {
    let (statement, span) = statement;

    let range = converter.convert(span);

    let mut children = Vec::new();

    match statement {
      Statement::Assignment(lhs, rhs) => {
        children.push(Self::from_assignment_target(lhs, converter));
        children.push(Self::from_expression(rhs, converter));
      }
      Statement::Block(statements)
      | Statement::Function(_, _, statements)
      | Statement::Loop(statements) => {
        for statement in statements {
          children.push(Self::from_statement(statement, converter));
        }
      }
      Statement::Break | Statement::Continue => {}
      Statement::Expression(expression) => {
        children.push(Self::from_expression(expression, converter));
      }
      Statement::For(_, iterable, body) => {
        children.push(Self::from_expression(iterable, converter));

        for statement in body {
          children.push(Self::from_statement(statement, converter));
        }
      }
      Statement::If(condition, then_branch, else_branch) => {
        children.push(Self::from_expression(condition, converter));

        for statement in then_branch {
          children.push(Self::from_statement(statement, converter));
        }

        if let Some(else_statements) = else_branch {
          for statement in else_statements {
            children.push(Self::from_statement(statement, converter));
          }
        }
      }
      Statement::Return(expression) => {
        if let Some(expression) = expression {
          children.push(Self::from_expression(expression, converter));
        }
      }
      Statement::While(condition, body) => {
        children.push(Self::from_expression(condition, converter));

        for statement in body {
          children.push(Self::from_statement(statement, converter));
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn converts_nested_ranges() {
    let program = val::parse(r#"["é", "😀"]"#).unwrap();

    let node = AstNode::from_program(&program);

    assert_eq!(node.range, Range { start: 0, end: 11 });

    assert_eq!(
      node.children[0].children[0].children[1].range,
      Range { start: 6, end: 10 },
    );
  }
}
