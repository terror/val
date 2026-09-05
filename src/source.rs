use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Source {
  data: Arc<SourceData>,
}

#[derive(Debug, PartialEq)]
struct SourceData {
  name: String,
  text: String,
}

impl Source {
  #[must_use]
  pub fn name(&self) -> &str {
    &self.data.name
  }

  #[must_use]
  pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
    Self {
      data: Arc::new(SourceData {
        name: name.into(),
        text: text.into(),
      }),
    }
  }

  #[must_use]
  pub fn text(&self) -> &str {
    &self.data.text
  }
}
