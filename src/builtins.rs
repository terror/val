use super::*;

pub(crate) const BUILTINS: &[Builtin] = &[
  Builtin::Constant {
    name: "e",
    value: constant_e,
  },
  Builtin::Constant {
    name: "phi",
    value: constant_phi,
  },
  Builtin::Constant {
    name: "pi",
    value: constant_pi,
  },
  Builtin::Constant {
    name: "tau",
    value: constant_tau,
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(abs),
    name: "abs",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(acos),
    name: "acos",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(acot),
    name: "acot",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(acsc),
    name: "acsc",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(2),
    function: BuiltinFunction::Fallible(append),
    name: "append",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(arc),
    name: "arc",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(asec),
    name: "asec",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(asin),
    name: "asin",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(r#bool),
    name: "bool",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(ceil),
    name: "ceil",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(cos),
    name: "cos",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(cosh),
    name: "cosh",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(cot),
    name: "cot",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(csc),
    name: "csc",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(e),
    name: "e",
  },
  Builtin::Function {
    arity: BuiltinArity::Range(0, 1),
    function: BuiltinFunction::Fallible(exit),
    name: "exit",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(float),
    name: "float",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(floor),
    name: "floor",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(2),
    function: BuiltinFunction::Fallible(gcd),
    name: "gcd",
  },
  Builtin::Function {
    arity: BuiltinArity::Range(0, 1),
    function: BuiltinFunction::Fallible(input),
    name: "input",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(int),
    name: "int",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(2),
    function: BuiltinFunction::Fallible(join),
    name: "join",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(2),
    function: BuiltinFunction::Fallible(lcm),
    name: "lcm",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(len),
    name: "len",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Infallible(list),
    name: "list",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(ln),
    name: "ln",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(log10),
    name: "log10",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(log2),
    name: "log2",
  },
  Builtin::Function {
    arity: BuiltinArity::Any,
    function: BuiltinFunction::Fallible(print),
    name: "print",
  },
  Builtin::Function {
    arity: BuiltinArity::Any,
    function: BuiltinFunction::Fallible(println),
    name: "println",
  },
  Builtin::Function {
    arity: BuiltinArity::Range(0, 1),
    function: BuiltinFunction::Fallible(quit),
    name: "quit",
  },
  Builtin::Function {
    arity: BuiltinArity::Range(2, 3),
    function: BuiltinFunction::Fallible(range),
    name: "range",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(sec),
    name: "sec",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(sin),
    name: "sin",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(sinh),
    name: "sinh",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(2),
    function: BuiltinFunction::Fallible(split),
    name: "split",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(sqrt),
    name: "sqrt",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(sum),
    name: "sum",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(tan),
    name: "tan",
  },
  Builtin::Function {
    arity: BuiltinArity::Exact(1),
    function: BuiltinFunction::Fallible(tanh),
    name: "tanh",
  },
];

fn abs<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  Ok(Value::Number(
    payload.arguments[0].number(payload.span)?.abs(),
  ))
}

fn acos<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let argument = payload.arguments[0].number(payload.span)?;

  if argument < Number::from(-1_i64) || argument > Number::from(1_i64) {
    return Err(Error::new(
      payload.span,
      "acos argument must be between -1 and 1",
    ));
  }

  Ok(Value::Number(argument.acos(payload.config)))
}

fn acot<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let argument = payload.arguments[0].number(payload.span)?;

  let pi_div_2 = Number::Approx(
    Float::with_val_round(
      payload.config.precision,
      Constant::Pi,
      payload.config.rounding_mode,
    )
    .0,
  )
  .div(&Number::from(2_i64), payload.config);

  Ok(Value::Number(
    pi_div_2.sub(&argument.atan(payload.config), payload.config),
  ))
}

fn acsc<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let argument = payload.arguments[0].number(payload.span)?;

  if argument.abs() < Number::from(1_i64) {
    return Err(Error::new(
      payload.span,
      "acsc argument must have absolute value at least 1",
    ));
  }

  let reciprocal = Number::from(1_i64).div(&argument, payload.config);

  Ok(Value::Number(reciprocal.asin(payload.config)))
}

fn append<'a>(
  payload: &BuiltinFunctionPayload<'a>,
) -> Result<Value<'a>, Error> {
  let mut list = payload.arguments[0].list(payload.span)?;

  list.push(payload.arguments[1].clone());

  Ok(Value::List(list))
}

fn arc<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  Ok(Value::Number(
    payload.arguments[0]
      .number(payload.span)?
      .atan(payload.config),
  ))
}

fn asec<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let argument = payload.arguments[0].number(payload.span)?;

  if argument.abs() < Number::from(1_i64) {
    return Err(Error::new(
      payload.span,
      "asec argument must have absolute value at least 1",
    ));
  }

  let reciprocal = Number::from(1_i64).div(&argument, payload.config);

  Ok(Value::Number(reciprocal.acos(payload.config)))
}

fn asin<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let argument = payload.arguments[0].number(payload.span)?;

  if argument < Number::from(-1_i64) || argument > Number::from(1_i64) {
    return Err(Error::new(
      payload.span,
      "asin argument must be between -1 and 1",
    ));
  }

  Ok(Value::Number(argument.asin(payload.config)))
}

fn r#bool<'a>(
  payload: &BuiltinFunctionPayload<'a>,
) -> Result<Value<'a>, Error> {
  let value = &payload.arguments[0];

  match value {
    Value::Boolean(b) => Ok(Value::Boolean(*b)),
    Value::Number(n) => Ok(Value::Boolean(!n.is_zero())),
    Value::String(s) => Ok(Value::Boolean(!s.is_empty())),
    Value::List(items) => Ok(Value::Boolean(!items.is_empty())),
    Value::Null => Ok(Value::Boolean(false)),
    Value::Function(_) => Err(Error::new(
      payload.span,
      format!("Cannot convert {} to bool", value.type_name()),
    )),
  }
}

fn ceil<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  Ok(Value::Number(
    payload.arguments[0].number(payload.span)?.ceil(),
  ))
}

fn constant_e(config: Config) -> Number {
  Number::e(config)
}

fn constant_phi(config: Config) -> Number {
  Number::from(1_i64)
    .add(&Number::from(5_i64).sqrt(config), config)
    .div(&Number::from(2_i64), config)
}

fn constant_pi(config: Config) -> Number {
  Number::Approx(
    Float::with_val_round(config.precision, Constant::Pi, config.rounding_mode)
      .0,
  )
}

fn constant_tau(config: Config) -> Number {
  Number::tau(config)
}

fn cos<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  Ok(Value::Number(
    payload.arguments[0]
      .number(payload.span)?
      .cos(payload.config),
  ))
}

fn cosh<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  Ok(Value::Number(
    payload.arguments[0]
      .number(payload.span)?
      .cosh(payload.config),
  ))
}

fn cot<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let tan = payload.arguments[0]
    .number(payload.span)?
    .tan(payload.config);

  if tan.is_zero() {
    return Err(Error::new(
      payload.span,
      "Cannot compute cot of multiple of π",
    ));
  }

  Ok(Value::Number(Number::from(1_i64).div(&tan, payload.config)))
}

fn csc<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let sin = payload.arguments[0]
    .number(payload.span)?
    .sin(payload.config);

  if sin.is_zero() {
    return Err(Error::new(
      payload.span,
      "Cannot compute csc of multiple of π",
    ));
  }

  Ok(Value::Number(Number::from(1_i64).div(&sin, payload.config)))
}

fn e<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  Ok(Value::Number(
    payload.arguments[0]
      .number(payload.span)?
      .exp(payload.config),
  ))
}

fn exit<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  if payload.arguments.is_empty() {
    process::exit(0);
  }

  let Some(code) = payload.arguments[0]
    .number(payload.span)?
    .to_non_negative_usize()
  else {
    return Err(Error::new(
      payload.span,
      "Argument to `exit` must be a non-negative finite number",
    ));
  };

  let Ok(code) = i32::try_from(code) else {
    return Err(Error::new(
      payload.span,
      "Argument to `exit` must fit in a 32-bit signed integer",
    ));
  };

  process::exit(code);
}

fn float<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let value = &payload.arguments[0];

  match value {
    Value::Number(number) => {
      Ok(Value::Number(number.to_approx(payload.config)))
    }
    Value::String(s) => Number::try_from(s.as_ref())
      .map(|number| Value::Number(number.to_approx(payload.config)))
      .map_err(|_| {
        Error::new(payload.span, format!("Cannot convert '{s}' to float"))
      }),
    Value::Boolean(b) => {
      Ok(Value::Number(Number::from(*b).to_approx(payload.config)))
    }
    _ => Err(Error::new(
      payload.span,
      format!("Cannot convert {} to float", value.type_name()),
    )),
  }
}

fn floor<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  Ok(Value::Number(
    payload.arguments[0].number(payload.span)?.floor(),
  ))
}

fn gcd<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let Some(a) = payload.arguments[0].number(payload.span)?.to_integer() else {
    return Err(Error::new(
      payload.span,
      "Arguments to `gcd` must be finite integers",
    ));
  };

  let Some(b) = payload.arguments[1].number(payload.span)?.to_integer() else {
    return Err(Error::new(
      payload.span,
      "Arguments to `gcd` must be finite integers",
    ));
  };

  let a = a.abs();
  let b = b.abs();

  Ok(Value::Number(Number::from(a.gcd(&b))))
}

fn input<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  use std::io::{self, BufRead, Write};

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

      Ok(Value::String(Cow::Owned(input)))
    }
    Err(e) => Err(Error::new(
      payload.span,
      format!("Failed to read input: {e}"),
    )),
  }
}

fn int<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let value = &payload.arguments[0];

  match value {
    Value::Number(number) => Ok(Value::Number(number.floor())),
    Value::String(s) => Number::try_from(s.as_ref())
      .map(|number| Value::Number(number.floor()))
      .map_err(|_| {
        Error::new(payload.span, format!("Cannot convert '{s}' to int"))
      }),
    Value::Boolean(b) => Ok(Value::Number(Number::from(*b))),
    _ => Err(Error::new(
      payload.span,
      format!("Cannot convert {} to int", value.type_name()),
    )),
  }
}

fn join<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let list = payload.arguments[0].list(payload.span)?;

  let delimiter = payload.arguments[1].string(payload.span)?;

  let joined_string = list
    .iter()
    .map(|value| match value {
      Value::String(s) => s.to_string(),
      _ => value.display(payload.config),
    })
    .collect::<Vec<_>>()
    .join(delimiter);

  Ok(Value::String(Cow::Owned(joined_string)))
}

fn lcm<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let Some(a) = payload.arguments[0].number(payload.span)?.to_integer() else {
    return Err(Error::new(
      payload.span,
      "Arguments to `lcm` must be finite integers",
    ));
  };

  let Some(b) = payload.arguments[1].number(payload.span)?.to_integer() else {
    return Err(Error::new(
      payload.span,
      "Arguments to `lcm` must be finite integers",
    ));
  };

  let a = a.abs();
  let b = b.abs();

  Ok(Value::Number(Number::from(a.lcm(&b))))
}

fn len<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let value = &payload.arguments[0];

  match value {
    Value::String(s) => Ok(Value::Number(Number::from(s.len()))),
    Value::List(items) => Ok(Value::Number(Number::from(items.len()))),
    _ => Err(Error::new(
      payload.span,
      format!("Cannot get length of {}", value.type_name()),
    )),
  }
}

fn list<'a>(payload: &BuiltinFunctionPayload<'a>) -> Value<'a> {
  let value = &payload.arguments[0];

  match value {
    Value::List(items) => Value::List(items.clone()),
    Value::String(s) => Value::List(
      s.chars()
        .map(|c| Value::String(Cow::Owned(c.to_string())))
        .collect(),
    ),
    _ => Value::List(vec![value.clone()]),
  }
}

fn ln<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let number = payload.arguments[0].number(payload.span)?;

  if number.is_zero() || number.is_negative() {
    return Err(Error::new(
      payload.span,
      "Cannot take logarithm of zero or negative number",
    ));
  }

  Ok(Value::Number(number.ln(payload.config)))
}

fn log10<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let number = payload.arguments[0].number(payload.span)?;

  if number.is_zero() || number.is_negative() {
    return Err(Error::new(
      payload.span,
      "Cannot take logarithm of zero or negative number",
    ));
  }

  Ok(Value::Number(number.log10(payload.config)))
}

fn log2<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let number = payload.arguments[0].number(payload.span)?;

  if number.is_zero() || number.is_negative() {
    return Err(Error::new(
      payload.span,
      "Cannot take logarithm of zero or negative number",
    ));
  }

  Ok(Value::Number(number.log2(payload.config)))
}

fn print<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  use std::io::Write;

  let mut output_strings = Vec::with_capacity(payload.arguments.len());

  for argument in &payload.arguments {
    output_strings.push(argument.display(payload.config));
  }

  write!(std::io::stdout(), "{}", output_strings.join(" "))
    .map_err(|error| Error::new(payload.span, error.to_string()))?;

  Ok(Value::Null)
}

fn println<'a>(
  payload: &BuiltinFunctionPayload<'a>,
) -> Result<Value<'a>, Error> {
  use std::io::Write;

  let mut output_strings = Vec::with_capacity(payload.arguments.len());

  for argument in &payload.arguments {
    output_strings.push(argument.display(payload.config));
  }

  writeln!(std::io::stdout(), "{}", output_strings.join(" "))
    .map_err(|error| Error::new(payload.span, error.to_string()))?;

  Ok(Value::Null)
}

fn quit<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  if payload.arguments.is_empty() {
    process::exit(0);
  }

  let Some(code) = payload.arguments[0]
    .number(payload.span)?
    .to_non_negative_usize()
  else {
    return Err(Error::new(
      payload.span,
      "Argument to `quit` must be a non-negative finite number",
    ));
  };

  let Ok(code) = i32::try_from(code) else {
    return Err(Error::new(
      payload.span,
      "Argument to `quit` must fit in a 32-bit signed integer",
    ));
  };

  process::exit(code);
}

fn range<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let mut numbers = Vec::with_capacity(payload.arguments.len());

  for argument in &payload.arguments {
    match argument.number(payload.span)?.to_i64() {
      Some(number) => {
        numbers.push(number);
      }
      None => {
        return Err(Error::new(
          payload.span,
          "Arguments to `range` must be finite integers",
        ));
      }
    }
  }

  let (start, end) = (numbers[0], numbers[1]);
  let step = numbers.get(2).copied().unwrap_or(1);

  if step == 0 {
    return Err(Error::new(
      payload.span,
      "Step argument to `range` must not be zero",
    ));
  }

  let mut current = start;
  let mut result = Vec::new();

  while if step > 0 {
    current < end
  } else {
    current > end
  } {
    result.push(Value::Number(Number::from(current)));

    current = current
      .checked_add(step)
      .ok_or_else(|| Error::new(payload.span, "`range` overflowed"))?;
  }

  Ok(Value::List(result))
}

fn sec<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let cos = payload.arguments[0]
    .number(payload.span)?
    .cos(payload.config);

  if cos.is_zero() {
    return Err(Error::new(payload.span, "Cannot compute sec of π/2 + nπ"));
  }

  Ok(Value::Number(Number::from(1_i64).div(&cos, payload.config)))
}

fn sin<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  Ok(Value::Number(
    payload.arguments[0]
      .number(payload.span)?
      .sin(payload.config),
  ))
}

fn sinh<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  Ok(Value::Number(
    payload.arguments[0]
      .number(payload.span)?
      .sinh(payload.config),
  ))
}

fn split<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let string = payload.arguments[0].string(payload.span)?;

  let delimiter = payload.arguments[1].string(payload.span)?;

  Ok(Value::List(
    string
      .split(delimiter)
      .filter(|part| !part.is_empty())
      .map(|part| Value::String(Cow::Owned(part.to_string())))
      .collect(),
  ))
}

fn sqrt<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let number = payload.arguments[0].number(payload.span)?;

  if number.is_negative() {
    return Err(Error::new(
      payload.span,
      "Cannot take square root of negative number",
    ));
  }

  Ok(Value::Number(number.sqrt(payload.config)))
}

fn sum<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  let list = payload.arguments[0].list(payload.span)?;

  let mut sum = Number::from(0_i64);

  for value in list {
    sum = sum.add(&value.number(payload.span)?, payload.config);
  }

  Ok(Value::Number(sum))
}

fn tan<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  Ok(Value::Number(
    payload.arguments[0]
      .number(payload.span)?
      .tan(payload.config),
  ))
}

fn tanh<'a>(payload: &BuiltinFunctionPayload<'a>) -> Result<Value<'a>, Error> {
  Ok(Value::Number(
    payload.arguments[0]
      .number(payload.span)?
      .tanh(payload.config),
  ))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn alphabetical_by_kind() {
    #[track_caller]
    fn case(kind: &str, names: impl IntoIterator<Item = &'static str>) {
      let names = names.into_iter().collect::<Vec<_>>();

      for window in names.windows(2) {
        assert!(
          window[0] < window[1],
          "{kind} names out of order in BUILTINS: {:?} before {:?}",
          window[0],
          window[1],
        );
      }
    }

    let mut previous_kind = "";

    for builtin in BUILTINS {
      let kind = builtin.kind();

      assert!(
        previous_kind <= kind,
        "builtin kinds out of order in BUILTINS: {previous_kind:?} before {kind:?}",
      );

      previous_kind = kind;
    }

    case(
      "constant",
      BUILTINS.iter().filter_map(|builtin| match builtin {
        Builtin::Constant { name, .. } => Some(*name),
        Builtin::Function { .. } => None,
      }),
    );

    case(
      "function",
      BUILTINS.iter().filter_map(|builtin| match builtin {
        Builtin::Function { name, .. } => Some(*name),
        Builtin::Constant { .. } => None,
      }),
    );
  }
}
