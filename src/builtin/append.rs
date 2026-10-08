use super::*;

builtin! {
  Append {
    name: "append",
    arity: BuiltinArity::Exact(2),
    call(payload) {
      let mut arguments = payload.arguments.into_iter();

      let mut list = arguments.next().unwrap().into_list(payload.span)?;

      list.push(arguments.next().unwrap());

      Ok(Value::List(list))
    }
  }
}
