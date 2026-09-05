use super::*;

pub struct BuiltinFunctionPayload {
  pub arguments: Vec<Value>,
  pub config: Config,
  pub name: &'static str,
  pub span: Span,
}

impl BuiltinFunctionPayload {
  pub(crate) fn format_arguments(&self) -> String {
    self
      .arguments
      .iter()
      .map(|argument| argument.display(self.config))
      .collect::<Vec<_>>()
      .join(" ")
  }

  pub(crate) fn integer(&self, index: usize) -> Result<Integer, Error> {
    self.number(index)?.to_integer().ok_or_else(|| {
      Error::new(
        self.span,
        format!("Arguments to `{}` must be finite integers", self.name),
      )
    })
  }

  pub(crate) fn logarithm_argument(
    &self,
    index: usize,
  ) -> Result<&Number, Error> {
    let number = self.number(index)?;

    if number.is_zero() || number.is_negative() {
      return Err(Error::new(
        self.span,
        "Cannot take logarithm of zero or negative number",
      ));
    }

    Ok(number)
  }

  pub(crate) fn number(&self, index: usize) -> Result<&Number, Error> {
    self.arguments[index].number(self.span)
  }

  pub(crate) fn rational(&self, index: usize) -> Result<&Rational, Error> {
    match self.number(index)? {
      Number::Exact(number) => Ok(number),
      Number::Approx(_) => Err(Error::new(
        self.span,
        format!("Arguments to `{}` must be exact numbers", self.name),
      )),
    }
  }
}
