use super::*;

pub struct Analyzer<'a> {
  environment: Environment<'a>,
}

impl<'a> Analyzer<'a> {
  pub fn new(environment: Environment<'a>) -> Self {
    Self { environment }
  }

  pub fn analyze(&mut self, ast: &Spanned<Program<'a>>) -> Vec<Error> {
    let (program, _) = ast;

    match program {
      Program::Statements(statements) => {
        let mut errors = Vec::new();

        for statement in statements {
          errors.extend(self.analyze_statement(statement));
        }

        errors
      }
    }
  }

  fn analyze_statement(
    &mut self,
    statement: &Spanned<Statement<'a>>,
  ) -> Vec<Error> {
    let (statement, span) = statement;

    let mut errors = Vec::new();

    match statement {
      Statement::Assignment(lhs, rhs) => {
        let is_valid_lvalue = match &lhs.0 {
          Expression::Identifier(_) => true,
          Expression::ListAccess(_, _) => true,
          _ => false,
        };

        if !is_valid_lvalue {
          errors.push(Error::new(
            lhs.1,
            "left-hand side must be a variable or list element",
          ));
        }

        errors.extend(self.analyze_expression(rhs));
      }
      Statement::Block(statements) => {
        for statement in statements {
          errors.extend(self.analyze_statement(statement));
        }
      }
      Statement::Break => {}
      Statement::Continue => {}
      Statement::Expression(expr) => {
        errors.extend(self.analyze_expression(expr));
      }
      Statement::Function(name, parameters, body) => {
        let mut parameter_names = std::collections::HashSet::new();

        for &name in parameters {
          if !parameter_names.insert(name) {
            errors.push(Error::new(
              *span,
              format!("Duplicate parameter name `{}`", name),
            ));
          }
        }

        let function = Value::Function(
          name,
          parameters.clone(),
          body.clone(),
          self.environment.clone(),
        );

        self
          .environment
          .add_function(name, Function::UserDefined(function.clone()));

        for statement in body {
          errors.extend(self.analyze_statement(statement));
        }
      }
      Statement::If(condition, then_branch, else_branch) => {
        errors.extend(self.analyze_expression(condition));

        for statement in then_branch {
          errors.extend(self.analyze_statement(statement));
        }

        if let Some(else_statements) = else_branch {
          for stmt in else_statements {
            errors.extend(self.analyze_statement(stmt));
          }
        }
      }
      Statement::Loop(body) => {
        for stmt in body {
          errors.extend(self.analyze_statement(stmt));
        }
      }
      Statement::Return(expr) => {
        if let Some(expr) = expr {
          errors.extend(self.analyze_expression(expr));
        }
      }
      Statement::While(condition, body) => {
        errors.extend(self.analyze_expression(condition));

        for stmt in body {
          errors.extend(self.analyze_statement(stmt));
        }
      }
    }

    errors
  }

  fn analyze_expression(
    &self,
    expression: &Spanned<Expression<'a>>,
  ) -> Vec<Error> {
    let (expr, span) = expression;

    let mut errors = Vec::new();

    match expr {
      Expression::BinaryOp(_, lhs, rhs) => {
        errors.extend(self.analyze_expression(lhs));
        errors.extend(self.analyze_expression(rhs));
      }
      Expression::UnaryOp(_, expr) => {
        errors.extend(self.analyze_expression(expr));
      }
      Expression::FunctionCall(name, arguments) => {
        if !self.environment.functions.contains_key(name) {
          errors
            .push(Error::new(*span, format!("Undefined function `{}`", name)));
        }

        for argument in arguments {
          errors.extend(self.analyze_expression(argument));
        }
      }
      Expression::Identifier(_) => {}
      Expression::List(items) => {
        for item in items {
          errors.extend(self.analyze_expression(item));
        }
      }
      Expression::ListAccess(list, index) => {
        errors.extend(self.analyze_expression(list));
        errors.extend(self.analyze_expression(index));
      }
      _ => {}
    }

    errors
  }
}
