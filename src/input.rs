use super::*;

pub(crate) struct Input<'a> {
  pub(crate) name: &'a str,
  pub(crate) text: &'a str,
}

impl Input<'_> {
  pub(crate) fn evaluate<'a>(
    &self,
    evaluator: &mut Evaluator<'a>,
  ) -> Result<Evaluation<'a>, Vec<Error>> {
    let ast = parse(self.text)?;

    evaluator.evaluate(&ast).map_err(|error| vec![error])
  }

  pub(crate) fn report(
    &self,
    errors: &[Error],
    mut writer: impl Write,
  ) -> io::Result<()> {
    for error in errors {
      error
        .report(self.name)
        .write((self.name, Source::from(self.text)), &mut writer)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn report() {
    let input = Input {
      name: "foo",
      text: "bar baz",
    };

    let errors = [
      Error::new((0..3).into(), "qux"),
      Error::new((4..7).into(), "quux"),
    ];

    let mut output = Vec::new();

    input.report(&errors, &mut output).unwrap();

    let output = String::from_utf8(output).unwrap();

    assert!(output.contains("foo:1:1"));
    assert!(output.contains("foo:1:5"));
    assert!(output.contains("qux"));
    assert!(output.contains("quux"));
  }

  #[test]
  fn report_io_error() {
    let input = Input {
      name: "foo",
      text: "bar",
    };

    assert_eq!(
      input
        .report(&[Error::new((0..3).into(), "baz")], &mut [][..])
        .unwrap_err()
        .kind(),
      io::ErrorKind::WriteZero,
    );
  }
}
