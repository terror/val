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

    let mut report =
      Report::build(ReportKind::Error, (source_id, span_range.clone()))
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

pub(crate) fn report_parse_errors(
  source_id: &str,
  source_content: &str,
  errors: &[Rich<'_, char>],
) -> Result {
  for error in errors {
    let span_range = error.span().into_range();

    let mut report =
      Report::build(ReportKind::Error, (error.to_string(), span_range.clone()))
        .with_message(error.to_string());

    report = report.with_label(
      Label::new((source_id.to_owned(), span_range))
        .with_message(error.reason().to_string())
        .with_color(Color::Red),
    );

    for (label_text, span) in error.contexts() {
      report = report.with_label(
        Label::new((source_id.to_owned(), span.into_range()))
          .with_message(format!("while parsing this {}", label_text))
          .with_color(Color::Yellow),
      );
    }

    report
      .finish()
      .eprint((source_id.to_owned(), Source::from(source_content)))?;
  }

  Ok(())
}
