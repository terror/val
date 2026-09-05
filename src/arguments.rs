use super::*;

#[derive(Debug, Parser)]
#[clap(
  about,
  author,
  version,
  help_template = "\
{before-help}{name} {version}

{about}

{usage-heading}: {usage}

{all-args}{after-help}
"
)]
pub(crate) struct Arguments {
  #[clap(
    short,
    long,
    value_parser = clap::value_parser!(NonZeroUsize),
    default_value = "16",
    help = "Decimal digits to display for approximate numbers"
  )]
  digits: NonZeroUsize,
  #[clap(
    short,
    long,
    conflicts_with = "filename",
    help = "Expression to evaluate"
  )]
  expression: Option<String>,
  #[clap(conflicts_with = "expression", help = "File to evaluate")]
  filename: Option<PathBuf>,
  #[clap(
    short,
    long,
    conflicts_with = "filename",
    help = "Load files before entering the REPL"
  )]
  load: Option<Vec<PathBuf>>,
  #[clap(
    short,
    long,
    value_parser = clap::value_parser!(NonZeroU32),
    default_value = "1024",
    help = "Binary precision (bits) to use for calculations"
  )]
  precision: NonZeroU32,
  #[clap(
    short,
    long,
    value_parser = clap::value_parser!(RoundingMode),
    default_value = "to-even",
    help = "Rounding mode to use for calculations",
  )]
  rounding_mode: RoundingMode,
  #[clap(
    long,
    value_parser = clap::value_parser!(NonZeroUsize),
    default_value = "128",
    help = "Stack size in MB for evaluations"
  )]
  pub stack_size: NonZeroUsize,
}

impl Arguments {
  fn eval(&self, filename: &PathBuf) -> Result {
    let source =
      Source::new(filename.to_string_lossy(), fs::read_to_string(filename)?);

    let mut evaluator =
      Evaluator::from(Environment::new(Into::<Config>::into(self)));

    match Self::evaluate(&mut evaluator, &source) {
      Ok(Evaluation::Exit { code, .. }) => process::exit(code),
      Ok(Evaluation::Value(_)) => Ok(()),
      Err(errors) => {
        for error in errors {
          Self::report(&error, &source, io::stderr())?;
        }

        process::exit(1);
      }
    }
  }

  fn evaluate<'a>(
    evaluator: &mut Evaluator<'a>,
    source: &Source,
  ) -> Result<Evaluation<'a>, Vec<Error>> {
    let ast = parse(source.text())?;

    evaluator.set_source(source.clone());

    evaluator.evaluate(&ast).map_err(|error| vec![error])
  }

  fn evaluate_expression(&self, value: String) -> Result {
    let source = Source::new("<expression>", value);

    let mut evaluator =
      Evaluator::from(Environment::new(Into::<Config>::into(self)));

    match Self::evaluate(&mut evaluator, &source) {
      Ok(Evaluation::Exit { code, .. }) => process::exit(code),
      Ok(Evaluation::Value(value)) => {
        if let Value::Null = value {
          return Ok(());
        }

        println!("{}", value.display(Into::<Config>::into(self)));

        Ok(())
      }
      Err(errors) => {
        for error in errors {
          Self::report(&error, &source, io::stderr())?;
        }

        process::exit(1);
      }
    }
  }

  #[cfg(not(target_family = "wasm"))]
  fn read(&self) -> Result {
    let history = dirs::home_dir().unwrap_or_default().join(".val_history");

    let editor_config = Builder::new()
      .color_mode(ColorMode::Enabled)
      .edit_mode(EditMode::Emacs)
      .history_ignore_space(true)
      .completion_type(CompletionType::Circular)
      .max_history_size(1000)?
      .build();

    let mut editor =
      Editor::<Prompt, DefaultHistory>::with_config(editor_config)?;

    editor.set_helper(Some(Prompt::new()));
    editor.load_history(&history).ok();

    let mut evaluator =
      Evaluator::from(Environment::new(Into::<Config>::into(self)));

    if let Some(filenames) = &self.load {
      for filename in filenames {
        let source = Source::new(
          filename.to_string_lossy(),
          fs::read_to_string(filename)?,
        );

        match Self::evaluate(&mut evaluator, &source) {
          Ok(Evaluation::Exit { code, .. }) => process::exit(code),
          Ok(Evaluation::Value(_)) => {}
          Err(errors) => {
            for error in errors {
              Self::report(&error, &source, io::stderr())?;
            }

            process::exit(1);
          }
        }
      }
    }

    loop {
      let line = editor.readline("> ")?;

      editor.add_history_entry(&line)?;
      editor.save_history(&history)?;

      let source = Source::new("<input>", line);

      match Self::evaluate(&mut evaluator, &source) {
        Ok(Evaluation::Exit { code, .. }) => process::exit(code),
        Ok(Evaluation::Value(value)) if !matches!(value, Value::Null) => {
          println!("{}", value.display(Into::<Config>::into(self)));
        }
        Ok(Evaluation::Value(_)) => {}
        Err(errors) => {
          for error in errors {
            Self::report(&error, &source, io::stderr())?;
          }
        }
      }
    }
  }

  fn report(error: &Error, source: &Source, writer: impl io::Write) -> Result {
    let source = error.origin().unwrap_or(source);

    error.report(source.name()).write(
      (source.name(), ariadne::Source::from(source.text())),
      writer,
    )?;

    Ok(())
  }

  pub(crate) fn run(self) -> Result {
    match (&self.filename, &self.expression) {
      (Some(filename), _) => self.eval(filename),
      (_, Some(expression)) => self.evaluate_expression(expression.clone()),
      _ => {
        #[cfg(not(target_family = "wasm"))]
        {
          self.read()
        }
        #[cfg(target_family = "wasm")]
        {
          Err(anyhow::anyhow!("Interactive mode not supported in WASM"))
        }
      }
    }
  }
}

impl From<&Arguments> for Config {
  fn from(arguments: &Arguments) -> Self {
    Config {
      digits: arguments.digits,
      precision: arguments.precision.get(),
      rounding_mode: arguments.rounding_mode.into(),
    }
  }
}

#[cfg(test)]
mod tests {
  use {super::*, clap::Parser, std::path::PathBuf};

  #[test]
  fn both_should_fail() {
    assert!(
      Arguments::try_parse_from(vec![
        "program",
        "file.txt",
        "--expression",
        "1 + 2"
      ])
      .is_err()
    );
  }

  #[test]
  fn conflict_error_message() {
    let result = Arguments::try_parse_from(vec![
      "program",
      "file.txt",
      "--expression",
      "1 + 2",
    ]);

    assert!(result.is_err());

    let error = result.unwrap_err().to_string();

    assert!(
      error.contains("cannot be used with"),
      "Error should mention conflicts: {error}"
    );
  }

  #[test]
  fn digits() {
    #[track_caller]
    fn case(argument: &str) {
      let arguments = Arguments::parse_from(vec!["program", argument, "4"]);

      assert_eq!(arguments.digits, NonZeroUsize::new(4).unwrap());
    }

    case("--digits");
    case("-d");
  }

  #[test]
  fn digits_rejects_zero() {
    let result = Arguments::try_parse_from(vec!["program", "--digits", "0"]);

    assert!(result.is_err());
  }

  #[test]
  fn error_sources() {
    #[track_caller]
    fn case(inputs: &[(&str, &str)], expected: usize, span: &str) {
      let sources = inputs
        .iter()
        .map(|(name, text)| Source::new(*name, *text))
        .collect::<Vec<_>>();

      let (source, definitions) = sources.split_last().unwrap();

      let mut evaluator = Evaluator::from(Environment::new(Config::default()));

      for source in definitions {
        Arguments::evaluate(&mut evaluator, source).unwrap();
      }

      let errors = Arguments::evaluate(&mut evaluator, source).unwrap_err();

      let [error] = errors.as_slice() else {
        panic!("expected one error");
      };

      let expected = &sources[expected];

      assert_eq!(error.origin(), Some(expected));
      assert_eq!(
        expected
          .text()
          .get(error.span().into_range())
          .map(str::trim),
        Some(span)
      );

      let mut report = Vec::new();

      Arguments::report(error, source, &mut report).unwrap();

      let report = String::from_utf8(report).unwrap();

      assert!(report.contains(expected.name()), "{report}");
      assert!(report.contains(expected.text()), "{report}");
    }

    case(
      &[("<input>", "fn foo() { bar }"), ("<input>", "foo()")],
      0,
      "bar",
    );
    case(
      &[
        ("<input>", "fn foo() { bar }"),
        ("<input>", "foo(); 'éééééééé'"),
      ],
      0,
      "bar",
    );
    case(
      &[
        ("foo.val", "fn foo() { bar }"),
        ("bar.val", "fn baz() { foo() }"),
        ("<input>", "baz()"),
      ],
      0,
      "bar",
    );
    case(
      &[
        ("foo.val", "fn foo() { bar }"),
        ("bar.val", "foo(); 'éééééééé'"),
      ],
      0,
      "bar",
    );
    case(
      &[
        ("foo.val", "fn foo() { fn() { bar } }"),
        ("<input>", "baz = foo()"),
        ("<input>", "baz()"),
      ],
      0,
      "bar",
    );
    case(
      &[("<input>", "foo = [fn() { bar }]"), ("<input>", "foo[0]()")],
      0,
      "bar",
    );
    case(
      &[
        ("foo.val", "fn foo(bar) { bar() }"),
        ("<input>", "foo(fn() { baz })"),
      ],
      1,
      "baz",
    );
    case(
      &[("<input>", "fn foo() { 'é'; bar }"), ("<input>", "foo()")],
      0,
      "bar",
    );
    case(
      &[("foo.val", "fn foo() { bar }"), ("<input>", "foo(1)")],
      1,
      "foo(1)",
    );
    case(
      &[("foo.val", "fn foo(bar) { bar }"), ("<input>", "foo(baz)")],
      1,
      "baz",
    );
    case(
      &[("foo.val", "fn foo() {}"), ("<input>", "sqrt(-1)")],
      1,
      "sqrt(-1)",
    );
  }

  #[test]
  fn exit_preserves_control_flow() {
    #[track_caller]
    fn case(definition: &str) {
      let mut evaluator = Evaluator::from(Environment::new(Config::default()));

      Arguments::evaluate(&mut evaluator, &Source::new("foo.val", definition))
        .unwrap();

      let result =
        Arguments::evaluate(&mut evaluator, &Source::new("<input>", "foo()"))
          .unwrap();

      assert!(matches!(result, Evaluation::Exit { code: 1, .. }));
    }

    case("fn foo() { exit(1) }");
    case("fn foo() { quit(1) }");
    case("fn bar() { exit(1) }; fn foo() { bar() }");
  }

  #[test]
  fn expression_only() {
    let arguments =
      Arguments::parse_from(vec!["program", "--expression", "1 + 2"]);

    assert!(arguments.filename.is_none());
    assert!(arguments.expression.is_some());

    assert_eq!(arguments.expression.unwrap(), "1 + 2");
  }

  #[test]
  fn expression_short_form() {
    let arguments = Arguments::parse_from(vec!["program", "-e", "1 + 2"]);

    assert!(arguments.filename.is_none());
    assert!(arguments.expression.is_some());

    assert_eq!(arguments.expression.unwrap(), "1 + 2");
  }

  #[test]
  fn filename_only() {
    let arguments = Arguments::parse_from(vec!["program", "file.txt"]);

    assert!(arguments.filename.is_some());
    assert!(arguments.expression.is_none());

    assert_eq!(arguments.filename.unwrap(), PathBuf::from("file.txt"));
  }

  #[test]
  fn load_conflicts_with_filename() {
    let result = Arguments::try_parse_from(vec![
      "program",
      "file.txt",
      "--load",
      "prelude.val",
    ]);

    assert!(result.is_err(), "Parser should reject filename + --load");

    let error = result.unwrap_err().to_string();

    assert!(
      error.contains("cannot be used with"),
      "Error should mention conflicts: {error}"
    );
  }

  #[test]
  fn neither_provided() {
    let arguments = Arguments::parse_from(vec!["program"]);

    assert!(arguments.filename.is_none());
    assert!(arguments.expression.is_none());
  }

  #[test]
  fn nonzero_arguments_reject_zero() {
    for argument in ["--precision", "--stack-size"] {
      let result = Arguments::try_parse_from(vec!["program", argument, "0"]);

      assert!(result.is_err());
    }
  }

  #[test]
  fn parser_error_uses_current_source() {
    let mut evaluator = Evaluator::from(Environment::new(Config::default()));

    Arguments::evaluate(
      &mut evaluator,
      &Source::new("foo.val", "fn foo() { bar }"),
    )
    .unwrap();

    let source = Source::new("<input>", "'éééééééé'; (");

    let errors = Arguments::evaluate(&mut evaluator, &source).unwrap_err();
    let mut report = Vec::new();

    for error in errors {
      Arguments::report(&error, &source, &mut report).unwrap();
    }

    let report = String::from_utf8(report).unwrap();

    assert!(report.contains(source.name()), "{report}");
    assert!(report.contains(source.text()), "{report}");
  }
}
