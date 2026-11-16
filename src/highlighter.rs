use super::*;

const COLOR_RESET: &str = "\x1b[0m";

pub(crate) struct Highlighter<'src> {
  content: &'src str,
}

impl<'src> Highlighter<'src> {
  fn apply_color_spans(&self, spans: &[HighlightSpan]) -> Cow<'src, str> {
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
    let source = Source::from(self.content);

    let (tokens, _) = lex(&source);

    tokens
      .iter()
      .enumerate()
      .map(|(index, (token, span))| {
        let kind = match token {
          Token::Comment(_) => HighlightKind::Comment,
          Token::Identifier(_) => {
            let next = tokens[index + 1..]
              .iter()
              .find(|(token, _)| !matches!(token, Token::Comment(_)));

            if matches!(next, Some((Token::Symbol("("), _))) {
              HighlightKind::Function
            } else {
              HighlightKind::Identifier
            }
          }
          Token::Invalid(_) => HighlightKind::Error,
          Token::Keyword("false" | "true") => HighlightKind::Boolean,
          Token::Keyword(_) => HighlightKind::Keyword,
          Token::Number(_) => HighlightKind::Number,
          Token::String(_) => HighlightKind::String,
          Token::Symbol(_) => HighlightKind::Operator,
        };

        HighlightSpan::new(span.start, span.end, kind)
      })
      .collect()
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

  pub(crate) fn new(content: &'src str) -> Self {
    Self { content }
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
  fn function_call_across_comment() {
    assert_eq!(
      Highlighter::new("foo // bar\n()").collect_highlight_spans(),
      [
        HighlightSpan::new(0, 3, HighlightKind::Function),
        HighlightSpan::new(4, 10, HighlightKind::Comment),
        HighlightSpan::new(11, 12, HighlightKind::Operator),
        HighlightSpan::new(12, 13, HighlightKind::Operator),
      ]
    );
  }

  #[test]
  fn scientific_notation() {
    #[track_caller]
    fn case(source: &str) {
      assert_eq!(
        Highlighter::new(source).collect_highlight_spans(),
        [HighlightSpan::new(0, source.len(), HighlightKind::Number)]
      );
    }

    case("1e-05");
    case("1E+05");
    case("1.5e2");
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
