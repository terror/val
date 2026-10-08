use super::*;

struct SourceData {
  name: String,
  text: String,
}

#[derive(Clone)]
pub struct Source(Arc<SourceData>);

impl Source {
  #[must_use]
  pub fn name(&self) -> &str {
    &self.0.name
  }

  #[must_use]
  pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
    Self(Arc::new(SourceData {
      name: name.into(),
      text: text.into(),
    }))
  }

  #[must_use]
  pub fn span(&self, range: Range<usize>) -> Span {
    Span::new(self.clone(), range)
  }

  #[must_use]
  pub fn text(&self) -> &str {
    &self.0.text
  }
}

impl AsRef<str> for Source {
  fn as_ref(&self) -> &str {
    self.text()
  }
}

impl Debug for Source {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("Source")
      .field("name", &self.name())
      .finish_non_exhaustive()
  }
}

impl Display for Source {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(self.name())
  }
}

impl Eq for Source {}

impl<T: Into<String>> From<T> for Source {
  fn from(text: T) -> Self {
    Self::new("<input>", text)
  }
}

impl Hash for Source {
  fn hash<H: Hasher>(&self, state: &mut H) {
    Arc::as_ptr(&self.0).hash(state);
  }
}

impl PartialEq for Source {
  fn eq(&self, other: &Self) -> bool {
    Arc::ptr_eq(&self.0, &other.0)
  }
}
