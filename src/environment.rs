use super::*;

#[derive(Clone, Debug, Default)]
pub struct Environment<'src> {
  pub config: Config,
  pub functions: HashMap<&'src str, Function<'src>>,
  pub parent: Option<Box<Environment<'src>>>,
  pub variables: HashMap<&'src str, Value<'src>>,
}

impl<'src> Environment<'src> {
  pub fn new(config: Config) -> Self {
    let mut env = Self {
      config: config.clone(),
      functions: HashMap::new(),
      parent: None,
      variables: HashMap::new(),
    };

    env.add_function(
      "sin",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `sin` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        Ok(Value::Number(
          payload.arguments[0].number(payload.span)?.sin(
            payload.config.precision,
            payload.config.rounding_mode,
            &mut Consts::new().unwrap(),
          ),
        ))
      }),
    );

    env.add_function(
      "cos",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `cos` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        Ok(Value::Number(
          payload.arguments[0].number(payload.span)?.cos(
            payload.config.precision,
            payload.config.rounding_mode,
            &mut Consts::new().unwrap(),
          ),
        ))
      }),
    );

    env.add_function(
      "tan",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `tan` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        Ok(Value::Number(
          payload.arguments[0].number(payload.span)?.tan(
            payload.config.precision,
            payload.config.rounding_mode,
            &mut Consts::new().unwrap(),
          ),
        ))
      }),
    );

    env.add_function(
      "csc",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `csc` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let sin_val = payload.arguments[0].number(payload.span)?.sin(
          payload.config.precision,
          payload.config.rounding_mode,
          &mut Consts::new().unwrap(),
        );

        if sin_val.is_zero() {
          return Err(Error::new(
            payload.span,
            "Cannot compute csc of multiple of π",
          ));
        }

        Ok(Value::Number(BigFloat::from(1.0).div(
          &sin_val,
          payload.config.precision,
          payload.config.rounding_mode,
        )))
      }),
    );

    env.add_function(
      "sec",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `sec` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let cos_val = payload.arguments[0].number(payload.span)?.cos(
          payload.config.precision,
          payload.config.rounding_mode,
          &mut Consts::new().unwrap(),
        );

        if cos_val.is_zero() {
          return Err(Error::new(
            payload.span,
            "Cannot compute sec of π/2 + nπ",
          ));
        }

        Ok(Value::Number(BigFloat::from(1.0).div(
          &cos_val,
          payload.config.precision,
          payload.config.rounding_mode,
        )))
      }),
    );

    env.add_function(
      "cot",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `cot` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let tan_val = payload.arguments[0].number(payload.span)?.tan(
          payload.config.precision,
          payload.config.rounding_mode,
          &mut Consts::new().unwrap(),
        );

        if tan_val.is_zero() {
          return Err(Error::new(
            payload.span,
            "Cannot compute cot of multiple of π",
          ));
        }

        Ok(Value::Number(BigFloat::from(1.0).div(
          &tan_val,
          payload.config.precision,
          payload.config.rounding_mode,
        )))
      }),
    );

    env.add_function(
      "sinh",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `sinh` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        Ok(Value::Number(
          payload.arguments[0].number(payload.span)?.sinh(
            payload.config.precision,
            payload.config.rounding_mode,
            &mut Consts::new().unwrap(),
          ),
        ))
      }),
    );

    env.add_function(
      "cosh",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `cosh` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        Ok(Value::Number(
          payload.arguments[0].number(payload.span)?.cosh(
            payload.config.precision,
            payload.config.rounding_mode,
            &mut Consts::new().unwrap(),
          ),
        ))
      }),
    );

    env.add_function(
      "tanh",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `tanh` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        Ok(Value::Number(
          payload.arguments[0].number(payload.span)?.tanh(
            payload.config.precision,
            payload.config.rounding_mode,
            &mut Consts::new().unwrap(),
          ),
        ))
      }),
    );

    env.add_function(
      "asin",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `asin` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let argument = payload.arguments[0].number(payload.span)?;

        if argument < BigFloat::from(-1.0) || argument > BigFloat::from(1.0) {
          return Err(Error::new(
            payload.span,
            "asin argument must be between -1 and 1",
          ));
        }

        Ok(Value::Number(argument.asin(
          payload.config.precision,
          payload.config.rounding_mode,
          &mut Consts::new().unwrap(),
        )))
      }),
    );

    env.add_function(
      "acos",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `acos` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let argument = payload.arguments[0].number(payload.span)?;

        if argument < BigFloat::from(-1.0) || argument > BigFloat::from(1.0) {
          return Err(Error::new(
            payload.span,
            "acos argument must be between -1 and 1",
          ));
        }

        Ok(Value::Number(argument.acos(
          payload.config.precision,
          payload.config.rounding_mode,
          &mut Consts::new().unwrap(),
        )))
      }),
    );

    env.add_function(
      "arc",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `arc` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        Ok(Value::Number(
          payload.arguments[0].number(payload.span)?.atan(
            payload.config.precision,
            payload.config.rounding_mode,
            &mut Consts::new().unwrap(),
          ),
        ))
      }),
    );

    env.add_function(
      "acsc",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `acsc` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let argument = payload.arguments[0].number(payload.span)?;

        if argument.abs() < BigFloat::from(1.0) {
          return Err(Error::new(
            payload.span,
            "acsc argument must have absolute value at least 1",
          ));
        }

        Ok(Value::Number(
          (BigFloat::from(1.0).div(
            &argument,
            payload.config.precision,
            payload.config.rounding_mode,
          ))
          .asin(
            payload.config.precision,
            payload.config.rounding_mode,
            &mut Consts::new().unwrap(),
          ),
        ))
      }),
    );

    env.add_function(
      "asec",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `asec` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let argument = payload.arguments[0].number(payload.span)?;

        if argument.abs() < BigFloat::from(1.0) {
          return Err(Error::new(
            payload.span,
            "asec argument must have absolute value at least 1",
          ));
        }

        Ok(Value::Number(
          (BigFloat::from(1.0).div(
            &argument,
            payload.config.precision,
            payload.config.rounding_mode,
          ))
          .acos(
            payload.config.precision,
            payload.config.rounding_mode,
            &mut Consts::new().unwrap(),
          ),
        ))
      }),
    );

    env.add_function(
      "acot",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `acot` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let argument = payload.arguments[0].number(payload.span)?;

        let pi_div_2 = BigFloat::from(std::f64::consts::FRAC_PI_2);

        // Formula: acot(x) = π/2 - atan(x)
        Ok(Value::Number(pi_div_2.sub(
          &argument.atan(
            payload.config.precision,
            payload.config.rounding_mode,
            &mut Consts::new().unwrap(),
          ),
          payload.config.precision,
          payload.config.rounding_mode,
        )))
      }),
    );

    env.add_function(
      "ln",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function 'ln' expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let number = payload.arguments[0].number(payload.span)?;

        if number.is_zero() || number.is_negative() {
          return Err(Error::new(
            payload.span,
            "Cannot take logarithm of zero or negative number",
          ));
        }

        Ok(Value::Number(number.ln(
          payload.config.precision,
          payload.config.rounding_mode,
          &mut Consts::new().unwrap(),
        )))
      }),
    );

    env.add_function(
      "log2",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `log2` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let number = payload.arguments[0].number(payload.span)?;

        if number.is_zero() || number.is_negative() {
          return Err(Error::new(
            payload.span,
            "Cannot take logarithm of zero or negative number",
          ));
        }

        Ok(Value::Number(number.log2(
          payload.config.precision,
          payload.config.rounding_mode,
          &mut Consts::new().unwrap(),
        )))
      }),
    );

    env.add_function(
      "log10",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `log10` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let number = payload.arguments[0].number(payload.span)?;

        if number.is_zero() || number.is_negative() {
          return Err(Error::new(
            payload.span,
            "Cannot take logarithm of zero or negative number",
          ));
        }

        Ok(Value::Number(number.log10(
          payload.config.precision,
          payload.config.rounding_mode,
          &mut Consts::new().unwrap(),
        )))
      }),
    );

    env.add_function(
      "e",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `e` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        Ok(Value::Number(
          payload.arguments[0].number(payload.span)?.exp(
            payload.config.precision,
            payload.config.rounding_mode,
            &mut Consts::new().unwrap(),
          ),
        ))
      }),
    );

    env.add_function(
      "sqrt",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `sqrt` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let number = payload.arguments[0].number(payload.span)?;

        if number.is_negative() {
          return Err(Error::new(
            payload.span,
            "Cannot take square root of negative number",
          ));
        }

        Ok(Value::Number(number.sqrt(
          payload.config.precision,
          payload.config.rounding_mode,
        )))
      }),
    );

    env.add_function(
      "ceil",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `ceil` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        Ok(Value::Number(
          payload.arguments[0].number(payload.span)?.ceil(),
        ))
      }),
    );

    env.add_function(
      "floor",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `floor` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        Ok(Value::Number(
          payload.arguments[0].number(payload.span)?.floor(),
        ))
      }),
    );

    env.add_function(
      "abs",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `abs` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        Ok(Value::Number(
          payload.arguments[0].number(payload.span)?.abs(),
        ))
      }),
    );

    env.add_function(
      "len",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `len` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let value = &payload.arguments[0];

        match value {
          Value::String(s) => Ok(Value::Number(BigFloat::from(s.len() as f64))),
          Value::List(items) => {
            Ok(Value::Number(BigFloat::from(items.len() as f64)))
          }
          _ => Err(Error::new(
            payload.span,
            format!("Cannot get length of {}", value.type_name()),
          )),
        }
      }),
    );

    env.add_function(
      "print",
      Function::Builtin(|payload| {
        let mut output_strings = Vec::with_capacity(payload.arguments.len());

        for argument in &payload.arguments {
          output_strings.push(format!("{}", argument));
        }

        print!("{}", output_strings.join(" "));

        Ok(Value::Null)
      }),
    );

    env.add_function(
      "println",
      Function::Builtin(|payload| {
        let mut output_strings = Vec::with_capacity(payload.arguments.len());

        for argument in &payload.arguments {
          output_strings.push(format!("{}", argument));
        }

        println!("{}", output_strings.join(" "));

        Ok(Value::Null)
      }),
    );

    env.add_function(
      "exit",
      Function::Builtin(|payload| {
        if payload.arguments.is_empty() {
          process::exit(0);
        }

        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `exit` expects 0 or 1 arguments, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let code = match payload.arguments[0]
          .number(payload.span)?
          .to_f64(payload.config.rounding_mode)
        {
          Some(n) if n.is_finite() && n >= 0.0 => n as usize,
          _ => {
            return Err(Error::new(
              payload.span,
              "Argument to `exit` must be a non-negative finite number",
            ));
          }
        };

        process::exit(code as i32);
      }),
    );

    env.add_function(
      "quit",
      Function::Builtin(|payload| {
        if payload.arguments.is_empty() {
          process::exit(0);
        }

        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `quit` expects 0 or 1 arguments, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let code = match payload.arguments[0]
          .number(payload.span)?
          .to_f64(payload.config.rounding_mode)
        {
          Some(n) if n.is_finite() && n >= 0.0 => n as usize,
          _ => {
            return Err(Error::new(
              payload.span,
              "Argument to `quit` must be a non-negative finite number",
            ));
          }
        };

        process::exit(code as i32);
      }),
    );

    env.add_function(
      "sum",
      Function::Builtin(|payload| {
        if payload.arguments.is_empty() {
          return Err(Error::new(
            payload.span,
            "Function `sum` expects at least one argument",
          ));
        }

        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `sum` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let list = payload.arguments[0].list(payload.span)?;

        if list.is_empty() {
          return Ok(Value::Number(BigFloat::from(0.0)));
        }

        let mut sum = list[0].number(payload.span)?;

        for val in list.iter().skip(1) {
          sum = sum.add(
            &val.number(payload.span)?,
            payload.config.precision,
            payload.config.rounding_mode,
          );
        }

        Ok(Value::Number(sum))
      }),
    );

    env.add_function(
      "input",
      Function::Builtin(|payload| {
        use std::io::{self, BufRead, Write};

        if payload.arguments.len() > 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `input` expects 0 or 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        if payload.arguments.len() == 1 {
          print!("{}", payload.arguments[0].string(payload.span)?);
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
          Err(e) => Err(Error::new(
            payload.span,
            format!("Failed to read input: {}", e),
          )),
        }
      }),
    );

    env.add_function(
      "int",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `int` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let value = &payload.arguments[0];

        match value {
          Value::Number(n) => Ok(Value::Number(n.floor())),
          Value::String(s) => match s.trim().parse::<f64>() {
            Ok(n) => Ok(Value::Number(BigFloat::from(n).floor())),
            Err(_) => Err(Error::new(
              payload.span,
              format!("Cannot convert '{}' to int", s),
            )),
          },
          Value::Boolean(b) => {
            Ok(Value::Number(BigFloat::from(if *b { 1.0 } else { 0.0 })))
          }
          _ => Err(Error::new(
            payload.span,
            format!("Cannot convert {} to int", value.type_name()),
          )),
        }
      }),
    );

    env.add_function(
      "float",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `float` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let value = &payload.arguments[0];

        match value {
          Value::Number(n) => Ok(Value::Number(n.clone())),
          Value::String(s) => match s.trim().parse::<f64>() {
            Ok(n) => Ok(Value::Number(BigFloat::from(n))),
            Err(_) => Err(Error::new(
              payload.span,
              format!("Cannot convert '{}' to float", s),
            )),
          },
          Value::Boolean(b) => {
            Ok(Value::Number(BigFloat::from(if *b { 1.0 } else { 0.0 })))
          }
          _ => Err(Error::new(
            payload.span,
            format!("Cannot convert {} to float", value.type_name()),
          )),
        }
      }),
    );

    env.add_function(
      "bool",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `bool` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let value = &payload.arguments[0];

        match value {
          Value::Boolean(b) => Ok(Value::Boolean(*b)),
          Value::Number(n) => Ok(Value::Boolean(!n.is_zero())),
          Value::String(s) => Ok(Value::Boolean(!s.is_empty())),
          Value::List(items) => Ok(Value::Boolean(!items.is_empty())),
          Value::Null => Ok(Value::Boolean(false)),
          _ => Err(Error::new(
            payload.span,
            format!("Cannot convert {} to bool", value.type_name()),
          )),
        }
      }),
    );

    env.add_function(
      "list",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 1 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `list` expects 1 argument, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let value = &payload.arguments[0];

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
      Function::Builtin(|payload| {
        if payload.arguments.len() != 2 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `split` expects 2 arguments, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let string = payload.arguments[0].string(payload.span)?;

        let delimiter = payload.arguments[1].string(payload.span)?;

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

    env.add_function(
      "join",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 2 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `join` expects 2 arguments, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let list = payload.arguments[0].list(payload.span)?;

        let delimiter = payload.arguments[1].string(payload.span)?;

        let joined_string = list
          .iter()
          .map(|value| match value {
            Value::String(s) => s.to_string(),
            _ => value.to_string(),
          })
          .collect::<Vec<_>>()
          .join(delimiter);

        Ok(Value::String(Box::leak(joined_string.into_boxed_str())))
      }),
    );

    env.add_function(
      "append",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 2 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `append` expects 2 arguments, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let mut list = payload.arguments[0].list(payload.span)?;

        let value = payload.arguments[1].clone();

        list.push(value);

        Ok(Value::List(list))
      }),
    );

    env.add_function(
      "gcd",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 2 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `gcd` expects 2 arguments, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let a = payload.arguments[0].number(payload.span)?;
        let b = payload.arguments[1].number(payload.span)?;

        let mut x = a.abs();
        let mut y = b.abs();

        while !y.is_zero() {
          let remainder = x.rem(&y);
          x = y;
          y = remainder;
        }

        Ok(Value::Number(x))
      }),
    );

    env.add_function(
      "lcm",
      Function::Builtin(|payload| {
        if payload.arguments.len() != 2 {
          return Err(Error::new(
            payload.span,
            format!(
              "Function `lcm` expects 2 arguments, got {}",
              payload.arguments.len()
            ),
          ));
        }

        let a = payload.arguments[0].number(payload.span)?;
        let b = payload.arguments[1].number(payload.span)?;

        if a.is_zero() || b.is_zero() {
          return Ok(Value::Number(BigFloat::from(0)));
        }

        let mut x = a.abs();
        let mut y = b.abs();

        let product =
          x.mul(&y, payload.config.precision, payload.config.rounding_mode);

        while !y.is_zero() {
          let remainder = x.rem(&y);
          x = y;
          y = remainder;
        }

        let lcm = product.div(
          &x,
          payload.config.precision,
          payload.config.rounding_mode,
        );

        Ok(Value::Number(lcm))
      }),
    );

    let mut consts = Consts::new().unwrap();

    env.add_variable(
      "e",
      Value::Number(consts.e(config.precision, config.rounding_mode)),
    );

    env.add_variable(
      "pi",
      Value::Number(consts.pi(config.precision, config.rounding_mode)),
    );

    env.add_variable(
      "tau",
      Value::Number(BigFloat::from_f64(
        std::f64::consts::TAU,
        config.precision,
      )),
    );

    env.add_variable(
      "phi",
      Value::Number(BigFloat::from_f64(
        1.618_033_988_749_895_f64,
        config.precision,
      )),
    );

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
        Function::Builtin(function) => function(BuiltinFunctionPayload {
          arguments,
          config: self.config.clone(),
          span,
        }),
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
            if matches!(argument, Value::Function(_, _, _, _)) {
              call_environment.add_function(
                parameter,
                Function::UserDefined(argument.clone()),
              )
            } else {
              call_environment.add_variable(parameter, argument.clone())
            }
          }

          let mut evaluator = Evaluator::from(call_environment);
          evaluator.inside_function = true;

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

  pub fn resolve_symbol(&self, symbol: &str) -> Option<&Value<'src>> {
    if let Some(value) = self.variables.get(symbol) {
      Some(value)
    } else if let Some(function) = self.functions.get(symbol) {
      match function {
        Function::UserDefined(value) => Some(value),
        Function::Builtin(_) => None, // We should support this at some point
      }
    } else if let Some(parent) = &self.parent {
      parent.resolve_symbol(symbol)
    } else {
      None
    }
  }

  pub fn with_parent(parent: Environment<'src>) -> Self {
    Self {
      config: parent.config.clone(),
      functions: parent.functions.clone(),
      parent: Some(Box::new(parent)),
      variables: HashMap::new(),
    }
  }
}
