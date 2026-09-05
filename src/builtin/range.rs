use super::*;

builtin! {
  Range {
    name: "range",
    arity: BuiltinArity::Range(2, 3),
    call(payload) {
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

        let Some(next) = current.checked_add(step) else {
          break;
        };

        current = next;
      }

      Ok(Value::List(result))
    }
  }
}
