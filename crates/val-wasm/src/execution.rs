use super::*;

#[derive(Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
  errors: Vec<ValError>,
  exit_code: i32,
  output: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  value: Option<String>,
}

impl Execution {
  pub(crate) fn run(source: &str, input: &str) -> Self {
    let config = Config::default();

    let environment = Runtime::environment(config, input);

    let (value, exit_code, errors) = match val::parse(source) {
      Ok(ast) => match Evaluator::from(environment).evaluate(&ast) {
        Ok(Evaluation::Value(value)) => (
          (value != Value::Null).then(|| value.display(config)),
          0,
          Vec::new(),
        ),
        Ok(Evaluation::Exit { code, .. }) => (None, code, Vec::new()),
        Err(error) => (
          None,
          1,
          vec![ValError {
            kind: ErrorKind::Evaluator,
            message: error.to_string(),
            range: RangeConverter::convert_span(error.span()),
          }],
        ),
      },
      Err(errors) => (
        None,
        1,
        errors
          .into_iter()
          .map(|error| ValError {
            kind: ErrorKind::Parser,
            message: error.to_string(),
            range: RangeConverter::convert_span(error.span()),
          })
          .collect(),
      ),
    };

    Self {
      errors,
      exit_code,
      output: Runtime::take_output(),
      value,
    }
  }
}
