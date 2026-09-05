use super::*;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
  #[error("division by zero")]
  DivisionByZero,
  #[error("exit requested with code {code}")]
  Exit { code: i32, span: Span },
  #[error("invalid decimal")]
  InvalidDecimal,
  #[error("{0}")]
  Message(String),
  #[error("modulo by zero")]
  ModuloByZero,
  #[error("{error}")]
  Spanned {
    error: Box<Self>,
    origin: Option<Source>,
    span: Span,
  },
  #[error("zero cannot be raised to a negative power")]
  ZeroToNegativePower,
}

impl Error {
  pub fn new(span: Span, message: impl Into<String>) -> Self {
    Self::Message(message.into()).with_span(span)
  }

  #[must_use]
  pub fn origin(&self) -> Option<&Source> {
    match self {
      Self::Spanned { origin, .. } => origin.as_ref(),
      _ => None,
    }
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
      Self::Exit { span, .. } | Self::Spanned { span, .. } => *span,
      _ => Span::from(0..0),
    }
  }

  pub(crate) fn with_source(self, source: Option<&Source>) -> Self {
    match self {
      Self::Spanned {
        error,
        origin: None,
        span,
      } => Self::Spanned {
        error,
        origin: source.cloned(),
        span,
      },
      error => error,
    }
  }

  #[must_use]
  pub fn with_span(self, span: Span) -> Self {
    match self {
      Self::Exit { code, .. } => Self::Exit { code, span },
      Self::Spanned { error, origin, .. } => Self::Spanned {
        error,
        origin,
        span,
      },
      error => Self::Spanned {
        error: Box::new(error),
        origin: None,
        span,
      },
    }
  }
}
