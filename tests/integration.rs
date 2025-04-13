use {
  Match::*,
  executable_path::executable_path,
  pretty_assertions::assert_eq,
  std::{fs::File, io::Write, process::Command, str},
  tempfile::TempDir,
  unindent::Unindent,
};

#[derive(Clone, Debug)]
enum Match<'a> {
  Contains(&'a str),
  Empty,
  Exact(&'a str),
}

type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

struct Test<'a> {
  expected_status: i32,
  expected_stderr: Match<'a>,
  expected_stdout: Match<'a>,
  program: &'a str,
  tempdir: TempDir,
}

impl<'a> Test<'a> {
  fn new() -> Result<Self> {
    Ok(Self {
      expected_status: 0,
      expected_stderr: Match::Empty,
      expected_stdout: Match::Empty,
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

  fn expected_stderr(self, expected_stderr: Match<'a>) -> Self {
    Self {
      expected_stderr,
      ..self
    }
  }

  fn expected_stdout(self, expected_stdout: Match<'a>) -> Self {
    Self {
      expected_stdout,
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

    let program_path = self.tempdir.path().join("program.val");

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

    match &self.expected_stderr {
      Match::Empty => {
        if !stderr.is_empty() {
          panic!("Expected empty stderr, but received: {}", stderr);
        }
      }
      Match::Contains(pattern) => {
        assert!(
          stderr.contains(pattern),
          "Expected stderr to contain: '{}', but got: '{}'",
          pattern,
          stderr
        );
      }
      Match::Exact(expected) => {
        assert_eq!(
          stderr, *expected,
          "Expected exact stderr: '{}', but got: '{}'",
          expected, stderr
        );
      }
    }

    let stdout = str::from_utf8(&output.stdout)?;

    match &self.expected_stdout {
      Match::Empty => {
        if !stdout.is_empty() {
          panic!("Expected empty stdout, but received: {}", stdout);
        }
      }
      Match::Contains(pattern) => {
        assert!(
          stdout.contains(pattern),
          "Expected stdout to contain: '{}', but got: '{}'",
          pattern,
          stdout
        );
      }
      Match::Exact(expected) => {
        assert_eq!(
          stdout, *expected,
          "Expected exact stdout: '{}', but got: '{}'",
          expected, stdout
        );
      }
    }

    assert_eq!(output.status.code(), Some(self.expected_status));

    Ok(self.tempdir)
  }
}

#[test]
fn integer_literals() -> Result {
  Test::new()?
    .program("25")
    .expected_status(0)
    .expected_stdout(Exact("25\n"))
    .run()
}

#[test]
fn float_literals() -> Result {
  Test::new()?
    .program("3.14")
    .expected_status(0)
    .expected_stdout(Exact("3.14\n"))
    .run()?;

  Test::new()?
    .program("0.5")
    .expected_status(0)
    .expected_stdout(Exact("0.5\n"))
    .run()?;

  Test::new()?
    .program("-2.718")
    .expected_status(0)
    .expected_stdout(Exact("-2.718\n"))
    .run()?;

  Test::new()?
    .program("1.0 + 2.5")
    .expected_status(0)
    .expected_stdout(Exact("3.5\n"))
    .run()?;

  Test::new()?
    .program("3.5 * 2.0")
    .expected_status(0)
    .expected_stdout(Exact("7\n"))
    .run()
}

#[test]
fn negate_integer_literal() -> Result {
  Test::new()?
    .program("-25")
    .expected_status(0)
    .expected_stdout(Exact("-25\n"))
    .run()?;

  Test::new()?
    .program("--25")
    .expected_status(0)
    .expected_stdout(Exact("25\n"))
    .run()?;

  Test::new()?
    .program("- - - -25")
    .expected_status(0)
    .expected_stdout(Exact("25\n"))
    .run()
}

#[test]
fn call_builtin_function() -> Result {
  Test::new()?
    .program("sin(1)")
    .expected_status(0)
    .expected_stdout(Exact("0.8414709848078965\n"))
    .run()?;

  Test::new()?
    .program("cos(1)")
    .expected_status(0)
    .expected_stdout(Exact("0.5403023058681398\n"))
    .run()
}

#[test]
fn undefined_variable() -> Result {
  Test::new()?
    .program("foo")
    .expected_status(1)
    .expected_stderr(Contains("Undefined variable 'foo'\n"))
    .run()
}

#[test]
fn addition() -> Result {
  Test::new()?
    .program("2 + 3")
    .expected_status(0)
    .expected_stdout(Exact("5\n"))
    .run()?;

  Test::new()?
    .program("2 + 3 + 4")
    .expected_status(0)
    .expected_stdout(Exact("9\n"))
    .run()?;

  Test::new()?
    .program("-5 + 10")
    .expected_status(0)
    .expected_stdout(Exact("5\n"))
    .run()
}

#[test]
fn subtraction() -> Result {
  Test::new()?
    .program("5 - 3")
    .expected_status(0)
    .expected_stdout(Exact("2\n"))
    .run()?;

  Test::new()?
    .program("10 - 5 - 2")
    .expected_status(0)
    .expected_stdout(Exact("3\n"))
    .run()?;

  Test::new()?
    .program("5 - 10")
    .expected_status(0)
    .expected_stdout(Exact("-5\n"))
    .run()
}

#[test]
fn multiplication() -> Result {
  Test::new()?
    .program("2 * 3")
    .expected_status(0)
    .expected_stdout(Exact("6\n"))
    .run()?;

  Test::new()?
    .program("2 * 3 * 4")
    .expected_status(0)
    .expected_stdout(Exact("24\n"))
    .run()?;

  Test::new()?
    .program("-5 * 10")
    .expected_status(0)
    .expected_stdout(Exact("-50\n"))
    .run()
}

#[test]
fn division() -> Result {
  Test::new()?
    .program("6 / 3")
    .expected_status(0)
    .expected_stdout(Exact("2\n"))
    .run()?;

  Test::new()?
    .program("10 / 2 / 5")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()?;

  Test::new()?
    .program("10 / 4")
    .expected_status(0)
    .expected_stdout(Exact("2.5\n"))
    .run()
}

#[test]
fn division_by_zero() -> Result {
  Test::new()?
    .program("5 / 0")
    .expected_status(1)
    .expected_stderr(Contains("Division by zero"))
    .run()
}

#[test]
fn modulo() -> Result {
  Test::new()?
    .program("7 % 4")
    .expected_status(0)
    .expected_stdout(Exact("3\n"))
    .run()?;

  Test::new()?
    .program("10 % 3")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()
}

#[test]
fn modulo_by_zero() -> Result {
  Test::new()?
    .program("5 % 0")
    .expected_status(1)
    .expected_stderr(Contains("Modulo by zero"))
    .run()
}

#[test]
fn operator_precedence() -> Result {
  Test::new()?
    .program("2 + 3 * 4")
    .expected_status(0)
    .expected_stdout(Exact("14\n"))
    .run()?;

  Test::new()?
    .program("(2 + 3) * 4")
    .expected_status(0)
    .expected_stdout(Exact("20\n"))
    .run()?;

  Test::new()?
    .program("2 * 3 + 4 * 5")
    .expected_status(0)
    .expected_stdout(Exact("26\n"))
    .run()?;

  Test::new()?
    .program("10 - 6 / 2")
    .expected_status(0)
    .expected_stdout(Exact("7\n"))
    .run()
}

#[test]
fn combined_operations() -> Result {
  Test::new()?
    .program("2 + 3 * 4 - 5 / 2")
    .expected_status(0)
    .expected_stdout(Exact("11.5\n"))
    .run()?;

  Test::new()?
    .program("10 % 3 + 4 * 2 - 1")
    .expected_status(0)
    .expected_stdout(Exact("8\n"))
    .run()?;

  Test::new()?
    .program("-5 * (2 + 3) / 5")
    .expected_status(0)
    .expected_stdout(Exact("-5\n"))
    .run()
}

#[test]
fn power() -> Result {
  Test::new()?
    .program("2 ^ 3")
    .expected_status(0)
    .expected_stdout(Exact("8\n"))
    .run()?;

  Test::new()?
    .program("10 ^ 2")
    .expected_status(0)
    .expected_stdout(Exact("100\n"))
    .run()?;

  Test::new()?
    .program("2 ^ -1")
    .expected_status(0)
    .expected_stdout(Exact("0.5\n"))
    .run()?;

  Test::new()?
    .program("2 ^ 0")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()
}

#[test]
fn arctangent() -> Result {
  Test::new()?
    .program("arc(1)")
    .expected_status(0)
    .expected_stdout(Exact("0.7853981633974483\n"))
    .run()?;

  Test::new()?
    .program("arc(0)")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("arc(-1)")
    .expected_status(0)
    .expected_stdout(Exact("-0.7853981633974483\n"))
    .run()?;

  Test::new()?
    .program("arc()")
    .expected_status(1)
    .expected_stderr(Contains("Function 'arc' expects 1 argument, got 0"))
    .run()
}

#[test]
fn natural_logarithm() -> Result {
  Test::new()?
    .program("ln(1)")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("ln(e)")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()?;

  Test::new()?
    .program("ln(10)")
    .expected_status(0)
    .expected_stdout(Exact("2.302585092994046\n"))
    .run()?;

  Test::new()?
    .program("ln()")
    .expected_status(1)
    .expected_stderr(Contains("Function 'log' expects 1 argument, got 0"))
    .run()?;

  Test::new()?
    .program("ln(0)")
    .expected_status(1)
    .expected_stderr(Contains(
      "Cannot take logarithm of zero or negative number",
    ))
    .run()?;

  Test::new()?
    .program("ln(-1)")
    .expected_status(1)
    .expected_stderr(Contains(
      "Cannot take logarithm of zero or negative number",
    ))
    .run()
}

#[test]
fn builtin_variables_and_functions_can_coexist() -> Result {
  Test::new()?
    .program("e * e(20)")
    .expected_status(0)
    .expected_stdout(Exact("1318815734.4832146\n"))
    .run()
}

#[test]
fn functions_with_constants() -> Result {
  Test::new()?
    .program("arc(pi / 4)")
    .expected_status(0)
    .expected_stdout(Exact("0.6657737500283538\n"))
    .run()?;

  Test::new()?
    .program("ln(e * 2)")
    .expected_status(0)
    .expected_stdout(Exact("1.6931471805599452\n"))
    .run()?;

  Test::new()?
    .program("e(pi)")
    .expected_status(0)
    .expected_stdout(Exact("23.140692632779267\n"))
    .run()
}

#[test]
fn square_root() -> Result {
  Test::new()?
    .program("sqrt(4)")
    .expected_status(0)
    .expected_stdout(Exact("2\n"))
    .run()?;

  Test::new()?
    .program("sqrt(2)")
    .expected_status(0)
    .expected_stdout(Exact("1.4142135623730951\n"))
    .run()?;

  Test::new()?
    .program("sqrt(0)")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("sqrt()")
    .expected_status(1)
    .expected_stderr(Contains("Function 'sqrt' expects 1 argument, got 0"))
    .run()?;

  Test::new()?
    .program("sqrt(-1)")
    .expected_status(1)
    .expected_stderr(Contains("Cannot take square root of negative number"))
    .run()
}

#[test]
fn less_than() -> Result {
  Test::new()?
    .program("1 < 2")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("1 < -1")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()
}

#[test]
fn greater_than() -> Result {
  Test::new()?
    .program("1 > 2")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("1 > -1")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn not() -> Result {
  Test::new()?
    .program("!(1 > 2)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("!(1 > -1)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("!true")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("!false")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn string_literals() -> Result {
  Test::new()?
    .program("\"Hello, world!\"")
    .expected_status(0)
    .expected_stdout(Exact("Hello, world!\n"))
    .run()
}
