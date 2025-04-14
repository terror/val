use super::*;

#[derive(Debug, PartialEq)]
pub struct Error {
  pub span: Span,
  pub message: String,
}

impl Error {
  pub fn new(span: Span, message: impl Into<String>) -> Self {
    Self {
      span,
      message: message.into(),
    }
  }

  pub fn report<'a>(&self, id: &'a str) -> Report<'a, (&'a str, Range<usize>)> {
    let span_range = self.span.into_range();

    let mut report = Report::build(
      ReportKind::Custom("error", Color::Red),
      (id, span_range.clone()),
    )
    .with_message(&self.message);

    report = report.with_label(
      Label::new((id, span_range))
        .with_message(&self.message)
        .with_color(Color::Red),
    );

    report.finish()
  }
}
