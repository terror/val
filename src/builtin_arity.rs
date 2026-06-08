use super::*;

#[derive(Clone, Copy, Debug)]
pub enum BuiltinArity {
  Any,
  Exact(usize),
  Range(usize, usize),
}

impl BuiltinArity {
  fn accepts(self, len: usize) -> bool {
    match self {
      Self::Any => true,
      Self::Exact(expected) => len == expected,
      Self::Range(min, max) => len >= min && len <= max,
    }
  }

  pub(crate) fn check(
    self,
    name: &str,
    len: usize,
    span: Span,
  ) -> Result<(), Error> {
    if self.accepts(len) {
      return Ok(());
    }

    Err(Error::new(
      span,
      format!("Function `{name}` expects {}, got {len}", self.expected()),
    ))
  }

  fn expected(self) -> String {
    match self {
      Self::Any => "any number of arguments".into(),
      Self::Exact(1) => "1 argument".into(),
      Self::Exact(expected) => format!("{expected} arguments"),
      Self::Range(min, max) if min + 1 == max => {
        format!("{min} or {max} arguments")
      }
      Self::Range(min, max) => format!("{min} to {max} arguments"),
    }
  }
}
