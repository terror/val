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
            "Left-hand side of assignment must be a variable or list access",
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
        for statement in body {
          errors.extend(self.analyze_statement(statement));
        }
      }
      Statement::Return(expr) => {
        if let Some(expr) = expr {
          errors.extend(self.analyze_expression(expr));
        }
      }
      Statement::While(condition, body) => {
        errors.extend(self.analyze_expression(condition));

        for statement in body {
          errors.extend(self.analyze_statement(statement));
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

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug)]
  struct Test {
    program: String,
    errors: Vec<String>,
  }

  impl Test {
    fn new() -> Self {
      Self {
        program: String::new(),
        errors: Vec::new(),
      }
    }

    fn errors(self, errors: &[&str]) -> Self {
      Self {
        errors: errors
          .iter()
          .map(|s| s.to_string())
          .collect::<Vec<String>>(),
        ..self
      }
    }

    fn program(self, program: &str) -> Self {
      Self {
        program: program.to_owned(),
        ..self
      }
    }

    fn run(self) -> Result<(), String> {
      let ast = match parse(&self.program) {
        Ok(ast) => ast,
        Err(errors) => {
          return Err(format!("Failed to parse program: {:?}", errors));
        }
      };

      let environment = Environment::new(Config::default());

      let mut analyzer = Analyzer::new(environment);

      let analysis_errors = analyzer.analyze(&ast);

      assert_eq!(
        analysis_errors.len(),
        self.errors.len(),
        "Expected {} error(s), got {}:\n{:?}",
        self.errors.len(),
        analysis_errors.len(),
        analysis_errors,
      );

      for (i, error) in analysis_errors.iter().enumerate() {
        if !error.message.contains(&self.errors[i]) {
          return Err(format!(
            "Error {} expected to contain '{}', got '{}'",
            i, self.errors[i], error.message
          ));
        }
      }

      Ok(())
    }
  }

  #[test]
  fn invalid_lvalues() -> Result<(), String> {
    Test::new().program("a = 10").errors(&[]).run()?;

    Test::new()
      .program("a = [1, 2, 3]; a[0] = 10")
      .errors(&[])
      .run()?;

    Test::new()
      .program("\"foo\" = 10")
      .errors(&[
        "Left-hand side of assignment must be a variable or list access",
      ])
      .run()?;

    Test::new()
      .program("5 = 10")
      .errors(&[
        "Left-hand side of assignment must be a variable or list access",
      ])
      .run()
  }

  #[test]
  fn function_parameters() -> Result<(), String> {
    Test::new()
      .program("fn add(a, b) { return a + b; }")
      .errors(&[])
      .run()?;

    Test::new()
      .program("fn add(a, a) { return a + a; }")
      .errors(&["Duplicate parameter name `a`"])
      .run()?;

    Test::new()
      .program("fn add(a, b, a, c, b) { return a + b + c; }")
      .errors(&[
        "Duplicate parameter name `a`",
        "Duplicate parameter name `b`",
      ])
      .run()
  }

  #[test]
  fn function_calls() -> Result<(), String> {
    Test::new().program("sin(3.14)").errors(&[]).run()?;

    Test::new()
      .program("fn greet() { return 'Hello'; }; greet()")
      .errors(&[])
      .run()?;

    Test::new()
      .program("undefined_function()")
      .errors(&["Undefined function"])
      .run()
  }
}
