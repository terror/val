#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HighlightKind {
  Boolean,
  Comment,
  Error,
  Function,
  Identifier,
  Keyword,
  Number,
  Operator,
  String,
}

impl HighlightKind {
  pub(crate) fn color(self) -> &'static str {
    match self {
      Self::Boolean | Self::Number => "\x1b[33m",
      Self::Comment => "\x1b[90m",
      Self::Error => "\x1b[31m",
      Self::Function => "\x1b[34m",
      Self::Identifier => "\x1b[37m",
      Self::Keyword => "\x1b[35m",
      Self::Operator => "\x1b[36m",
      Self::String => "\x1b[32m",
    }
  }
}
