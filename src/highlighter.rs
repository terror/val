use super::*;

const COLOR_RESET: &str = "\x1b[0m";

pub(crate) struct Highlighter<'src> {
  content: &'src str,
}

impl<'src> Highlighter<'src> {
  fn apply_color_spans(&self, spans: &[HighlightSpan]) -> Cow<'src, str> {
    let spans = self.normalize_spans(spans);

    if spans.is_empty() {
      return Cow::Borrowed(self.content);
    }

    let mut result =
      String::with_capacity(self.content.len() + spans.len() * 10);

    let mut last_end = 0;

    for span in spans {
      if span.start > last_end {
        result.push_str(&self.content[last_end..span.start]);
      }

      result.push_str(span.kind.color());
      result.push_str(&self.content[span.start..span.end]);
      result.push_str(COLOR_RESET);

      last_end = span.end;
    }

    if last_end < self.content.len() {
      result.push_str(&self.content[last_end..]);
    }

    Owned(result)
  }

  fn collect_highlight_spans(&self) -> Vec<HighlightSpan> {
    let mut spans = Vec::new();
    let mut cursor = 0;

    while cursor < self.content.len() {
      let Some(character) = self.content[cursor..].chars().next() else {
        break;
      };

      if character.is_whitespace() {
        cursor += character.len_utf8();
      } else if let Some(end) = self.scan_comment(cursor) {
        spans.push(HighlightSpan::new(cursor, end, HighlightKind::Comment));

        cursor = end;
      } else if character == '_' || character.is_alphabetic() {
        let end = self.scan_identifier(cursor);
        let kind = self.identifier_kind(cursor, end);

        spans.push(HighlightSpan::new(cursor, end, kind));

        cursor = end;
      } else if character.is_ascii_digit() {
        let end = self.scan_number(cursor);

        spans.push(HighlightSpan::new(cursor, end, HighlightKind::Number));

        cursor = end;
      } else if character == '\'' || character == '"' {
        let end = self.scan_string(cursor, character);

        spans.push(HighlightSpan::new(cursor, end, HighlightKind::String));

        cursor = end;
      } else if let Some(end) = self.scan_operator(cursor) {
        spans.push(HighlightSpan::new(cursor, end, HighlightKind::Operator));

        cursor = end;
      } else {
        cursor += character.len_utf8();
      }
    }

    spans
  }

  pub(crate) fn highlight(&self) -> Cow<'src, str> {
    match parse(self.content) {
      Ok(_) => {
        let spans = self.collect_highlight_spans();

        self.apply_color_spans(&spans)
      }
      Err(_) => Owned(format!(
        "{}{}{}",
        HighlightKind::Error.color(),
        self.content,
        COLOR_RESET
      )),
    }
  }

  fn identifier_kind(&self, start: usize, end: usize) -> HighlightKind {
    let token = &self.content[start..end];

    match token {
      "false" | "true" => HighlightKind::Boolean,
      "break" | "continue" | "else" | "fn" | "for" | "if" | "in" | "loop"
      | "null" | "return" | "while" => HighlightKind::Keyword,
      _ if self.next_non_padding_char(end) == Some('(') => {
        HighlightKind::Function
      }
      _ => HighlightKind::Identifier,
    }
  }

  pub(crate) fn new(content: &'src str) -> Self {
    Self { content }
  }

  fn next_non_padding_char(&self, start: usize) -> Option<char> {
    let mut cursor = start;

    loop {
      while let Some(character) = self.content[cursor..].chars().next() {
        if character.is_whitespace() {
          cursor += character.len_utf8();
        } else {
          break;
        }
      }

      if let Some(end) = self.scan_comment(cursor) {
        cursor = end;
      } else {
        break;
      }
    }

    self.content[cursor..].chars().next()
  }

  fn normalize_spans(&self, spans: &[HighlightSpan]) -> Vec<HighlightSpan> {
    let mut spans = spans
      .iter()
      .copied()
      .filter(|span| span.start < span.end && span.end <= self.content.len())
      .collect::<Vec<_>>();

    spans.sort_by_key(|span| (span.start, span.end));

    let mut normalized = Vec::<HighlightSpan>::new();

    for span in spans {
      if let Some(last) = normalized.last_mut() {
        if last.start == span.start && last.end == span.end {
          *last = span;
        } else if span.start < last.end {
          let span = HighlightSpan::new(last.end, span.end, span.kind);

          if span.start < span.end {
            normalized.push(span);
          }
        } else {
          normalized.push(span);
        }
      } else {
        normalized.push(span);
      }
    }

    normalized
  }

  fn scan_comment(&self, start: usize) -> Option<usize> {
    if !self.content[start..].starts_with("//") {
      return None;
    }

    let mut end = start;

    while let Some(character) = self.content[end..].chars().next() {
      if character == '\n' {
        break;
      }

      end += character.len_utf8();
    }

    Some(end)
  }

  fn scan_identifier(&self, start: usize) -> usize {
    let mut end = start;

    while let Some(character) = self.content[end..].chars().next() {
      if character == '_' || character.is_alphanumeric() {
        end += character.len_utf8();
      } else {
        break;
      }
    }

    end
  }

  fn scan_number(&self, start: usize) -> usize {
    let bytes = self.content.as_bytes();

    let mut end = start;

    while end < bytes.len() && bytes[end].is_ascii_digit() {
      end += 1;
    }

    if end + 1 < bytes.len() && bytes[end] == b'.' {
      let mut decimal_end = end + 1;

      while decimal_end < bytes.len() && bytes[decimal_end].is_ascii_digit() {
        decimal_end += 1;
      }

      if decimal_end > end + 1 {
        end = decimal_end;
      }
    }

    end
  }

  fn scan_operator(&self, start: usize) -> Option<usize> {
    for operator in [">=", "<=", "==", "!=", "&&", "||"] {
      if self.content[start..].starts_with(operator) {
        return Some(start + operator.len());
      }
    }

    let character = self.content[start..].chars().next()?;

    matches!(
      character,
      '('
        | ')'
        | '['
        | ']'
        | '{'
        | '}'
        | ','
        | ';'
        | '+'
        | '-'
        | '*'
        | '/'
        | '%'
        | '^'
        | '>'
        | '<'
        | '='
        | '!'
    )
    .then_some(start + character.len_utf8())
  }

  fn scan_string(&self, start: usize, quote: char) -> usize {
    let mut end = start + quote.len_utf8();

    while let Some(character) = self.content[end..].chars().next() {
      end += character.len_utf8();

      if character == quote {
        break;
      }
    }

    end
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn assignment_operator_is_not_index_comparison() {
    let highlighter = Highlighter::new("foo[bar == baz] = 1");

    assert_eq!(
      highlighter.collect_highlight_spans(),
      [
        HighlightSpan::new(0, 3, HighlightKind::Identifier),
        HighlightSpan::new(3, 4, HighlightKind::Operator),
        HighlightSpan::new(4, 7, HighlightKind::Identifier),
        HighlightSpan::new(8, 10, HighlightKind::Operator),
        HighlightSpan::new(11, 14, HighlightKind::Identifier),
        HighlightSpan::new(14, 15, HighlightKind::Operator),
        HighlightSpan::new(16, 17, HighlightKind::Operator),
        HighlightSpan::new(18, 19, HighlightKind::Number),
      ]
    );
  }

  #[test]
  fn comments() {
    let highlighter = Highlighter::new("x = 1 // foo\n// bar");

    assert_eq!(
      highlighter.collect_highlight_spans(),
      [
        HighlightSpan::new(0, 1, HighlightKind::Identifier),
        HighlightSpan::new(2, 3, HighlightKind::Operator),
        HighlightSpan::new(4, 5, HighlightKind::Number),
        HighlightSpan::new(6, 12, HighlightKind::Comment),
        HighlightSpan::new(13, 19, HighlightKind::Comment),
      ]
    );
  }

  #[test]
  fn function_and_identifier_spans_are_direct() {
    let highlighter = Highlighter::new("fn foo(foo) { foo(foo, 1) }");

    assert_eq!(
      highlighter.collect_highlight_spans(),
      [
        HighlightSpan::new(0, 2, HighlightKind::Keyword),
        HighlightSpan::new(3, 6, HighlightKind::Function),
        HighlightSpan::new(6, 7, HighlightKind::Operator),
        HighlightSpan::new(7, 10, HighlightKind::Identifier),
        HighlightSpan::new(10, 11, HighlightKind::Operator),
        HighlightSpan::new(12, 13, HighlightKind::Operator),
        HighlightSpan::new(14, 17, HighlightKind::Function),
        HighlightSpan::new(17, 18, HighlightKind::Operator),
        HighlightSpan::new(18, 21, HighlightKind::Identifier),
        HighlightSpan::new(21, 22, HighlightKind::Operator),
        HighlightSpan::new(23, 24, HighlightKind::Number),
        HighlightSpan::new(24, 25, HighlightKind::Operator),
        HighlightSpan::new(26, 27, HighlightKind::Operator),
      ]
    );
  }

  #[test]
  fn string_contents_are_not_highlighted_as_tokens() {
    let highlighter = Highlighter::new("\"if\" + 'else'");

    assert_eq!(
      highlighter.collect_highlight_spans(),
      [
        HighlightSpan::new(0, 4, HighlightKind::String),
        HighlightSpan::new(5, 6, HighlightKind::Operator),
        HighlightSpan::new(7, 13, HighlightKind::String),
      ]
    );
  }
}
