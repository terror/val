use super::*;

const COLOR_BOOLEAN: &str = "\x1b[33m"; // Yellow
const COLOR_ERROR: &str = "\x1b[31m"; // Red
const COLOR_FUNCTION: &str = "\x1b[34m"; // Blue
const COLOR_IDENTIFIER: &str = "\x1b[37m"; // White
const COLOR_KEYWORD: &str = "\x1b[35m"; // Magenta
const COLOR_NUMBER: &str = "\x1b[33m"; // Yellow
const COLOR_OPERATOR: &str = "\x1b[36m"; // Cyan
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_STRING: &str = "\x1b[32m"; // Green

pub struct TreeHighlighter<'src> {
  content: &'src str,
}

impl<'src> TreeHighlighter<'src> {
  pub fn new(content: &'src str) -> Self {
    Self { content }
  }

  pub fn highlight(&self) -> Cow<'src, str> {
    match parse(self.content) {
      Ok(ast) => self.colorize_ast(&ast),
      Err(_) => {
        Owned(format!("{}{}{}", COLOR_ERROR, self.content, COLOR_RESET))
      }
    }
  }

  fn colorize_ast(&self, program: &Spanned<Program<'src>>) -> Cow<'src, str> {
    let mut color_spans = Vec::new();
    self.collect_color_spans(program, &mut color_spans);
    color_spans.sort_by_key(|span| span.0);
    self.apply_color_spans(&color_spans)
  }

  fn apply_color_spans(
    &self,
    spans: &[(usize, usize, &str)],
  ) -> Cow<'src, str> {
    if spans.is_empty() {
      return Cow::Borrowed(self.content);
    }

    let mut result =
      String::with_capacity(self.content.len() + spans.len() * 10);

    let mut last_end = 0;

    for &(start, end, color) in spans {
      if start > last_end {
        result.push_str(&self.content[last_end..start]);
      }

      result.push_str(color);
      result.push_str(&self.content[start..end]);
      result.push_str(COLOR_RESET);

      last_end = end;
    }

    if last_end < self.content.len() {
      result.push_str(&self.content[last_end..]);
    }

    Owned(result)
  }

  fn collect_color_spans(
    &self,
    program: &Spanned<Program<'src>>,
    spans: &mut Vec<(usize, usize, &'static str)>,
  ) {
    let (node, _) = program;

    match node {
      Program::Statements(statements) => {
        for statement in statements {
          self.collect_statement_spans(statement, spans);
        }
      }
    }
  }

  fn collect_statement_spans(
    &self,
    statement: &Spanned<Statement<'src>>,
    spans: &mut Vec<(usize, usize, &'static str)>,
  ) {
    let (node, span) = statement;

    let (start, end) = (span.start, span.end);

    match node {
      Statement::Assignment(name, expr) => {
        let name_span = self.find_identifier_span(start, name);

        if let Some((name_start, name_end)) = name_span {
          spans.push((name_start, name_end, COLOR_IDENTIFIER));
        }

        if let Some(eq_pos) = self.content[start..end].find('=') {
          spans.push((start + eq_pos, start + eq_pos + 1, COLOR_OPERATOR));
        }

        self.collect_expression_spans(expr, spans);
      }
      Statement::Block(statements) => {
        if let Some(open_brace) = self.content[start..end].find('{') {
          spans.push((
            start + open_brace,
            start + open_brace + 1,
            COLOR_OPERATOR,
          ));
        }

        if let Some(close_brace) = self.content[start..end].rfind('}') {
          spans.push((
            start + close_brace,
            start + close_brace + 1,
            COLOR_OPERATOR,
          ));
        }

        for statement in statements {
          self.collect_statement_spans(statement, spans);
        }
      }
      Statement::Expression(expression) => {
        self.collect_expression_spans(expression, spans);
      }
      Statement::Function(name, params, body) => {
        if let Some(fn_pos) = self.content[start..end].find("fn") {
          spans.push((start + fn_pos, start + fn_pos + 2, COLOR_KEYWORD));
        }

        let name_span = self.find_identifier_span(start, name);

        if let Some((name_start, name_end)) = name_span {
          spans.push((name_start, name_end, COLOR_FUNCTION));
        }

        for param in params {
          let param_span = self.find_identifier_span(start, param);
          if let Some((param_start, param_end)) = param_span {
            spans.push((param_start, param_end, COLOR_IDENTIFIER));
          }
        }

        if let Some(open_paren) = self.content[start..end].find('(') {
          spans.push((
            start + open_paren,
            start + open_paren + 1,
            COLOR_OPERATOR,
          ));
        }

        if let Some(close_paren) = self.content[start..end].find(')') {
          spans.push((
            start + close_paren,
            start + close_paren + 1,
            COLOR_OPERATOR,
          ));
        }

        if let Some(open_brace) = self.content[start..end].find('{') {
          spans.push((
            start + open_brace,
            start + open_brace + 1,
            COLOR_OPERATOR,
          ));
        }

        if let Some(close_brace) = self.content[start..end].rfind('}') {
          spans.push((
            start + close_brace,
            start + close_brace + 1,
            COLOR_OPERATOR,
          ));
        }

        for statement in body {
          self.collect_statement_spans(statement, spans);
        }
      }
      Statement::If(condition, then_branch, else_branch) => {
        if let Some(if_pos) = self.content[start..end].find("if") {
          spans.push((start + if_pos, start + if_pos + 2, COLOR_KEYWORD));
        }

        self.collect_expression_spans(condition, spans);

        for statement in then_branch {
          self.collect_statement_spans(statement, spans);
        }

        if let Some(else_statements) = else_branch {
          if let Some(else_pos) = self.content[start..end].find("else") {
            spans.push((start + else_pos, start + else_pos + 4, COLOR_KEYWORD));
          }

          for statement in else_statements {
            self.collect_statement_spans(statement, spans);
          }
        }
      }
      Statement::Return(expr_opt) => {
        if let Some(return_pos) = self.content[start..end].find("return") {
          spans.push((
            start + return_pos,
            start + return_pos + 6,
            COLOR_KEYWORD,
          ));
        }

        if let Some(expr) = expr_opt {
          self.collect_expression_spans(expr, spans);
        }
      }
      Statement::While(condition, body) => {
        if let Some(while_pos) = self.content[start..end].find("while") {
          spans.push((start + while_pos, start + while_pos + 5, COLOR_KEYWORD));
        }

        self.collect_expression_spans(condition, spans);

        if let Some(open_paren) = self.content[start..end].find('(') {
          spans.push((
            start + open_paren,
            start + open_paren + 1,
            COLOR_OPERATOR,
          ));
        }

        if let Some(close_paren) = self.content[start..end].find(')') {
          spans.push((
            start + close_paren,
            start + close_paren + 1,
            COLOR_OPERATOR,
          ));
        }

        for statement in body {
          self.collect_statement_spans(statement, spans);
        }
      }
    }
  }

  fn collect_expression_spans(
    &self,
    expression: &Spanned<Expression<'src>>,
    spans: &mut Vec<(usize, usize, &'static str)>,
  ) {
    let (node, span) = expression;

    let (start, end) = (span.start, span.end);

    match node {
      Expression::BinaryOp(op, lhs, rhs) => {
        self.collect_expression_spans(lhs, spans);
        self.collect_expression_spans(rhs, spans);

        let op_str = op.to_string();

        if let Some(op_pos) = self.find_operator(&op_str, lhs, rhs) {
          spans.push((op_pos, op_pos + op_str.len(), COLOR_OPERATOR));
        }
      }
      Expression::Boolean(value) => {
        let value_str = if *value { "true" } else { "false" };

        if let Some(bool_pos) = self.content[start..end].find(value_str) {
          spans.push((
            start + bool_pos,
            start + bool_pos + value_str.len(),
            COLOR_BOOLEAN,
          ));
        }
      }
      Expression::FunctionCall(name, arguments) => {
        let name_span = self.find_identifier_span(start, name);

        if let Some((name_start, name_end)) = name_span {
          spans.push((name_start, name_end, COLOR_FUNCTION));
        }

        if let Some(open_paren) = self.content[start..end].find('(') {
          spans.push((
            start + open_paren,
            start + open_paren + 1,
            COLOR_OPERATOR,
          ));
        }

        if let Some(close_paren) = self.content[start..end].rfind(')') {
          spans.push((
            start + close_paren,
            start + close_paren + 1,
            COLOR_OPERATOR,
          ));
        }

        for argument in arguments {
          self.collect_expression_spans(argument, spans);
        }
      }
      Expression::Identifier(name) => {
        let name_span = self.find_identifier_span(start, name);

        if let Some((name_start, name_end)) = name_span {
          spans.push((name_start, name_end, COLOR_IDENTIFIER));
        }
      }
      Expression::List(items) => {
        if let Some(open_bracket) = self.content[start..end].find('[') {
          spans.push((
            start + open_bracket,
            start + open_bracket + 1,
            COLOR_OPERATOR,
          ));
        }

        if let Some(close_bracket) = self.content[start..end].rfind(']') {
          spans.push((
            start + close_bracket,
            start + close_bracket + 1,
            COLOR_OPERATOR,
          ));
        }

        for item in items {
          self.collect_expression_spans(item, spans);
        }
      }
      Expression::ListAccess(list, index) => {
        self.collect_expression_spans(list, spans);

        if let Some(open_bracket) = self.content[list.1.end..end].find('[') {
          spans.push((
            list.1.end + open_bracket,
            list.1.end + open_bracket + 1,
            COLOR_OPERATOR,
          ));
        }

        if let Some(close_bracket) = self.content[list.1.end..end].rfind(']') {
          spans.push((
            list.1.end + close_bracket,
            list.1.end + close_bracket + 1,
            COLOR_OPERATOR,
          ));
        }

        self.collect_expression_spans(index, spans);
      }
      Expression::Number(_) => {
        let number_pattern = self.find_number_span(start, end);

        if let Some((num_start, num_end)) = number_pattern {
          spans.push((num_start, num_end, COLOR_NUMBER));
        }
      }
      Expression::String(value) => {
        let quoted_value = format!("'{}'", value);

        if let Some(str_pos) = self.content[start..end].find(&quoted_value) {
          spans.push((
            start + str_pos,
            start + str_pos + quoted_value.len(),
            COLOR_STRING,
          ));
        } else {
          let double_quoted = format!("\"{}\"", value);

          if let Some(str_pos) = self.content[start..end].find(&double_quoted) {
            spans.push((
              start + str_pos,
              start + str_pos + double_quoted.len(),
              COLOR_STRING,
            ));
          }
        }
      }
      Expression::UnaryOp(op, expr) => {
        let op_str = op.to_string();

        if let Some(op_pos) = self.content[start..expr.1.start].find(&op_str) {
          spans.push((
            start + op_pos,
            start + op_pos + op_str.len(),
            COLOR_OPERATOR,
          ));
        }

        self.collect_expression_spans(expr, spans);
      }
    }
  }

  fn find_identifier_span(
    &self,
    start_search: usize,
    name: &str,
  ) -> Option<(usize, usize)> {
    Regex::new(&format!(r"\b{}\b", regex::escape(name)))
      .ok()?
      .find(&self.content[start_search..])
      .map(|mat| (start_search + mat.start(), start_search + mat.end()))
  }

  fn find_operator(
    &self,
    op: &str,
    lhs: &Spanned<Expression<'src>>,
    rhs: &Spanned<Expression<'src>>,
  ) -> Option<usize> {
    let (start, end) = (lhs.1.end, rhs.1.start);

    self.content[start..end].find(op).map(|pos| start + pos)
  }

  fn find_number_span(
    &self,
    start: usize,
    end: usize,
  ) -> Option<(usize, usize)> {
    Regex::new(r"[-+]?\d+(\.\d+)?")
      .ok()?
      .find(&self.content[start..end])
      .map(|mat| (start + mat.start(), start + mat.end()))
  }
}

pub struct Highlighter {
  completer: FilenameCompleter,
  hinter: HistoryHinter,
}

impl Highlighter {
  pub fn new() -> Self {
    Self {
      completer: FilenameCompleter::new(),
      hinter: HistoryHinter::new(),
    }
  }
}

impl Completer for Highlighter {
  type Candidate = Pair;
  fn complete(
    &self,
    line: &str,
    pos: usize,
    ctx: &Context<'_>,
  ) -> Result<(usize, Vec<Pair>), ReadlineError> {
    self.completer.complete(line, pos, ctx)
  }
}

impl Helper for Highlighter {}

impl Hinter for Highlighter {
  type Hint = String;

  fn hint(&self, line: &str, a: usize, b: &Context) -> Option<Self::Hint> {
    self.hinter.hint(line, a, b)
  }
}

impl RustylineHighlighter for Highlighter {
  fn highlight_char(&self, _: &str, _: usize, _: CmdKind) -> bool {
    true
  }

  fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
    Owned(format!("\x1b[90m{}\x1b[0m", hint))
  }

  fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
    TreeHighlighter::new(line).highlight()
  }
}

impl Validator for Highlighter {}
