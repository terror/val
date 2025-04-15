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
    .program("print(25)")
    .expected_status(0)
    .expected_stdout(Exact("25\n"))
    .run()
}

#[test]
fn float_literals() -> Result {
  Test::new()?
    .program("print(3.14)")
    .expected_status(0)
    .expected_stdout(Exact("3.14\n"))
    .run()?;

  Test::new()?
    .program("print(0.5)")
    .expected_status(0)
    .expected_stdout(Exact("0.5\n"))
    .run()?;

  Test::new()?
    .program("print(-2.718)")
    .expected_status(0)
    .expected_stdout(Exact("-2.718\n"))
    .run()?;

  Test::new()?
    .program("print(1.0 + 2.5)")
    .expected_status(0)
    .expected_stdout(Exact("3.5\n"))
    .run()?;

  Test::new()?
    .program("print(3.5 * 2.0)")
    .expected_status(0)
    .expected_stdout(Exact("7\n"))
    .run()
}

#[test]
fn negate_integer_literal() -> Result {
  Test::new()?
    .program("print(-25)")
    .expected_status(0)
    .expected_stdout(Exact("-25\n"))
    .run()?;

  Test::new()?
    .program("print(--25)")
    .expected_status(0)
    .expected_stdout(Exact("25\n"))
    .run()?;

  Test::new()?
    .program("print(- - - -25)")
    .expected_status(0)
    .expected_stdout(Exact("25\n"))
    .run()
}

#[test]
fn call_builtin_function() -> Result {
  Test::new()?
    .program("print(sin(1))")
    .expected_status(0)
    .expected_stdout(Exact("0.8414709848078965\n"))
    .run()?;

  Test::new()?
    .program("print(cos(1))")
    .expected_status(0)
    .expected_stdout(Exact("0.5403023058681398\n"))
    .run()
}

#[test]
fn undefined_variable() -> Result {
  Test::new()?
    .program("print(foo)")
    .expected_status(1)
    .expected_stderr(Contains("Undefined variable `foo`\n"))
    .run()
}

#[test]
fn addition() -> Result {
  Test::new()?
    .program("print(2 + 3)")
    .expected_status(0)
    .expected_stdout(Exact("5\n"))
    .run()?;

  Test::new()?
    .program("print(2 + 3 + 4)")
    .expected_status(0)
    .expected_stdout(Exact("9\n"))
    .run()?;

  Test::new()?
    .program("print(-5 + 10)")
    .expected_status(0)
    .expected_stdout(Exact("5\n"))
    .run()
}

#[test]
fn subtraction() -> Result {
  Test::new()?
    .program("print(5 - 3)")
    .expected_status(0)
    .expected_stdout(Exact("2\n"))
    .run()?;

  Test::new()?
    .program("print(10 - 5 - 2)")
    .expected_status(0)
    .expected_stdout(Exact("3\n"))
    .run()?;

  Test::new()?
    .program("print(5 - 10)")
    .expected_status(0)
    .expected_stdout(Exact("-5\n"))
    .run()
}

#[test]
fn multiplication() -> Result {
  Test::new()?
    .program("print(2 * 3)")
    .expected_status(0)
    .expected_stdout(Exact("6\n"))
    .run()?;

  Test::new()?
    .program("print(2 * 3 * 4)")
    .expected_status(0)
    .expected_stdout(Exact("24\n"))
    .run()?;

  Test::new()?
    .program("print(-5 * 10)")
    .expected_status(0)
    .expected_stdout(Exact("-50\n"))
    .run()
}

#[test]
fn division() -> Result {
  Test::new()?
    .program("print(6 / 3)")
    .expected_status(0)
    .expected_stdout(Exact("2\n"))
    .run()?;

  Test::new()?
    .program("print(10 / 2 / 5)")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()?;

  Test::new()?
    .program("print(10 / 4)")
    .expected_status(0)
    .expected_stdout(Exact("2.5\n"))
    .run()
}

#[test]
fn division_by_zero() -> Result {
  Test::new()?
    .program("print(5 / 0)")
    .expected_status(1)
    .expected_stderr(Contains("Division by zero"))
    .run()
}

#[test]
fn modulo() -> Result {
  Test::new()?
    .program("print(7 % 4)")
    .expected_status(0)
    .expected_stdout(Exact("3\n"))
    .run()?;

  Test::new()?
    .program("print(10 % 3)")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()
}

#[test]
fn modulo_by_zero() -> Result {
  Test::new()?
    .program("print(5 % 0)")
    .expected_status(1)
    .expected_stderr(Contains("Modulo by zero"))
    .run()
}

#[test]
fn operator_precedence() -> Result {
  Test::new()?
    .program("print(2 + 3 * 4)")
    .expected_status(0)
    .expected_stdout(Exact("14\n"))
    .run()?;

  Test::new()?
    .program("print((2 + 3) * 4)")
    .expected_status(0)
    .expected_stdout(Exact("20\n"))
    .run()?;

  Test::new()?
    .program("print(2 * 3 + 4 * 5)")
    .expected_status(0)
    .expected_stdout(Exact("26\n"))
    .run()?;

  Test::new()?
    .program("print(10 - 6 / 2)")
    .expected_status(0)
    .expected_stdout(Exact("7\n"))
    .run()
}

#[test]
fn combined_operations() -> Result {
  Test::new()?
    .program("print(2 + 3 * 4 - 5 / 2)")
    .expected_status(0)
    .expected_stdout(Exact("11.5\n"))
    .run()?;

  Test::new()?
    .program("print(10 % 3 + 4 * 2 - 1)")
    .expected_status(0)
    .expected_stdout(Exact("8\n"))
    .run()?;

  Test::new()?
    .program("print(-5 * (2 + 3) / 5)")
    .expected_status(0)
    .expected_stdout(Exact("-5\n"))
    .run()
}

#[test]
fn power() -> Result {
  Test::new()?
    .program("print(2 ^ 3)")
    .expected_status(0)
    .expected_stdout(Exact("8\n"))
    .run()?;

  Test::new()?
    .program("print(10 ^ 2)")
    .expected_status(0)
    .expected_stdout(Exact("100\n"))
    .run()?;

  Test::new()?
    .program("print(2 ^ -1)")
    .expected_status(0)
    .expected_stdout(Exact("0.5\n"))
    .run()?;

  Test::new()?
    .program("print(2 ^ 0)")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()
}

#[test]
fn arctangent() -> Result {
  Test::new()?
    .program("print(arc(1))")
    .expected_status(0)
    .expected_stdout(Exact("0.7853981633974483\n"))
    .run()?;

  Test::new()?
    .program("print(arc(0))")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("print(arc(-1))")
    .expected_status(0)
    .expected_stdout(Exact("-0.7853981633974483\n"))
    .run()?;

  Test::new()?
    .program("print(arc())")
    .expected_status(1)
    .expected_stderr(Contains("Function `arc` expects 1 argument, got 0"))
    .run()
}

#[test]
fn natural_logarithm() -> Result {
  Test::new()?
    .program("print(ln(1))")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("print(ln(e))")
    .expected_status(0)
    .expected_stdout(Exact("1\n"))
    .run()?;

  Test::new()?
    .program("print(ln(10))")
    .expected_status(0)
    .expected_stdout(Exact("2.302585092994046\n"))
    .run()?;

  Test::new()?
    .program("print(ln())")
    .expected_status(1)
    .expected_stderr(Contains("Function 'log' expects 1 argument, got 0"))
    .run()?;

  Test::new()?
    .program("print(ln(0))")
    .expected_status(1)
    .expected_stderr(Contains(
      "Cannot take logarithm of zero or negative number",
    ))
    .run()?;

  Test::new()?
    .program("print(ln(-1))")
    .expected_status(1)
    .expected_stderr(Contains(
      "Cannot take logarithm of zero or negative number",
    ))
    .run()
}

#[test]
fn builtin_variables_and_functions_can_coexist() -> Result {
  Test::new()?
    .program("print(e * e(20))")
    .expected_status(0)
    .expected_stdout(Exact("1318815734.4832146\n"))
    .run()
}

#[test]
fn functions_with_constants() -> Result {
  Test::new()?
    .program("print(arc(pi / 4))")
    .expected_status(0)
    .expected_stdout(Exact("0.6657737500283538\n"))
    .run()?;

  Test::new()?
    .program("print(ln(e * 2))")
    .expected_status(0)
    .expected_stdout(Exact("1.6931471805599452\n"))
    .run()?;

  Test::new()?
    .program("print(e(pi))")
    .expected_status(0)
    .expected_stdout(Exact("23.140692632779267\n"))
    .run()
}

#[test]
fn square_root() -> Result {
  Test::new()?
    .program("print(sqrt(4))")
    .expected_status(0)
    .expected_stdout(Exact("2\n"))
    .run()?;

  Test::new()?
    .program("print(sqrt(2))")
    .expected_status(0)
    .expected_stdout(Exact("1.4142135623730951\n"))
    .run()?;

  Test::new()?
    .program("print(sqrt(0))")
    .expected_status(0)
    .expected_stdout(Exact("0\n"))
    .run()?;

  Test::new()?
    .program("print(sqrt())")
    .expected_status(1)
    .expected_stderr(Contains("Function `sqrt` expects 1 argument, got 0"))
    .run()?;

  Test::new()?
    .program("print(sqrt(-1))")
    .expected_status(1)
    .expected_stderr(Contains("Cannot take square root of negative number"))
    .run()
}

#[test]
fn less_than() -> Result {
  Test::new()?
    .program("print(1 < 2)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print(1 < -1)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()
}

#[test]
fn greater_than() -> Result {
  Test::new()?
    .program("print(1 > 2)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("print(1 > -1)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn not() -> Result {
  Test::new()?
    .program("print(!(1 > 2))")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print(!(1 > -1))")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("print(!true)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("print(!false)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn string_literals() -> Result {
  Test::new()?
    .program("print(\"Hello, world!\")")
    .expected_status(0)
    .expected_stdout(Exact("'Hello, world!'\n"))
    .run()?;

  Test::new()?
    .program("print('Hello, world!')")
    .expected_status(0)
    .expected_stdout(Exact("'Hello, world!'\n"))
    .run()
}

#[test]
fn len() -> Result {
  Test::new()?
    .program("print(len(\"Hello, world!\"))")
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
    .program("print(1 == 1)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print(1 == 2)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("print(\"hello\" == \"hello\")")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print(\"hello\" == \"world\")")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()
}

#[test]
fn not_equal_to() -> Result {
  Test::new()?
    .program("print(1 != 1)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("print(1 != 2)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print(\"hello\" != \"hello\")")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("print(\"hello\" != \"world\")")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn less_than_or_equal() -> Result {
  Test::new()?
    .program("print(1 <= 1)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print(1 <= 2)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print(2 <= 1)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()
}

#[test]
fn greater_than_or_equal() -> Result {
  Test::new()?
    .program("print(1 >= 1)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print(1 >= 2)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("print(2 >= 1)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn comparison_with_expressions() -> Result {
  Test::new()?
    .program("print((1 + 2) == (4 - 1))")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print(2 * 3 >= 5)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print(10 / 2 <= 4)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("print(2 ^ 3 != 9)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn boolean_comparison() -> Result {
  Test::new()?
    .program("print(true == true)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print(true != false)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print((1 < 2) == true)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn mixed_type_comparisons() -> Result {
  Test::new()?
    .program("print(\"true\" == true)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("print(\"false\" != false)")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()?;

  Test::new()?
    .program("print(1 == true)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("print(0 == false)")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("print(123 == \"123\")")
    .expected_status(0)
    .expected_stdout(Exact("false\n"))
    .run()?;

  Test::new()?
    .program("print(0 != \"0\")")
    .expected_status(0)
    .expected_stdout(Exact("true\n"))
    .run()
}

#[test]
fn list_literals() -> Result {
  Test::new()?
    .program("print([1, 2, 1 + 2])")
    .expected_status(0)
    .expected_stdout(Exact("[1, 2, 3]\n"))
    .run()?;

  Test::new()?
    .program("print([print('foo'), 'foo', 1 + 2])")
    .expected_status(0)
    .expected_stdout(Exact("'foo'\n[null, 'foo', 3]\n"))
    .run()
}

#[test]
fn assignment() -> Result {
  Test::new()?
    .program(indoc! {
      "
      a = 1
      print(a)

      a = [1, 2, 3]
      print(a)
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
      print(a[1])

      a = [1, 2, 3]
      print(a[1 + 1])
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
      print(a[20])
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

      print(sum)
      print(counter)
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
          print(a + b)
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
        print('greater')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("'greater'\n"))
    .run()
}

#[test]
fn if_statement_false_condition() -> Result {
  Test::new()?
    .program(indoc! {
      "
      x = 3
      if (x > 5) {
        print('greater')
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
        print('greater')
      } else {
        print('not greater')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("'greater'\n"))
    .run()
}

#[test]
fn if_else_statement_false_condition() -> Result {
  Test::new()?
    .program(indoc! {
      "
      x = 3
      if (x > 5) {
        print('greater')
      } else {
        print('not greater')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("'not greater'\n"))
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
        print('sum is greater than 8')
      } else {
        print('sum is not greater than 8')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("'sum is greater than 8'\n"))
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
          print('both conditions met')
        } else {
          print('only x condition met')
        }
      } else {
        print('x condition not met')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("'both conditions met'\n"))
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

      print(result)
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("'greater'\n"))
    .run()
}

#[test]
fn if_statement_with_function_call() -> Result {
  Test::new()?
    .program(indoc! {
      "
      value = 16

      if (sqrt(value) == 4) {
        print('square root is 4')
      } else {
        print('square root is not 4')
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("'square root is 4'\n"))
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
        print('condition met')
      } else {
        print('condition not met')
      }
      "
    })
    .expected_status(0)
    .expected_stderr(Contains("found '&' expected"))
    .expected_status(1)
    .run()
}

#[test]
fn if_statement_chain() -> Result {
  Test::new()?
    .program(indoc! {
      "
      score = 85

      if (score >= 90) {
        print('A')
      } else {
        if (score >= 80) {
          print('B')
        } else {
          if (score >= 70) {
            print('C')
          } else {
            if (score >= 60) {
              print('D')
            } else {
              print('F')
            }
          }
        }
      }
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("'B'\n"))
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
          print(x)
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
        print(numbers[index])
      } else {
        print('index too large')
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
        a + b
      }

      print(add(3, 4))
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
        'Hello, world!'
      }

      print(get_message())
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("'Hello, world!'\n"))
    .run()
}

#[test]
fn function_with_multiple_statements() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn calculate(x) {
        doubled = x * 2;
        doubled + 10
      }

      print(calculate(5))
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
        a + b
      }

      print(add(1))
      "
    })
    .expected_status(1)
    .expected_stderr(Contains("Function `add` expects 2 arguments, got 1"))
    .run()
}

#[test]
fn recursive_function() -> Result {
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

        result
      }

      print(factorial(5))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("120\n"))
    .run()
}

#[test]
fn recursive_function_direct_style() -> Result {
  Test::new()?
    .program(indoc! {
      "
      fn factorial(n) {
        if (n <= 1) {
          1
        } else {
          n * factorial(n - 1)
        }
      }

      print(factorial(5))
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
        x * 2
      }

      fn triple(x) {
        x * 3
      }

      print(double(triple(2)))
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
        global + local
      }

      print(test_scope())
      print(global)
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
        counter
      }

      print(increment())
      print(increment())
      print(counter)
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
        3.14159
      }

      print(get_pi())
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
        a + b
      }

      fn multiply(a, b) {
        a * b
      }

      print(add(multiply(2, 3), multiply(4, 5)))
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
        sqrt(sin(x))
      }

      print(square_root_of_sin(0.5))
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
          0
        } else {
          if (n == 1) {
            1
          } else {
            fibonacci(n - 1) + fibonacci(n - 2)
          }
        }
      }

      print(fibonacci(0))
      print(fibonacci(1))
      print(fibonacci(2))
      print(fibonacci(3))
      print(fibonacci(4))
      print(fibonacci(5))
      "
    })
    .expected_status(0)
    .expected_stdout(Exact("0\n1\n1\n2\n3\n5\n"))
    .run()
}
