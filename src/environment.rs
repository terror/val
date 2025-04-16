use super::*;

#[derive(Clone, Debug, Default)]
pub struct Environment<'src> {
  functions: HashMap<&'src str, Function<'src>>,
  parent: Option<Box<Environment<'src>>>,
  variables: HashMap<&'src str, Value<'src>>,
}

impl<'src> Environment<'src> {
  pub fn new() -> Self {
    let mut env = Self::default();

    env.add_function(
      "sin",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `sin` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        Ok(Value::Number(arguments[0].number(span)?.sin()))
      }),
    );

    env.add_function(
      "cos",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `cos` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        Ok(Value::Number(arguments[0].number(span)?.cos()))
      }),
    );

    env.add_function(
      "tan",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `tan` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        Ok(Value::Number(arguments[0].number(span)?.tan()))
      }),
    );

    env.add_function(
      "csc",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `csc` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let sin_val = arguments[0].number(span)?.sin();

        if sin_val == 0.0 {
          return Err(Error::new(span, "Cannot compute csc of multiple of π"));
        }

        Ok(Value::Number(1.0 / sin_val))
      }),
    );

    env.add_function(
      "sec",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `sec` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let cos_val = arguments[0].number(span)?.cos();

        if cos_val == 0.0 {
          return Err(Error::new(span, "Cannot compute sec of π/2 + nπ"));
        }

        Ok(Value::Number(1.0 / cos_val))
      }),
    );

    env.add_function(
      "cot",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `cot` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let tan_val = arguments[0].number(span)?.tan();

        if tan_val == 0.0 {
          return Err(Error::new(span, "Cannot compute cot of multiple of π"));
        }

        Ok(Value::Number(1.0 / tan_val))
      }),
    );

    env.add_function(
      "sinh",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `sinh` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        Ok(Value::Number(arguments[0].number(span)?.sinh()))
      }),
    );

    env.add_function(
      "cosh",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `cosh` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        Ok(Value::Number(arguments[0].number(span)?.cosh()))
      }),
    );

    env.add_function(
      "tanh",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `tanh` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        Ok(Value::Number(arguments[0].number(span)?.tanh()))
      }),
    );

    env.add_function(
      "asin",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `asin` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let x = arguments[0].number(span)?;

        if !(-1.0..=1.0).contains(&x) {
          return Err(Error::new(
            span,
            "asin argument must be between -1 and 1",
          ));
        }

        Ok(Value::Number(x.asin()))
      }),
    );

    env.add_function(
      "acos",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `acos` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let x = arguments[0].number(span)?;

        if !(-1.0..=1.0).contains(&x) {
          return Err(Error::new(
            span,
            "acos argument must be between -1 and 1",
          ));
        }

        Ok(Value::Number(x.acos()))
      }),
    );

    env.add_function(
      "arc",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `arc` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        Ok(Value::Number(arguments[0].number(span)?.atan()))
      }),
    );

    env.add_function(
      "acsc",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `acsc` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let x = arguments[0].number(span)?;

        if x.abs() < 1.0 {
          return Err(Error::new(
            span,
            "acsc argument must have absolute value at least 1",
          ));
        }

        Ok(Value::Number((1.0 / x).asin()))
      }),
    );

    env.add_function(
      "asec",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `asec` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let x = arguments[0].number(span)?;

        if x.abs() < 1.0 {
          return Err(Error::new(
            span,
            "asec argument must have absolute value at least 1",
          ));
        }

        Ok(Value::Number((1.0 / x).acos()))
      }),
    );

    env.add_function(
      "acot",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `acot` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let x = arguments[0].number(span)?;

        // Formula: acot(x) = π/2 - atan(x)
        Ok(Value::Number(std::f64::consts::FRAC_PI_2 - x.atan()))
      }),
    );

    env.add_function(
      "ln",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function 'ln' expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let number = arguments[0].number(span)?;

        if number <= 0.0 {
          return Err(Error::new(
            span,
            "Cannot take logarithm of zero or negative number",
          ));
        }

        Ok(Value::Number(number.ln()))
      }),
    );

    env.add_function(
      "log2",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `log2` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let number = arguments[0].number(span)?;

        if number <= 0.0 {
          return Err(Error::new(
            span,
            "Cannot take logarithm of zero or negative number",
          ));
        }

        Ok(Value::Number(number.log2()))
      }),
    );

    env.add_function(
      "log10",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `log10` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let number = arguments[0].number(span)?;

        if number <= 0.0 {
          return Err(Error::new(
            span,
            "Cannot take logarithm of zero or negative number",
          ));
        }

        Ok(Value::Number(number.log10()))
      }),
    );

    env.add_function(
      "e",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!("Function `e` expects 1 argument, got {}", arguments.len()),
          ));
        }

        Ok(Value::Number(arguments[0].number(span)?.exp()))
      }),
    );

    env.add_function(
      "sqrt",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `sqrt` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let number = arguments[0].number(span)?;

        if number < 0.0 {
          return Err(Error::new(
            span,
            "Cannot take square root of negative number",
          ));
        }

        Ok(Value::Number(number.sqrt()))
      }),
    );

    env.add_function(
      "ceil",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `ceil` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        Ok(Value::Number(arguments[0].number(span)?.ceil()))
      }),
    );

    env.add_function(
      "floor",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `floor` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        Ok(Value::Number(arguments[0].number(span)?.floor()))
      }),
    );

    env.add_function(
      "abs",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `abs` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        Ok(Value::Number(arguments[0].number(span)?.abs()))
      }),
    );

    env.add_function(
      "len",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `len` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        Ok(Value::Number(arguments[0].string(span)?.len() as f64))
      }),
    );

    env.add_function(
      "print",
      Function::Builtin(|arguments, _| {
        let mut output_strings = Vec::with_capacity(arguments.len());

        for argument in arguments {
          output_strings.push(format!("{}", argument));
        }

        print!("{}", output_strings.join(" "));

        Ok(Value::Null)
      }),
    );

    env.add_function(
      "println",
      Function::Builtin(|arguments, _| {
        let mut output_strings = Vec::with_capacity(arguments.len());

        for argument in arguments {
          output_strings.push(format!("{}", argument));
        }

        println!("{}", output_strings.join(" "));

        Ok(Value::Null)
      }),
    );

    env.add_function(
      "exit",
      Function::Builtin(|arguments, span| {
        if arguments.is_empty() {
          process::exit(0);
        }

        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `exit` expects 0 or 1 arguments, got {}",
              arguments.len()
            ),
          ));
        }

        process::exit(arguments[0].number(span)? as i32);
      }),
    );

    env.add_function(
      "quit",
      Function::Builtin(|arguments, span| {
        if arguments.is_empty() {
          process::exit(0);
        }

        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `quit` expects 0 or 1 arguments, got {}",
              arguments.len()
            ),
          ));
        }

        process::exit(arguments[0].number(span)? as i32);
      }),
    );

    env.add_function(
      "sum",
      Function::Builtin(|arguments, span| {
        if arguments.is_empty() {
          process::exit(0);
        }

        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `sum` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let list = arguments[0].list(span)?;

        let numbers = list
          .iter()
          .map(|x| x.number(span))
          .collect::<Result<Vec<_>, _>>()?;

        Ok(Value::Number(numbers.iter().sum::<f64>()))
      }),
    );

    env.add_function(
      "input",
      Function::Builtin(|arguments, span| {
        use std::io::{self, BufRead, Write};

        if arguments.len() > 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `input` expects 0 or 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        if arguments.len() == 1 {
          print!("{}", arguments[0].string(span)?);
          io::stdout().flush().unwrap();
        }

        let stdin = io::stdin();

        let mut input = String::new();

        match stdin.lock().read_line(&mut input) {
          Ok(_) => {
            if input.ends_with('\n') {
              input.pop();

              if input.ends_with('\r') {
                input.pop();
              }
            }

            Ok(Value::String(Box::leak(input.into_boxed_str())))
          }
          Err(e) => {
            Err(Error::new(span, format!("Failed to read input: {}", e)))
          }
        }
      }),
    );

    env.add_function(
      "int",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `int` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let value = &arguments[0];

        match value {
          Value::Number(n) => Ok(Value::Number(n.floor())),
          Value::String(s) => match s.trim().parse::<f64>() {
            Ok(n) => Ok(Value::Number(n.floor())),
            Err(_) => {
              Err(Error::new(span, format!("Cannot convert '{}' to int", s)))
            }
          },
          Value::Boolean(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
          _ => Err(Error::new(
            span,
            format!("Cannot convert {} to int", value.type_name()),
          )),
        }
      }),
    );

    env.add_function(
      "float",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `float` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let value = &arguments[0];

        match value {
          Value::Number(n) => Ok(Value::Number(*n)),
          Value::String(s) => match s.trim().parse::<f64>() {
            Ok(n) => Ok(Value::Number(n)),
            Err(_) => {
              Err(Error::new(span, format!("Cannot convert '{}' to float", s)))
            }
          },
          Value::Boolean(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
          _ => Err(Error::new(
            span,
            format!("Cannot convert {} to float", value.type_name()),
          )),
        }
      }),
    );

    env.add_function(
      "bool",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `bool` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let value = &arguments[0];

        match value {
          Value::Boolean(b) => Ok(Value::Boolean(*b)),
          Value::Number(n) => Ok(Value::Boolean(*n != 0.0)),
          Value::String(s) => Ok(Value::Boolean(!s.is_empty())),
          Value::List(items) => Ok(Value::Boolean(!items.is_empty())),
          Value::Null => Ok(Value::Boolean(false)),
          _ => Err(Error::new(
            span,
            format!("Cannot convert {} to bool", value.type_name()),
          )),
        }
      }),
    );

    env.add_function(
      "list",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 1 {
          return Err(Error::new(
            span,
            format!(
              "Function `list` expects 1 argument, got {}",
              arguments.len()
            ),
          ));
        }

        let value = &arguments[0];

        match value {
          Value::List(items) => Ok(Value::List(items.clone())),
          Value::String(s) => Ok(Value::List(
            s.chars()
              .map(|c| Value::String(Box::leak(c.to_string().into_boxed_str())))
              .collect(),
          )),
          _ => Ok(Value::List(vec![value.clone()])),
        }
      }),
    );

    env.add_function(
      "split",
      Function::Builtin(|arguments, span| {
        if arguments.len() != 2 {
          return Err(Error::new(
            span,
            format!(
              "Function `split` expects 2 arguments, got {}",
              arguments.len()
            ),
          ));
        }

        let string = arguments[0].string(span)?;

        let delimiter = arguments[1].string(span)?;

        Ok(Value::List(
          string
            .split(delimiter)
            .filter(|part| !part.is_empty())
            .map(|part| {
              Value::String(Box::leak(part.to_string().into_boxed_str()))
            })
            .collect(),
        ))
      }),
    );

    env.add_variable("e", Value::Number(std::f64::consts::E));
    env.add_variable("pi", Value::Number(std::f64::consts::PI));

    env
  }

  pub fn add_function(&mut self, name: &'src str, function: Function<'src>) {
    self.functions.insert(name, function);
  }

  pub fn add_variable(&mut self, name: &'src str, value: Value<'src>) {
    self.variables.insert(name, value);
  }

  pub fn call_function(
    &self,
    name: &str,
    arguments: Vec<Value<'src>>,
    span: Span,
  ) -> Result<Value<'src>, Error> {
    if let Some(function) = self.functions.get(name) {
      match function {
        Function::Builtin(function) => function(arguments, span),
        Function::UserDefined(Value::Function(
          name,
          parameters,
          body,
          environment,
        )) => {
          if parameters.len() != arguments.len() {
            return Err(Error::new(
              span,
              format!(
                "Function `{}` expects {} arguments, got {}",
                name,
                parameters.len(),
                arguments.len()
              ),
            ));
          }

          let mut call_environment =
            Environment::with_parent(environment.clone());

          call_environment.add_function(name, function.clone());

          for (parameter, argument) in parameters.iter().zip(arguments.iter()) {
            call_environment.add_variable(parameter, argument.clone());
          }

          let mut evaluator = Evaluator::with_environment(call_environment);

          if body.is_empty() {
            return Ok(Value::Null);
          }

          for statement in body.iter() {
            let result = evaluator.eval_statement(statement)?;

            if result.is_return() {
              return Ok(result.unwrap());
            }
          }

          Ok(Value::Null)
        }
        _ => Err(Error::new(span, format!("`{}` is not a function", name))),
      }
    } else {
      Err(Error::new(
        span,
        format!("Function `{}` is not defined", name),
      ))
    }
  }

  pub fn get_variable(&self, name: &str) -> Option<&Value<'src>> {
    if let Some(val) = self.variables.get(name) {
      Some(val)
    } else if let Some(parent) = &self.parent {
      parent.get_variable(name)
    } else {
      None
    }
  }

  pub fn with_parent(parent: Environment<'src>) -> Self {
    Self {
      functions: parent.functions.clone(),
      variables: HashMap::new(),
      parent: Some(Box::new(parent)),
    }
  }
}
