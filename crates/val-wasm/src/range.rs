use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct Range {
  pub start: u32,
  pub end: u32,
}

pub(crate) struct RangeConverter {
  offsets: Vec<u32>,
}

impl RangeConverter {
  pub(crate) fn convert(&self, range: Range) -> Range {
    let last = self.offsets.last().copied().unwrap_or_default();

    Range {
      start: self
        .offsets
        .get(range.start as usize)
        .copied()
        .unwrap_or(last),
      end: self
        .offsets
        .get(range.end as usize)
        .copied()
        .unwrap_or(last),
    }
  }

  pub(crate) fn new(input: &str) -> Self {
    let mut offsets = vec![0; input.len() + 1];
    let mut utf16 = 0_u32;

    for (byte, character) in input.char_indices() {
      let end = byte + character.len_utf8();

      offsets[byte..end].fill(utf16);

      utf16 = utf16.saturating_add(
        u32::try_from(character.len_utf16()).unwrap_or(u32::MAX),
      );

      offsets[end] = utf16;
    }

    Self { offsets }
  }
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn converts_utf8_bytes_to_utf16_units() {
    #[track_caller]
    fn case(input: &str, span: std::ops::Range<usize>, expected: Range) {
      let converter = RangeConverter::new(input);
      let actual = converter.convert(Range::from(Span::from(span)));

      assert_eq!(actual, expected);
    }

    case("foo", 1..3, Range { start: 1, end: 3 });
    case("é foo", 3..6, Range { start: 2, end: 5 });
    case("😀 foo", 5..8, Range { start: 3, end: 6 });
  }
}
