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
  Spanned { error: Box<Self>, span: Span },
  #[error("zero cannot be raised to a negative power")]
  ZeroToNegativePower,
}

impl Error {
  pub fn new(span: impl Into<Span>, message: impl Into<String>) -> Self {
    Self::Message(message.into()).with_span(span)
  }

  #[must_use]
  pub fn report(&self) -> Option<Report<'static, Span>> {
    let span = self.span()?;

    let mut report =
      Report::build(ReportKind::Custom("error", Color::Red), span.clone())
        .with_config(ariadne::Config::new().with_index_type(IndexType::Byte))
        .with_message(self.to_string());

    report = report.with_label(
      Label::new(span.clone())
        .with_message(self.to_string())
        .with_color(Color::Red),
    );

    Some(report.finish())
  }

  #[must_use]
  pub fn span(&self) -> Option<&Span> {
    match self {
      Self::Exit { span, .. } | Self::Spanned { span, .. } => Some(span),
      _ => None,
    }
  }

  #[must_use]
  pub fn with_span(self, span: impl Into<Span>) -> Self {
    let span = span.into();

    match self {
      Self::Exit { code, .. } => Self::Exit { code, span },
      Self::Spanned { error, .. } => Self::Spanned { error, span },
      error => Self::Spanned {
        error: Box::new(error),
        span,
      },
    }
  }
}
