use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct HighlightSpan {
  pub(crate) end: usize,
  pub(crate) kind: HighlightKind,
  pub(crate) start: usize,
}

impl HighlightSpan {
  pub(crate) fn new(start: usize, end: usize, kind: HighlightKind) -> Self {
    Self { end, kind, start }
  }
}
