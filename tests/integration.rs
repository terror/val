use {
  executable_path::executable_path,
  indoc::indoc,
  pretty_assertions::assert_eq,
  std::{fs::File, io::Write, process::Command, str},
  tempfile::TempDir,
  unindent::Unindent,
};

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

struct Test<'a> {
  expected_status: i32,
  expected_stderr: String,
  expected_stdout: String,
  program: &'a str,
  tempdir: TempDir,
}

impl<'a> Test<'a> {
  fn new() -> Result<Self> {
    Ok(Self {
      expected_status: 0,
      expected_stderr: String::new(),
      expected_stdout: String::new(),
      program: "",
      tempdir: TempDir::new()?,
    })
  }

  fn expected_status(self, expected_status: i32) -> Self {
    Self {
      expected_status,
      ..self
    }
  }

  fn expected_stderr(self, expected_stderr: &str) -> Self {
    Self {
      expected_stderr: expected_stderr.unindent(),
      ..self
    }
  }

  fn expected_stdout(self, expected_stdout: &str) -> Self {
    Self {
      expected_stdout: expected_stdout.unindent(),
      ..self
    }
  }

  fn program(self, program: &'a str) -> Self {
    Self { program, ..self }
  }

  fn run(self) -> Result {
    self.run_and_return_tempdir().map(|_| ())
  }

  fn run_and_return_tempdir(self) -> Result<TempDir> {
    let mut command = Command::new(executable_path(env!("CARGO_PKG_NAME")));

    let program_path = self.tempdir.path().join("program.lave");

    let mut file = File::create(&program_path)?;
    write!(file, "{}", self.program.unindent())?;

    command.arg(&program_path);

    let output = command.output().map_err(|e| {
      format!(
        "Failed to execute command `{}`: {}",
        command.get_program().to_string_lossy(),
        e
      )
    })?;

    let stderr = str::from_utf8(&output.stderr)?;

    assert_eq!(stderr, self.expected_stderr, "Stderr mismatch.");

    if self.expected_stderr.is_empty() && !stderr.is_empty() {
      panic!("Expected empty stderr, but received: {}", stderr);
    } else {
      assert_eq!(stderr, self.expected_stderr);
    }

    assert_eq!(str::from_utf8(&output.stdout)?, self.expected_stdout);

    assert_eq!(output.status.code(), Some(self.expected_status));

    Ok(self.tempdir)
  }
}

#[test]
fn integer_literal() -> Result {
  Test::new()?
    .program("25")
    .expected_status(0)
    .expected_stdout("25\n")
    .run()
}

#[test]
fn negate_integer_literal() -> Result {
  Test::new()?
    .program("-25")
    .expected_status(0)
    .expected_stdout("-25\n")
    .run()?;

  Test::new()?
    .program("--25")
    .expected_status(0)
    .expected_stdout("25\n")
    .run()?;

  Test::new()?
    .program("- - - -25")
    .expected_status(0)
    .expected_stdout("25\n")
    .run()
}
