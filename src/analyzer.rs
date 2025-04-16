use super::*;

#[derive(Debug, Clone, Default)]
struct SymbolTable<'src> {
  functions: HashMap<&'src str, (Span, bool, Vec<&'src str>)>,
  parent: Option<Box<SymbolTable<'src>>>,
  variables: HashMap<&'src str, (Span, bool)>,
}

impl<'src> SymbolTable<'src> {
  pub fn new() -> Self {
    let mut table = Self::default();

    for func in &[
      "sin", "cos", "tan", "csc", "sec", "cot", "sinh", "cosh", "tanh", "asin",
      "acos", "arc", "acsc", "asec", "acot", "ln", "log2", "log10", "e",
      "sqrt", "ceil", "floor", "abs", "len", "print", "println", "exit",
      "quit", "sum", "input", "int", "float", "bool", "list", "split",
    ] {
      table
        .functions
        .insert(func, (SimpleSpan::new((), 0..0), false, vec![]));
    }

    for var in &["e", "pi"] {
      table
        .variables
        .insert(var, (SimpleSpan::new((), 0..0), false));
    }

    table
  }

  pub fn with_parent(parent: SymbolTable<'src>) -> Self {
    Self {
      variables: HashMap::new(),
      functions: HashMap::new(),
      parent: Some(Box::new(parent)),
    }
  }

  pub fn declare_variable(&mut self, name: &'src str, span: Span) -> bool {
    if self.variables.contains_key(name) {
      false
    } else {
      self.variables.insert(name, (span, false));
      true
    }
  }

  pub fn declare_function(
    &mut self,
    name: &'src str,
    span: Span,
    parameters: Vec<&'src str>,
  ) -> bool {
    if self.functions.contains_key(name) {
      false
    } else {
      self.functions.insert(name, (span, false, parameters));
      true
    }
  }

  pub fn mark_variable_used(&mut self, name: &'src str) -> bool {
    if let Some((_, used)) = self.variables.get_mut(name) {
      *used = true;
      true
    } else if let Some(parent) = &mut self.parent {
      parent.mark_variable_used(name)
    } else {
      false
    }
  }

  pub fn mark_function_used(&mut self, name: &'src str) -> bool {
    if let Some((_, used, _)) = self.functions.get_mut(name) {
      *used = true;
      true
    } else if let Some(parent) = &mut self.parent {
      parent.mark_function_used(name)
    } else {
      false
    }
  }

  pub fn get_function(
    &self,
    name: &str,
  ) -> Option<&(Span, bool, Vec<&'src str>)> {
    if let Some(func) = self.functions.get(name) {
      Some(func)
    } else if let Some(parent) = &self.parent {
      parent.get_function(name)
    } else {
      None
    }
  }

  pub fn get_unused_variables(&self) -> Vec<(&'src str, Span)> {
    let mut unused = Vec::new();

    for (name, (span, used)) in &self.variables {
      if !used && span.start != 0 {
        unused.push((*name, *span));
      }
    }

    if let Some(parent) = &self.parent {
      unused.extend(parent.get_unused_variables());
    }

    unused
  }

  pub fn get_unused_functions(&self) -> Vec<(&'src str, Span)> {
    let mut unused = Vec::new();

    for (name, (span, used, _)) in &self.functions {
      if !used && span.start != 0 {
        unused.push((*name, *span));
      }
    }

    if let Some(parent) = &self.parent {
      unused.extend(parent.get_unused_functions());
    }

    unused
  }
}

#[derive(Clone)]
pub struct Analyzer<'src> {
  symbol_table: SymbolTable<'src>,
  errors: Vec<Error>,
  return_seen: bool,
}

impl<'src> Analyzer<'src> {
  pub fn new() -> Self {
    Self {
      symbol_table: SymbolTable::new(),
      errors: Vec::new(),
      return_seen: false,
    }
  }

  pub fn analyze(&mut self, ast: &Spanned<Program<'src>>) -> Vec<Error> {
    self.analyze_program(ast);

    for (name, span) in self.symbol_table.get_unused_variables() {
      self.errors.push(Error::new(
        span,
        format!("Variable `{}` is declared but never used", name),
      ));
    }

    for (name, span) in self.symbol_table.get_unused_functions() {
      self.errors.push(Error::new(
        span,
        format!("Function `{}` is defined but never called", name),
      ));
    }

    self.errors.clone()
  }

  fn analyze_program(&mut self, program: &Spanned<Program<'src>>) {
    let (node, _) = program;

    match node {
      Program::Statements(statements) => {
        for statement in statements {
          self.analyze_statement(statement);
        }
      }
    }
  }

  fn analyze_statement(&mut self, statement: &Spanned<Statement<'src>>) {
    if self.return_seen {
      self.errors.push(Error::new(
        statement.1,
        "Unreachable code after return statement".to_string(),
      ));

      return;
    }

    let (node, span) = statement;

    match node {
      Statement::Assignment(name, expr) => {
        self.analyze_expression(expr);

        if !self.symbol_table.declare_variable(name, *span) {
          self.symbol_table.mark_variable_used(name);
        }
      }
      Statement::Block(statements) => {
        let parent_table = self.symbol_table.clone();

        self.symbol_table = SymbolTable::with_parent(parent_table);

        let previous_return_seen = self.return_seen;

        self.return_seen = false;

        for statement in statements {
          self.analyze_statement(statement);

          if self.return_seen {
            break;
          }
        }

        let child_table = &self.symbol_table;

        self.symbol_table = *child_table.parent.as_ref().unwrap().clone();

        self.return_seen = previous_return_seen;
      }
      Statement::Expression(expression) => {
        self.analyze_expression(expression);
      }
      Statement::Function(name, params, body) => {
        if !self
          .symbol_table
          .declare_function(name, *span, params.clone())
        {
          self.errors.push(Error::new(
            *span,
            format!("Function `{}` is already defined in this scope", name),
          ));
        }

        let parent_table = self.symbol_table.clone();

        self.symbol_table = SymbolTable::with_parent(parent_table);

        for param in params {
          self.symbol_table.declare_variable(param, *span);
          self.symbol_table.mark_variable_used(param);
        }

        let previous_return_seen = self.return_seen;

        self.return_seen = false;

        for statement in body {
          self.analyze_statement(statement);

          if self.return_seen {
            break;
          }
        }

        let child_table = self.symbol_table.clone();

        self.symbol_table = *child_table.parent.as_ref().unwrap().clone();

        self.return_seen = previous_return_seen;
      }
      Statement::If(condition, then_branch, else_branch) => {
        self.analyze_expression(condition);

        let parent_table = self.symbol_table.clone();
        self.symbol_table = SymbolTable::with_parent(parent_table.clone());

        let previous_return_seen = self.return_seen;
        self.return_seen = false;

        for statement in then_branch {
          self.analyze_statement(statement);
          if self.return_seen {
            break;
          }
        }

        let then_returns = self.return_seen;

        self.symbol_table = parent_table;
        self.return_seen = previous_return_seen;

        if let Some(else_statements) = else_branch {
          let parent_table = self.symbol_table.clone();

          self.symbol_table = SymbolTable::with_parent(parent_table);

          self.return_seen = false;

          for statement in else_statements {
            self.analyze_statement(statement);

            if self.return_seen {
              break;
            }
          }

          if then_returns && self.return_seen {
            self.return_seen = true;
          } else {
            self.return_seen = previous_return_seen;
          }

          let child_table = self.symbol_table.clone();

          self.symbol_table = *child_table.parent.as_ref().unwrap().clone();
        } else {
          self.return_seen = previous_return_seen;
        }
      }
      Statement::Return(expr) => {
        if let Some(expr) = expr {
          self.analyze_expression(expr);
        }

        self.return_seen = true;
      }
      Statement::While(condition, body) => {
        self.analyze_expression(condition);

        let parent_table = self.symbol_table.clone();

        self.symbol_table = SymbolTable::with_parent(parent_table);

        let previous_return_seen = self.return_seen;

        self.return_seen = false;

        for statement in body {
          self.analyze_statement(statement);

          if self.return_seen {
            break;
          }
        }

        let child_table = self.symbol_table.clone();

        self.symbol_table = *child_table.parent.as_ref().unwrap().clone();

        self.return_seen = previous_return_seen;
      }
    }
  }

  fn analyze_expression(&mut self, expression: &Spanned<Expression<'src>>) {
    let (node, span) = expression;

    match node {
      Expression::BinaryOp(op, lhs, rhs) => {
        self.analyze_expression(lhs);
        self.analyze_expression(rhs);

        match op {
          BinaryOp::Divide => {
            if let Expression::Number(n) = &rhs.0 {
              if *n == 0.0 {
                self
                  .errors
                  .push(Error::new(rhs.1, "Division by zero".to_string()));
              }
            }
          }
          BinaryOp::Modulo => {
            if let Expression::Number(n) = &rhs.0 {
              if *n == 0.0 {
                self
                  .errors
                  .push(Error::new(rhs.1, "Modulo by zero".to_string()));
              }
            }
          }
          _ => {}
        }
      }
      Expression::Boolean(_) => {}
      Expression::FunctionCall(name, arguments) => {
        if let Some((_, _, params)) = self.symbol_table.get_function(name) {
          let params_len = params.len();

          let has_params = !params.is_empty();

          self.symbol_table.mark_function_used(name);

          if has_params && params_len != arguments.len() {
            self.errors.push(Error::new(
              *span,
              format!(
                "Function `{}` expects {} arguments, got {}",
                name,
                params_len,
                arguments.len()
              ),
            ));
          }
        } else {
          self.errors.push(Error::new(
            *span,
            format!("Call to undefined function `{}`", name),
          ));
        };

        for argument in arguments {
          self.analyze_expression(argument);
        }
      }
      Expression::Identifier(name) => {
        if !self.symbol_table.mark_variable_used(name) {
          self.errors.push(Error::new(
            *span,
            format!("Reference to undefined variable `{}`", name),
          ));
        }
      }
      Expression::List(items) => {
        for item in items {
          self.analyze_expression(item);
        }
      }
      Expression::ListAccess(list, index) => {
        self.analyze_expression(list);
        self.analyze_expression(index);

        if let Expression::Number(idx) = &index.0 {
          if *idx < 0.0 {
            self.errors.push(Error::new(
              index.1,
              format!("Negative list index: {}", idx),
            ));
          }

          if let Expression::List(items) = &list.0 {
            let idx_usize = *idx as usize;

            if idx_usize >= items.len() {
              self.errors.push(Error::new(
                *span,
                format!(
                  "List index {} out of bounds for list of length {}",
                  idx_usize,
                  items.len()
                ),
              ));
            }
          }
        }
      }
      Expression::Number(_) => {}
      Expression::String(_) => {}
      Expression::UnaryOp(_, expression) => {
        self.analyze_expression(expression);
      }
    }
  }
}
