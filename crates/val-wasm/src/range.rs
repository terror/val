use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct Range {
  pub start: u32,
  pub end: u32,
}

impl From<Span> for Range {
  fn from(span: val::Span) -> Self {
    let range = span.into_range();

    Range {
      start: u32::try_from(range.start).unwrap_or(u32::MAX),
      end: u32::try_from(range.end).unwrap_or(u32::MAX),
    }
  }
}

impl From<&Span> for Range {
  fn from(span: &val::Span) -> Self {
    let range = span.into_range();

    Range {
      start: u32::try_from(range.start).unwrap_or(u32::MAX),
      end: u32::try_from(range.end).unwrap_or(u32::MAX),
    }
  }
}
