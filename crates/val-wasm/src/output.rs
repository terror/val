use super::*;

#[derive(Clone, Default)]
pub(crate) struct Output(Rc<RefCell<String>>);

impl Output {
  const LIMIT: usize = 1024 * 1024;

  pub(crate) fn take(&self) -> String {
    self.0.take()
  }
}

impl Write for Output {
  fn flush(&mut self) -> io::Result<()> {
    Ok(())
  }

  fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
    let mut output = self.0.borrow_mut();

    if output.len().saturating_add(buffer.len()) > Self::LIMIT {
      return Err(io::Error::other("Output exceeds 1 MiB"));
    }

    output.push_str(str::from_utf8(buffer).map_err(io::Error::other)?);

    Ok(buffer.len())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn limit() {
    let mut output = Output::default();

    output.write_all(&vec![b'a'; Output::LIMIT]).unwrap();

    assert_eq!(
      output.write_all(b"a").unwrap_err().to_string(),
      "Output exceeds 1 MiB",
    );
  }
}
