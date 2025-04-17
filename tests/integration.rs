use {
  Match::*,
  executable_path::executable_path,
  indoc::indoc,
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
    .program("println(25)")
    .expected_status(0)
    .expected_stdout(Exact("25\n"))
    .run()
}

#[test]
fn float_literals() -> Result {
  Test::new()?
    .program("println(3.14)")
    .expected_status(0)
    .expected_stdout(Exact("3.14\n"))
    .run()?;

  Test::new()?
    .program("println(0.5)")
    .expected_status(0)
    .expected_stdout(Exact("0.5\n"))
    .run()?;

  Test::new()?
    .program("println(-2.718)")
    .expected_status(0)
    .expected_stdout(Exact("-2.718\n"))
    .run()?;

  Test::new()?
    .program("println(1.0 + 2.5)")
    .expected_status(0)
    .expected_stdout(Exact("3.5\n"))
    .run()?;

  Test::new()?
    .program("println(3.5 * 2.0)")
    .expected_status(0)
    .expected_stdout(Exact("7\n"))
    .run()
}

#[test]
fn negate_integer_literal() -> Result {
  Test::new()?
    .program("println(-25)")
    .expected_status(0)
    .expected_stdout(Exact("-25\n"))
    .run()?;

  Test::new()?
    .program("println(--25)")
    .expected_status(0)
    .expected_stdout(Exact("25\n"))
    .run()?;

  Test::new()?
    .program("println(- - - -25)")
    .expected_status(0)
    .expected_stdout(Exact("25\n"))
    .run()
}

#[test]
fn call_builtin_function() -> Result {
  Test::new()?
    .program("println(sin(1))")
    .expected_status(0)
    .expected_stdout(Exact("0.8414709848078965\n"))
    .run()?;

  Test::new()?
    .program("println(cos(1))")
    .expected_status(0)
    .expected_stdout(Exact("0.5403023058681398\n"))
    .run()
}

#[test]
fn undefined_variable() -> Result {
  Test::new()?
    .program("println(foo)")
    .expected_status(1)
    .expected_stderr(Contains("Undefined variable `foo`\n"))
    .run()
}

#[test]
fn addition() -> Result {
  Test::new()?
    .program("println(2 + 3)")
    .expected_status(0)
    .expected_stdout(Exact("5\n"))
    .run()?;

  Test::new()?
    .program("println(2 + 3 + 4)")
    .expected_status(0)
    .expected_stdout(Exact("9\n"))
    .run()?;

  Test::new()?
    .program("println(-5 + 10)")
    .expected_status(0)
    .expected_stdout(Exact("5\n"))
    .run()
}

#[test]
fn subtraction() -> Result {
  Test::new()?
    .program("println(5 - 3)")
    .expected_status(0)
    .expected_stdout(Exact("2\n"))
    .run()?;

  Test::new()?
    .program("println(10 - 5 - 2)")
    .expected_status(0)
    .expected_stdout(Exact("3\n"))
    .run()?;

  Test::new()?
    .program("println(5 - 10)")
    .expected_status(0)
    .expected_stdout(Exact("-5\n"))
    .run()
}

#[test]
fn multiplication() -> Result {
  Test::new()?
    .program("println(2 * 3)")
    .expected_status(0)
    .expected_stdout(Exact("6\n"))
    .run()?;

  Test::new()?
    .program("println(2 * 3 * 4)")
    .expected_status(0)
    .expected_stdout(Exact("24\n"))
    .run()?;

  Test::new()?
    .program("println(-5 * 10)")
    .expected_status(0)
    .expected_stdout(Exact("-50\n"))
    .run()
}

#[test]
fn division() -> Result {
  Test::new()?
    .program("println(6 / 3)")
    .expected_status(0)
    .expected_stdout(Exact("2\n"))
    .run()?;

  Test::new()?
    .program("println(10 / 2 / 5)")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()?;

  Test::new()?
    .program("println(10 / 4)")
    .expected_status(0)
    .expected_stdout(Exact("2.5\n"))
    .run()
}

#[test]
fn division_by_zero() -> Result {
  Test::new()?
    .program("println(5 / 0)")
    .expected_status(1)
    .expected_stderr(Contains("Division by zero"))
    .run()
}

#[test]
fn modulo() -> Result {
  Test::new()?
    .program("println(7 % 4)")
    .expected_status(0)
    .expected_stdout(Exact("3\n"))
    .run()?;

  Test::new()?
    .program("println(10 % 3)")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()
}

#[test]
fn modulo_by_zero() -> Result {
  Test::new()?
    .program("println(5 % 0)")
    .expected_status(1)
    .expected_stderr(Contains("Modulo by zero"))
    .run()
}

#[test]
fn operator_precedence() -> Result {
  Test::new()?
    .program("println(2 + 3 * 4)")
    .expected_status(0)
    .expected_stdout(Exact("14\n"))
    .run()?;

  Test::new()?
    .program("println((2 + 3) * 4)")
    .expected_status(0)
    .expected_stdout(Exact("20\n"))
    .run()?;

  Test::new()?
    .program("println(2 * 3 + 4 * 5)")
    .expected_status(0)
    .expected_stdout(Exact("26\n"))
    .run()?;

  Test::new()?
    .program("println(10 - 6 / 2)")
    .expected_status(0)
    .expected_stdout(Exact("7\n"))
    .run()
}

#[test]
fn combined_operations() -> Result {
  Test::new()?
    .program("println(2 + 3 * 4 - 5 / 2)")
    .expected_status(0)
    .expected_stdout(Exact("11.5\n"))
    .run()?;

  Test::new()?
    .program("println(10 % 3 + 4 * 2 - 1)")
    .expected_status(0)
    .expected_stdout(Exact("8\n"))
    .run()?;

  Test::new()?
    .program("println(-5 * (2 + 3) / 5)")
    .expected_status(0)
    .expected_stdout(Exact("-5\n"))
    .run()
}

#[test]
fn power() -> Result {
  Test::new()?
    .program("println(2 ^ 3)")
    .expected_status(0)
    .expected_stdout(Exact("8\n"))
    .run()?;

  Test::new()?
    .program("println(10 ^ 2)")
    .expected_status(0)
    .expected_stdout(Exact("100\n"))
    .run()?;

  Test::new()?
    .program("println(2 ^ -1)")
    .expected_status(0)
    .expected_stdout(Exact("0.5\n"))
    .run()?;

  Test::new()?
    .program("println(2 ^ 0)")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()
}

#[test]
fn arctangent() -> Result {
  Test::new()?
    .program("println(arc(1))")
    .expected_status(0)
    .expected_stdout(Exact("0.7853981633974483\n"))
    .run()?;

  Test::new()?
    .program("println(arc(0))")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("println(arc(-1))")
    .expected_status(0)
    .expected_stdout(Exact("-0.7853981633974483\n"))
    .run()?;

  Test::new()?
    .program("println(arc())")
    .expected_status(1)
    .expected_stderr(Contains("Function `arc` expects 1 argument, got 0"))
    .run()
}

#[test]
fn tangent() -> Result {
  Test::new()?
    .program("println(tan(1))")
    .expected_status(0)
    .expected_stdout(Contains("1.557407724654902"))
    .run()?;

  Test::new()?
    .program("println(tan(0))")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("println(tan(pi/4))")
    .expected_status(0)
    .expected_stdout(Contains("0.99"))
    .run()
}

#[test]
fn cosecant() -> Result {
  Test::new()?
    .program("println(csc(1))")
    .expected_status(0)
    .expected_stdout(Contains("1.188"))  // ~1.1883951...
    .run()?;

  Test::new()?
    .program("println(csc(pi/2))")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()?;

  Test::new()?
    .program("println(csc(0))")
    .expected_status(1)
    .expected_stderr(Contains("Cannot compute csc of multiple of π"))
    .run()
}

#[test]
fn secant() -> Result {
  Test::new()?
    .program("println(sec(0))")
    .expected_status(0)
    .expected_stdout(Contains("1\n"))
    .run()?;

  Test::new()?
    .program("println(sec(pi/3))")
    .expected_status(0)
    .expected_stdout(Contains("1.99"))  // ~2.0000...
    .run()
}

#[test]
fn cotangent() -> Result {
  Test::new()?
    .program("println(cot(1))")
    .expected_status(0)
    .expected_stdout(Contains("0.64"))  // ~0.6420...
    .run()?;

  Test::new()?
    .program("println(cot(pi/4))")
    .expected_status(0)
    .expected_stdout(Contains("1.0000"))
    .run()?;

  Test::new()?
    .program("println(cot(0))")
    .expected_status(1)
    .expected_stderr(Contains("Cannot compute cot of multiple of π"))
    .run()
}

#[test]
fn hyperbolic_functions() -> Result {
  Test::new()?
    .program("println(sinh(1))")
    .expected_status(0)
    .expected_stdout(Contains("1.17520"))  // ~1.1752...
    .run()?;

  Test::new()?
    .program("println(cosh(0))")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()?;

  Test::new()?
    .program("println(tanh(2))")
    .expected_status(0)
    .expected_stdout(Contains("0.96"))  // ~0.964...
    .run()
}

#[test]
fn arcsine() -> Result {
  Test::new()?
    .program("println(asin(0))")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("println(asin(1))")
    .expected_status(0)
    .expected_stdout(Contains("1.57"))  // π/2 ~= 1.5708...
    .run()?;

  Test::new()?
    .program("println(asin(0.5))")
    .expected_status(0)
    .expected_stdout(Contains("0.52"))  // ~0.5235...
    .run()?;

  Test::new()?
    .program("println(asin(2))")
    .expected_status(1)
    .expected_stderr(Contains("asin argument must be between -1 and 1"))
    .run()
}

#[test]
fn arccosine() -> Result {
  Test::new()?
    .program("println(acos(1))")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("println(acos(0))")
    .expected_status(0)
    .expected_stdout(Contains("1.57"))  // π/2 ~= 1.5708...
    .run()?;

  Test::new()?
    .program("println(acos(0.5))")
    .expected_status(0)
    .expected_stdout(Contains("1.04"))  // ~1.0472...
    .run()?;

  Test::new()?
    .program("println(acos(-2))")
    .expected_status(1)
    .expected_stderr(Contains("acos argument must be between -1 and 1"))
    .run()
}

#[test]
fn arccosecant() -> Result {
  Test::new()?
    .program("println(acsc(1))")
    .expected_status(0)
    .expected_stdout(Contains("1.57"))  // π/2 ~= 1.5708...
    .run()?;

  Test::new()?
    .program("println(acsc(2))")
    .expected_status(0)
    .expected_stdout(Contains("0.52"))  // ~0.5235...
    .run()?;

  Test::new()?
    .program("println(acsc(0.5))")
    .expected_status(1)
    .expected_stderr(Contains(
      "acsc argument must have absolute value at least 1",
    ))
    .run()
}

#[test]
fn arcsecant() -> Result {
  Test::new()?
    .program("println(asec(1))")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("println(asec(2))")
    .expected_status(0)
    .expected_stdout(Contains("1.04"))  // ~1.0472...
    .run()?;

  Test::new()?
    .program("println(asec(0.5))")
    .expected_status(1)
    .expected_stderr(Contains(
      "asec argument must have absolute value at least 1",
    ))
    .run()
}

#[test]
fn arccotangent() -> Result {
  Test::new()?
    .program("println(acot(1))")
    .expected_status(0)
    .expected_stdout(Contains("0.78"))  // π/4 ~= 0.7853...
    .run()?;

  Test::new()?
    .program("println(acot(0))")
    .expected_status(0)
    .expected_stdout(Contains("1.57"))  // π/2 ~= 1.5708...
    .run()
}

#[test]
fn logarithms() -> Result {
  Test::new()?
    .program("println(log2(8))")
    .expected_status(0)
    .expected_stdout(Exact("3\n"))
    .run()?;

  Test::new()?
    .program("println(log2(0))")
    .expected_status(1)
    .expected_stderr(Contains(
      "Cannot take logarithm of zero or negative number",
    ))
    .run()?;

  Test::new()?
    .program("println(log10(100))")
    .expected_status(0)
    .expected_stdout(Exact("2\n"))
    .run()?;

  Test::new()?
    .program("println(log10(-5))")
    .expected_status(1)
    .expected_stderr(Contains(
      "Cannot take logarithm of zero or negative number",
    ))
    .run()
}

#[test]
fn ceiling_function() -> Result {
  Test::new()?
    .program("println(ceil(3.14))")
    .expected_status(0)
    .expected_stdout(Exact("4\n"))
    .run()?;

  Test::new()?
    .program("println(ceil(-2.5))")
    .expected_status(0)
    .expected_stdout(Exact("-2\n"))
    .run()?;

  Test::new()?
    .program("println(ceil(5.0))")
    .expected_status(0)
    .expected_stdout(Exact("5\n"))
    .run()
}

#[test]
fn floor_function() -> Result {
  Test::new()?
    .program("println(floor(3.14))")
    .expected_status(0)
    .expected_stdout(Exact("3\n"))
    .run()?;

  Test::new()?
    .program("println(floor(-2.5))")
    .expected_status(0)
    .expected_stdout(Exact("-3\n"))
    .run()?;

  Test::new()?
    .program("println(floor(5.0))")
    .expected_status(0)
    .expected_stdout(Exact("5\n"))
    .run()
}

#[test]
fn absolute_value() -> Result {
  Test::new()?
    .program("println(abs(3.14))")
    .expected_status(0)
    .expected_stdout(Exact("3.14\n"))
    .run()?;

  Test::new()?
    .program("println(abs(-2.5))")
    .expected_status(0)
    .expected_stdout(Exact("2.5\n"))
    .run()?;

  Test::new()?
    .program("println(abs(0))")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()
}

#[test]
fn natural_logarithm() -> Result {
  Test::new()?
    .program("println(ln(1))")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("println(ln(e))")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()?;

  Test::new()?
    .program("println(ln(10))")
    .expected_status(0)
    .expected_stdout(Exact("2.302585092994046\n"))
    .run()?;

  Test::new()?
    .program("println(ln())")
    .expected_status(1)
    .expected_stderr(Contains("Function 'ln' expects 1 argument, got 0"))
    .run()?;

  Test::new()?
    .program("println(ln(0))")
    .expected_status(1)
    .expected_stderr(Contains(
      "Cannot take logarithm of zero or negative number",
    ))
    .run()?;

  Test::new()?
    .program("println(ln(-1))")
    .expected_status(1)
    .expected_stderr(Contains(
      "Cannot take logarithm of zero or negative number",
    ))
    .run()
}

#[test]
fn builtin_variables_and_functions_can_coexist() -> Result {
  Test::new()?
    .program("println(e * e(20))")
    .expected_status(0)
    .expected_stdout(Exact("1318815734.4832146\n"))
    .run()
}

#[test]
fn functions_with_constants() -> Result {
  Test::new()?
    .program("println(arc(pi / 4))")
    .expected_status(0)
    .expected_stdout(Exact("0.6657737500283538\n"))
    .run()?;

  Test::new()?
    .program("println(ln(e * 2))")
    .expected_status(0)
    .expected_stdout(Exact("1.6931471805599452\n"))
    .run()?;

  Test::new()?
    .program("println(e(pi))")
    .expected_status(0)
    .expected_stdout(Exact("23.140692632779267\n"))
    .run()
}

#[test]
fn square_root() -> Result {
  Test::new()?
    .program("println(sqrt(4))")
    .expected_status(0)
    .expected_stdout(Exact("2\n"))
    .run()?;

  Test::new()?
    .program("println(sqrt(2))")
    .expected_status(0)
    .expected_stdout(Exact("1.4142135623730951\n"))
    .run()?;

  Test::new()?
    .program("println(sqrt(0))")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("println(sqrt())")
    .expected_status(1)
    .expected_stderr(Contains("Function `sqrt` expects 1 argument, got 0"))
    .run()?;

  Test::new()?
    .program("println(sqrt(-1))")
    .expected_status(1)
    .expected_stderr(Contains("Cannot take square root of negative number"))
    .run()
}

#[test]
fn less_than() -> Result {
  Test::new()?
    .program("println(1 < 2)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(1 < -1)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()
}

#[test]
fn greater_than() -> Result {
  Test::new()?
    .program("println(1 > 2)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(1 > -1)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn not() -> Result {
  Test::new()?
    .program("println(!(1 > 2))")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(!(1 > -1))")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(!true)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(!false)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn string_literals() -> Result {
  Test::new()?
    .program("println(\"Hello, world!\")")
    .expected_status(0)
    .expected_stdout(Exact("Hello, world!\n"))
    .run()?;

  Test::new()?
    .program("println('Hello, world!')")
    .expected_status(0)
    .expected_stdout(Exact("Hello, world!\n"))
    .run()
}

#[test]
fn len() -> Result {
  Test::new()?
    .program("println(len(\"Hello, world!\"))")
    .expected_status(0)
    .expected_stdout(Exact("13\n"))
    .run()
}

#[test]
fn exit_or_quit() -> Result {
  Test::new()?.program("exit()").expected_status(0).run()?;
  Test::new()?.program("quit()").expected_status(0).run()?;

  Test::new()?.program("exit(1)").expected_status(1).run()?;
  Test::new()?.program("quit(1)").expected_status(1).run()?;

  Test::new()?
    .program("exit(1, 2)")
    .expected_status(1)
    .expected_stderr(Contains(
      "Function `exit` expects 0 or 1 arguments, got 2",
    ))
    .run()?;

  Test::new()?
    .program("quit(1, 2)")
    .expected_status(1)
    .expected_stderr(Contains(
      "Function `quit` expects 0 or 1 arguments, got 2",
    ))
    .run()
}

#[test]
fn equal_to() -> Result {
  Test::new()?
    .program("println(1 == 1)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(1 == 2)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(\"hello\" == \"hello\")")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(\"hello\" == \"world\")")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()
}

#[test]
fn not_equal_to() -> Result {
  Test::new()?
    .program("println(1 != 1)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(1 != 2)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(\"hello\" != \"hello\")")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(\"hello\" != \"world\")")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn less_than_or_equal() -> Result {
  Test::new()?
    .program("println(1 <= 1)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(1 <= 2)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(2 <= 1)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()
}

#[test]
fn greater_than_or_equal() -> Result {
  Test::new()?
    .program("println(1 >= 1)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(1 >= 2)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(2 >= 1)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn comparison_with_expressions() -> Result {
  Test::new()?
    .program("println((1 + 2) == (4 - 1))")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(2 * 3 >= 5)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(10 / 2 <= 4)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(2 ^ 3 != 9)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn boolean_comparison() -> Result {
  Test::new()?
    .program("println(true == true)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(true != false)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println((1 < 2) == true)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn mixed_type_comparisons() -> Result {
  Test::new()?
    .program("println(\"true\" == true)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(\"false\" != false)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(1 == true)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(0 == false)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(123 == \"123\")")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(0 != \"0\")")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn list_literals() -> Result {
  Test::new()?
    .program("println([1, 2, 1 + 2])")
    .expected_status(0)
    .expected_stdout(Exact("[1, 2, 3]\n"))
    .run()?;

  Test::new()?
    .program("println([println('foo'), 'foo', 1 + 2])")
    .expected_status(0)
    .expected_stdout(Exact("foo\n[null, 'foo', 3]\n"))
    .run()
}

#[test]
fn assignment() -> Result {
  Test::new()?
    .program(indoc! {
      "
      a = 1
      println(a)

      a = [1, 2, 3]
      println(a)
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("1\n[1, 2, 3]\n"))
    .run()
}

#[test]
fn list_access() -> Result {
  Test::new()?
    .program(indoc! {
      "
      a = [1, 2, 3]
      println(a[1])

      a = [1, 2, 3]
      println(a[1 + 1])
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("2\n3\n"))
    .run()
}

#[test]
fn list_access_out_of_bounds() -> Result {
  Test::new()?
    .program(indoc! {
      "
      a = [1, 2, 3]
      println(a[20])
      "
    })
    .expected_status(1)
    .expected_stderr(Contains("Index 20 out of bounds for list of length 3"))
    .run()
}

#[test]
fn while_loops() -> Result {
  Test::new()?
    .program(indoc! {
      "
      counter = 0
      sum = 0

      while (counter < 5) {
        sum = sum + counter
        counter = counter + 1
      }

      println(sum)
      println(counter)
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("10\n5\n"))
    .run()
}

#[test]
fn nested_while_loops() -> Result {
  Test::new()?
    .program(indoc! {
      "
      a = 0

      while (a < 5) {
        b = 0

        while (b < 1) {
          println(a + b)
          b = b + 1
        }

        a = a + 1
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("0\n1\n2\n3\n4\n"))
    .run()
}

#[test]
fn if_statement_true_condition() -> Result {
  Test::new()?
    .program(indoc! {
      "
      x = 10
      if (x > 5) {
        println('greater')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("greater\n"))
    .run()
}

#[test]
fn if_statement_false_condition() -> Result {
  Test::new()?
    .program(indoc! {
      "
      x = 3
      if (x > 5) {
        println('greater')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Empty)
    .run()
}

#[test]
fn if_else_statement_true_condition() -> Result {
  Test::new()?
    .program(indoc! {
      "
      x = 10
      if (x > 5) {
        println('greater')
      } else {
        println('not greater')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("greater\n"))
    .run()
}

#[test]
fn if_else_statement_false_condition() -> Result {
  Test::new()?
    .program(indoc! {
      "
      x = 3
      if (x > 5) {
        println('greater')
      } else {
        println('not greater')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("not greater\n"))
    .run()
}

#[test]
fn if_statement_with_expression() -> Result {
  Test::new()?
    .program(indoc! {
      "
      x = 3
      y = 7
      if (x + y > 8) {
        println('sum is greater than 8')
      } else {
        println('sum is not greater than 8')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("sum is greater than 8\n"))
    .run()
}

#[test]
fn nested_if_statements() -> Result {
  Test::new()?
    .program(indoc! {
      "
      x = 10
      y = 5

      if (x > 5) {
        if (y > 3) {
          println('both conditions met')
        } else {
          println('only x condition met')
        }
      } else {
        println('x condition not met')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("both conditions met\n"))
    .run()
}

#[test]
fn if_statement_with_variable_assignment() -> Result {
  Test::new()?
    .program(indoc! {
      "
      x = 10

      if (x > 5) {
        result = 'greater'
      } else {
        result = 'not greater'
      }

      println(result)
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("greater\n"))
    .run()
}

#[test]
fn if_statement_with_function_call() -> Result {
  Test::new()?
    .program(indoc! {
      "
      value = 16

      if (sqrt(value) == 4) {
        println('square root is 4')
      } else {
        println('square root is not 4')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("square root is 4\n"))
    .run()
}

#[test]
fn if_statement_with_boolean_expressions() -> Result {
  Test::new()?
    .program(indoc! {
      "
      a = 5
      b = 10

      if (a < 10 && b > 5) {
        println('condition met')
      } else {
        println('condition not met')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("condition met\n"))
    .run()
}

#[test]
fn if_statement_chain() -> Result {
  Test::new()?
    .program(indoc! {
      "
      score = 85

      if (score >= 90) {
        println('A')
      } else {
        if (score >= 80) {
          println('B')
        } else {
          if (score >= 70) {
            println('C')
          } else {
            if (score >= 60) {
              println('D')
            } else {
              println('F')
            }
          }
        }
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("B\n"))
    .run()
}

#[test]
fn if_with_while_loop() -> Result {
  Test::new()?
    .program(indoc! {
      "
      x = 0

      if (true) {
        while (x < 3) {
          println(x)
          x = x + 1
        }
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("0\n1\n2\n"))
    .run()
}

#[test]
fn if_with_list_access() -> Result {
  Test::new()?
    .program(indoc! {
      "
      numbers = [10, 20, 30]
      index = 1

      if (index < numbers[0]) {
        println(numbers[index])
      } else {
        println('index too large')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("20\n"))
    .run()
}

#[test]
fn simple_function_definition_and_call() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn add(a, b) {
        return a + b
      }

      println(add(3, 4))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("7\n"))
    .run()
}

#[test]
fn function_with_return_value() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn get_message() {
        return 'Hello, world!'
      }

      println(get_message())
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("Hello, world!\n"))
    .run()
}

#[test]
fn function_with_multiple_statements() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn calculate(x) {
        doubled = x * 2;
        return doubled + 10
      }

      println(calculate(5))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("20\n"))
    .run()
}

#[test]
fn function_wrong_argument_count() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn add(a, b) {
        return a + b
      }

      println(add(1))
      "
    })
    .expected_status(1)
    .expected_stderr(Contains("Function `add` expects 2 arguments, got 1"))
    .run()
}

#[test]
fn iterative_factorial() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn factorial(n) {
        result = 1;
        i = n;

        while (i > 0) {
          result = result * i;
          i = i - 1;
        }

        return result
      }

      println(factorial(5))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("120\n"))
    .run()
}

#[test]
fn direct_recursive_function() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn factorial(n) {
        if (n <= 1) {
          return 1
        } else {
          return n * factorial(n - 1)
        }
      }

      println(factorial(5))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("120\n"))
    .run()
}

#[test]
fn function_call_as_argument() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn double(x) {
        return x * 2
      }

      fn triple(x) {
        return x * 3
      }

      println(double(triple(2)))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("12\n"))
    .run()
}

#[test]
fn function_with_local_variables() -> Result {
  Test::new()?
    .program(indoc! {
      "
      global = 10;

      fn test_scope() {
        local = 5
        return global + local
      }

      println(test_scope())
      println(global)
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("15\n10\n"))
    .run()
}

#[test]
#[ignore]
fn function_modifying_outer_scope() -> Result {
  Test::new()?
    .program(indoc! {
      "
      counter = 0;

      fn increment() {
        counter = counter + 1;
        return counter
      }

      println(increment())
      println(increment())
      println(counter)
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("1\n2\n2\n"))
    .run()
}

#[test]
fn function_with_no_arguments() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn get_pi() {
        return 3.14159
      }

      println(get_pi())
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("3.14159\n"))
    .run()
}

#[test]
fn nested_function_calls() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn add(a, b) {
        return a + b
      }

      fn multiply(a, b) {
        return a * b
      }

      println(add(multiply(2, 3), multiply(4, 5)))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("26\n"))
    .run()
}

#[test]
fn function_calling_builtin() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn square_root_of_sin(x) {
        return sqrt(sin(x))
      }

      println(square_root_of_sin(0.5))
      "
    })
    .expected_status(0)
    .expected_stdout(Contains("0."))
    .run()
}

#[test]
fn fibonacci_function() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn fibonacci(n) {
        if (n <= 0) {
          return 0
        } else {
          if (n == 1) {
            return 1
          } else {
            return fibonacci(n - 1) + fibonacci(n - 2)
          }
        }
      }

      println(fibonacci(0))
      println(fibonacci(1))
      println(fibonacci(2))
      println(fibonacci(3))
      println(fibonacci(4))
      println(fibonacci(5))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("0\n1\n1\n2\n3\n5\n"))
    .run()
}

#[test]
fn can_override_builtin_functions() -> Result {
  Test::new()?
    .program(indoc! {
      "
      println(abs(-5))

      fn abs(x) {
        if (x < 0) {
          return x
        } else {
          return -x
        }
      }

      println(abs(-5))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("5\n-5\n"))
    .run()
}

#[test]
fn list_access_with_comparison() -> Result {
  Test::new()?
    .program(indoc! {
      "
      a = [1, 2, 3]
      println(a[0] == 1)
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn print_without_newline() -> Result {
  Test::new()?
    .program("print(1 + 1)")
    .expected_status(0)
    .expected_stdout(Exact("2"))
    .run()
}

#[test]
fn logical_and_operator() -> Result {
  Test::new()?
    .program("println(true && true)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(true && false)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(false && true)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("println(false && false)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()
}

#[test]
fn logical_or_operator() -> Result {
  Test::new()?
    .program("println(true || true)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(true || false)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(false || true)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("println(false || false)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()
}

#[test]
fn type_conversions_int() -> Result {
  Test::new()?
    .program(indoc! {
      "
      println(int(5.7))
      println(int('42'))
      println(int(true))
      println(int(false))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("5\n42\n1\n0\n"))
    .run()
}

#[test]
fn type_conversions_float() -> Result {
  Test::new()?
    .program(indoc! {
      "
      println(float(5))
      println(float('3.14'))
      println(float(true))
      println(float(false))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("5\n3.14\n1\n0\n"))
    .run()
}

#[test]
fn type_conversions_list() -> Result {
  Test::new()?
    .program(indoc! {
      "
      println(list('abc'))
      println(list(123))
      println(list(true))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("['a', 'b', 'c']\n[123]\n[true]\n"))
    .run()
}

#[test]
fn string_split() -> Result {
  Test::new()?
    .program(indoc! {
      "
      println(split('a,b,c', ','))
      println(split('hello world', ' '))
      println(split('abc', ''))
      println(split('no-delimiter-here', 'x'))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("['a', 'b', 'c']\n['hello', 'world']\n['a', 'b', 'c']\n['no-delimiter-here']\n"))
    .run()
}

#[test]
fn split_and_convert() -> Result {
  Test::new()?
    .program(indoc! {
      "
      csv_line = '10,20,30'
      parts = split(csv_line, ',')

      sum = 0
      i = 0
      while (i < 3) {
        sum = sum + int(parts[i])
        i = i + 1
      }

      println(sum)
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("60\n"))
    .run()
}

#[test]
fn cannot_return_outside_of_function() -> Result {
  Test::new()?
    .program(indoc! {
      "
      a = 0

      if (a == 0) {
        return 1
      }

      print(a)
      "
    })
    .expected_status(1)
    .expected_stderr(Contains("Cannot return outside of a function\n"))
    .run()
}

#[test]
fn list_element_assignment_updates_value() -> Result {
  Test::new()?
    .program(indoc! {
      "
      nums = [1, 2, 3]
      nums[0] = 10
      println(nums)
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("[10, 2, 3]\n"))
    .run()
}

#[test]
fn list_element_assignment_then_read() -> Result {
  Test::new()?
    .program(indoc! {
      "
      letters = ['a', 'b', 'c']
      letters[1] = 'x'
      println(letters[1])
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("x\n"))
    .run()
}

#[test]
fn element_assignment_on_non_list_errors() -> Result {
  Test::new()?
    .program(indoc! {
      "
      value = 42
      value[0] = 1
      "
    })
    .expected_status(1)
    .expected_stderr(Contains("'value' is not a list"))
    .run()
}
