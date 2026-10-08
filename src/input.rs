use super::*;

pub(crate) struct Input<'a> {
  pub(crate) name: &'a str,
  pub(crate) text: &'a str,
}

impl Input<'_> {
  pub(crate) fn evaluate(
    &self,
    evaluator: &mut Evaluator,
  ) -> Result<Evaluation, Vec<Error>> {
    let ast = parse(Source::new(self.name, self.text))?;

    evaluator.evaluate(&ast).map_err(|error| vec![error])
  }

  pub(crate) fn report(
    errors: &[Error],
    mut writer: impl Write,
  ) -> io::Result<()> {
    let mut cache = SourceCache::default();

    for error in errors {
      if let Some(report) = error.report() {
        report.write(&mut cache, &mut writer)?;
      } else {
        writeln!(writer, "error: {error}")?;
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn report() {
    let source = Source::new("foo", "bar baz");

    let errors = [
      Error::new(source.span(0..3), "qux"),
      Error::new(source.span(4..7), "quux"),
    ];

    let mut output = Vec::new();

    Input::report(&errors, &mut output).unwrap();

    let output = String::from_utf8(output).unwrap();

    assert!(output.contains("foo:1:1"));
    assert!(output.contains("foo:1:5"));
    assert!(output.contains("qux"));
    assert!(output.contains("quux"));
  }

  #[test]
  fn report_io_error() {
    let source = Source::new("foo", "bar");

    assert_eq!(
      Input::report(&[Error::new(source.span(0..3), "baz")], &mut [][..])
        .unwrap_err()
        .kind(),
      io::ErrorKind::WriteZero,
    );
  }
}
