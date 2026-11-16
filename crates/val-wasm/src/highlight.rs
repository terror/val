use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Tsify)]
pub enum HighlightKind {
  Boolean,
  Comment,
  Function,
  Identifier,
  Invalid,
  Keyword,
  Number,
  Operator,
  Punctuation,
  String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Tsify)]
pub struct Highlight {
  pub kind: HighlightKind,
  pub range: Range,
}

#[derive(Debug, Serialize, Tsify)]
pub struct Highlights(Vec<Highlight>);

impl Highlights {
  pub(crate) fn new(source: &str) -> Self {
    let source = Source::from(source);

    let (tokens, _) = val::lex(&source);

    let converter = RangeConverter::new(source.text());

    let highlights = tokens
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
          Token::Invalid(_) => HighlightKind::Invalid,
          Token::Keyword("false" | "true") => HighlightKind::Boolean,
          Token::Keyword(_) => HighlightKind::Keyword,
          Token::Number(_) => HighlightKind::Number,
          Token::String(_) => HighlightKind::String,
          Token::Symbol("(" | ")" | "[" | "]" | "{" | "}" | "," | ";") => {
            HighlightKind::Punctuation
          }
          Token::Symbol(_) => HighlightKind::Operator,
        };

        Highlight {
          kind,
          range: converter.convert(span),
        }
      })
      .collect();

    Self(highlights)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    fn case(kind: HighlightKind, start: u32, end: u32) -> Highlight {
      Highlight {
        kind,
        range: Range { start, end },
      }
    }

    assert_eq!(
      Highlights::new("'😀' foo(true) // bar\n@").0,
      [
        case(HighlightKind::String, 0, 4),
        case(HighlightKind::Function, 5, 8),
        case(HighlightKind::Punctuation, 8, 9),
        case(HighlightKind::Boolean, 9, 13),
        case(HighlightKind::Punctuation, 13, 14),
        case(HighlightKind::Comment, 15, 21),
        case(HighlightKind::Invalid, 22, 23),
      ],
    );
  }
}
