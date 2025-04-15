use super::*;

#[derive(Clone)]
#[wasm_bindgen]
pub struct Range {
  pub start: u32,
  pub end: u32,
}

impl From<Span> for Range {
  fn from(span: val::Span) -> Self {
    let range = span.into_range();

    Range {
      start: range.start as u32,
      end: range.end as u32,
    }
  }
}

impl From<&Span> for Range {
  fn from(span: &val::Span) -> Self {
    let range = span.into_range();

    Range {
      start: range.start as u32,
      end: range.end as u32,
    }
  }
}
