use super::*;

#[derive(Debug, PartialEq)]
pub struct Error {
  pub message: String,
  pub span: Span,
}

impl Error {
  pub fn new(span: Span, message: impl Into<String>) -> Self {
    Self {
      message: message.into(),
      span,
    }
  }

  #[must_use]
  pub fn report<'a>(&self, id: &'a str) -> Report<'a, (&'a str, Range<usize>)> {
    let span_range = self.span.into_range();

    let mut report = Report::build(
      ReportKind::Custom("error", Color::Red),
      (id, span_range.clone()),
    )
    .with_config(ariadne::Config::new().with_index_type(IndexType::Byte))
    .with_message(&self.message);

    report = report.with_label(
      Label::new((id, span_range))
        .with_message(&self.message)
        .with_color(Color::Red),
    );

    report.finish()
  }
}

#[cfg(test)]
mod tests {
  use {super::*, ariadne::Source};

  #[test]
  fn report_uses_byte_spans() {
    let source = "'é' +* 3";
    let error = parse(source).unwrap_err().into_iter().next().unwrap();
    let mut output = Vec::new();

    error
      .report("foo")
      .write(("foo", Source::from(source)), &mut output)
      .unwrap();

    let output = String::from_utf8(output).unwrap();

    assert!(output.contains("foo:1:6"), "{output}");
  }
}
