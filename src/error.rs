use super::*;

#[derive(Debug)]
pub(crate) struct Error {
  pub(crate) span: Span,
  pub(crate) message: String,
}

impl Error {
  pub(crate) fn new(span: Span, message: impl Into<String>) -> Self {
    Self {
      span,
      message: message.into(),
    }
  }

  pub(crate) fn report(&self, source_id: &str, source_content: &str) -> Result {
    let span_range = self.span.into_range();

    let mut report = Report::build(
      ReportKind::Custom("error", Color::Red),
      (source_id, span_range.clone()),
    )
    .with_message(&self.message);

    report = report.with_label(
      Label::new((source_id, span_range))
        .with_message(&self.message)
        .with_color(Color::Red),
    );

    report
      .finish()
      .eprint((source_id, Source::from(source_content)))?;

    Ok(())
  }
}
