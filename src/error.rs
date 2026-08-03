use super::*;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
  #[error("division by zero")]
  DivisionByZero,
  #[error("invalid decimal")]
  InvalidDecimal,
  #[error("{0}")]
  Message(String),
  #[error("modulo by zero")]
  ModuloByZero,
  #[error("{error}")]
  Spanned { error: Box<Self>, span: Span },
  #[error("zero cannot be raised to a negative power")]
  ZeroToNegativePower,
}

impl Error {
  pub fn new(span: Span, message: impl Into<String>) -> Self {
    Self::Message(message.into()).with_span(span)
  }

  #[must_use]
  pub fn report<'a>(&self, id: &'a str) -> Report<'a, (&'a str, Range<usize>)> {
    let no_color =
      env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty());

    self.report_with_color(id, io::stderr().is_terminal() && !no_color)
  }

  #[must_use]
  pub fn report_with_color<'a>(
    &self,
    id: &'a str,
    color: bool,
  ) -> Report<'a, (&'a str, Range<usize>)> {
    let span_range = self.span().into_range();

    let mut report = Report::build(
      if color {
        ReportKind::Custom("error", Color::Red)
      } else {
        ReportKind::Error
      },
      (id, span_range.clone()),
    )
    .with_config(
      ariadne::Config::new()
        .with_color(color)
        .with_index_type(IndexType::Byte),
    )
    .with_message(self.to_string());

    let label = Label::new((id, span_range)).with_message(self.to_string());
    let label = if color {
      label.with_color(Color::Red)
    } else {
      label
    };

    report = report.with_label(label);

    report.finish()
  }

  #[must_use]
  pub fn span(&self) -> Span {
    match self {
      Self::Spanned { span, .. } => *span,
      _ => Span::from(0..0),
    }
  }

  #[must_use]
  pub fn with_span(self, span: Span) -> Self {
    match self {
      Self::Spanned { error, .. } => Self::Spanned { error, span },
      error => Self::Spanned {
        error: Box::new(error),
        span,
      },
    }
  }
}
