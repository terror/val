use super::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Span {
  pub end: usize,
  source: Source,
  pub start: usize,
}

impl Span {
  #[must_use]
  pub fn new(source: Source, range: Range<usize>) -> Self {
    Self {
      end: range.end,
      source,
      start: range.start,
    }
  }

  #[must_use]
  pub fn range(&self) -> Range<usize> {
    self.start..self.end
  }

  #[must_use]
  pub fn source(&self) -> &Source {
    &self.source
  }
}

impl ariadne::Span for Span {
  type SourceId = Source;

  fn end(&self) -> usize {
    self.end
  }

  fn source(&self) -> &Source {
    &self.source
  }

  fn start(&self) -> usize {
    self.start
  }
}

impl chumsky::span::Span for Span {
  type Context = Source;
  type Offset = usize;

  fn context(&self) -> Source {
    self.source.clone()
  }

  fn end(&self) -> usize {
    self.end
  }

  fn new(source: Source, range: Range<usize>) -> Self {
    Self::new(source, range)
  }

  fn start(&self) -> usize {
    self.start
  }
}

impl Display for Span {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}..{}", self.start, self.end)
  }
}

impl From<&Self> for Span {
  fn from(span: &Self) -> Self {
    span.clone()
  }
}
