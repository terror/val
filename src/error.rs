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
    let span_range = self.span().into_range();

    let mut report = Report::build(
      ReportKind::Custom("error", Color::Red),
      (id, span_range.clone()),
    )
    .with_config(ariadne::Config::new().with_index_type(IndexType::Byte))
    .with_message(self.to_string());

    report = report.with_label(
      Label::new((id, span_range))
        .with_message(self.to_string())
        .with_color(Color::Red),
    );

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
