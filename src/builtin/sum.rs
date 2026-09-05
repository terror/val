use super::*;

builtin! {
  Sum {
    name: "sum",
    arity: BuiltinArity::Exact(1),
    call(payload) {
      let list = payload.arguments[0].list(payload.span)?;

      let mut sum = Number::from(0_i64);

      for value in list {
        sum = sum.add(value.number(payload.span)?, payload.config);
      }

      Ok(Value::Number(sum))
    }
  }
}
