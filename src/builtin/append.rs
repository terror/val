use super::*;

builtin! {
  Append {
    name: "append",
    arity: BuiltinArity::Exact(2),
    call(payload) {
      let mut list = payload.arguments[0].list(payload.span)?.to_vec();

      list.push(payload.arguments[1].clone());

      Ok(Value::List(list))
    }
  }
}
