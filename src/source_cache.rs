use super::*;

#[derive(Default)]
pub struct SourceCache {
  sources: HashMap<Source, ariadne::Source<Source>>,
}

impl ariadne::Cache<Source> for SourceCache {
  type Storage = Source;

  fn display<'a>(&self, source: &'a Source) -> Option<impl Display + 'a> {
    Some(source.name())
  }

  fn fetch(
    &mut self,
    source: &Source,
  ) -> Result<&ariadne::Source<Self::Storage>, impl Debug> {
    Ok::<_, Infallible>(
      self
        .sources
        .entry(source.clone())
        .or_insert_with(|| ariadne::Source::from(source.clone())),
    )
  }
}
