use super::*;

pub struct Analyzer<'a> {
  environment: Environment<'a>,
  inside_function: bool,
  inside_loop: bool,
}

impl<'a> Analyzer<'a> {
  pub fn new(environment: Environment<'a>) -> Self {
    Self {
      environment,
      inside_function: false,
      inside_loop: false,
    }
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
        let is_valid_lvalue = matches!(
          &lhs.0,
          Expression::Identifier(_) | Expression::ListAccess(_, _)
        );

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
      Statement::Break => {
        if !self.inside_loop {
          errors
            .push(Error::new(*span, "Cannot use 'break' outside of a loop"));
        }
      }
      Statement::Continue => {
        if !self.inside_loop {
          errors
            .push(Error::new(*span, "Cannot use 'continue' outside of a loop"));
        }
      }
      Statement::Expression(expr) => {
        errors.extend(self.analyze_expression(expr));
      }
      Statement::Function(name, parameters, body) => {
        let mut parameter_names = HashSet::new();

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

        let old_environment = self.environment.clone();
        let old_inside_function = self.inside_function;

        self.environment = Environment::with_parent(self.environment.clone());
        self.inside_function = true;

        for &parameter in parameters {
          self.environment.add_variable(parameter, Value::Null);
        }

        for statement in body {
          errors.extend(self.analyze_statement(statement));
        }

        self.environment = old_environment;
        self.inside_function = old_inside_function;
      }
      Statement::If(condition, then_branch, else_branch) => {
        errors.extend(self.analyze_expression(condition));

        for statement in then_branch {
          errors.extend(self.analyze_statement(statement));
        }

        if let Some(else_statements) = else_branch {
          for statement in else_statements {
            errors.extend(self.analyze_statement(statement));
          }
        }
      }
      Statement::Loop(body) => {
        let parent_env = self.environment.clone();
        let old_inside_loop = self.inside_loop;

        self.environment = Environment::with_parent(parent_env.clone());
        self.inside_loop = true;

        for statement in body {
          errors.extend(self.analyze_statement(statement));
        }

        self.environment = parent_env;
        self.inside_loop = old_inside_loop;
      }
      Statement::Return(expr) => {
        if let Some(expr) = expr {
          errors.extend(self.analyze_expression(expr));
        }
      }
      Statement::While(condition, body) => {
        errors.extend(self.analyze_expression(condition));

        let parent_env = self.environment.clone();
        let old_inside_loop = self.inside_loop;

        self.environment = Environment::with_parent(parent_env.clone());
        self.inside_loop = true;

        for statement in body {
          errors.extend(self.analyze_statement(statement));
        }

        self.environment = parent_env;
        self.inside_loop = old_inside_loop;
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
        if !self.environment.has_symbol(name) {
          errors
            .push(Error::new(*span, format!("Undefined function `{}`", name)));
        } else if let Some(Function::UserDefined(Value::Function(
          _,
          parameters,
          _,
          _,
        ))) = self.environment.functions.get(name)
        {
          if arguments.len() != parameters.len() {
            errors.push(Error::new(
              *span,
              format!(
                "Function `{}` expects {} arguments, got {}",
                name,
                parameters.len(),
                arguments.len()
              ),
            ));
          }
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
  use {super::*, anyhow::anyhow};

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

    fn run(self) -> Result {
      let ast = match parse(&self.program) {
        Ok(ast) => ast,
        Err(errors) => {
          return Err(anyhow!("Failed to parse program: {:?}", errors));
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
          return Err(anyhow!(
            "Error {} expected to contain '{}', got '{}'",
            i,
            self.errors[i],
            error.message
          ));
        }
      }

      Ok(())
    }
  }

  #[test]
  fn invalid_lvalues() -> Result {
    Test::new().program("a = 10").errors(&[]).run()?;

    Test::new().program("a = [1, 2, 3]; a[0] = 10").run()?;

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
  fn function_parameters() -> Result {
    Test::new()
      .program("fn add(a, b) { return a + b; }")
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
  fn function_calls() -> Result {
    Test::new().program("sin(3.14)").errors(&[]).run()?;

    Test::new()
      .program("fn greet() { return 'Hello'; }; greet()")
      .errors(&[])
      .run()?;

    Test::new()
      .program("undefined_function()")
      .errors(&["Undefined function `undefined_function`"])
      .run()
  }

  #[test]
  fn break_outside_of_loop() -> Result {
    Test::new()
      .program("a = 0; while (a < 100) { if (i == 25) { break }; a = a + 1 }")
      .run()?;

    Test::new()
      .program("break")
      .errors(&["Cannot use 'break' outside of a loop"])
      .run()
  }

  #[test]
  fn continue_outside_of_loop() -> Result {
    Test::new()
      .program(
        "a = 0; while (a < 100) { if (i == 25) { continue }; a = a + 1 }",
      )
      .run()?;

    Test::new()
      .program("continue")
      .errors(&["Cannot use 'continue' outside of a loop"])
      .run()
  }
}
