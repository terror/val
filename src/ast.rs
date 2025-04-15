use super::*;

#[derive(Debug, Clone)]
pub enum Program<'a> {
  Statements(Vec<Spanned<Statement<'a>>>),
}

impl Display for Program<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Program::Statements(statements) => {
        write!(
          f,
          "statements({})",
          statements
            .iter()
            .map(|s| s.0.to_string())
            .collect::<Vec<_>>()
            .join(", ")
        )
      }
    }
  }
}

impl Program<'_> {
  pub fn kind(&self) -> String {
    String::from(match self {
      Program::Statements(_) => "statements",
    })
  }
}

#[derive(Debug, Clone)]
pub enum Statement<'a> {
  Assignment(&'a str, Spanned<Expression<'a>>),
  Block(Vec<Spanned<Statement<'a>>>),
  Expression(Spanned<Expression<'a>>),
  Function(&'a str, Vec<&'a str>, Vec<Spanned<Statement<'a>>>),
  If(
    Spanned<Expression<'a>>,
    Vec<Spanned<Statement<'a>>>,
    Option<Vec<Spanned<Statement<'a>>>>,
  ),
  While(Spanned<Expression<'a>>, Vec<Spanned<Statement<'a>>>),
}

impl Display for Statement<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Statement::Assignment(name, expression) => {
        write!(f, "assignment({}, {})", name, expression.0)
      }
      Statement::Block(statements) => {
        write!(
          f,
          "block({})",
          statements
            .iter()
            .map(|s| s.0.to_string())
            .collect::<Vec<_>>()
            .join(", ")
        )
      }
      Statement::Expression(expression) => {
        write!(f, "expression({})", expression.0)
      }
      Statement::Function(name, params, body) => {
        write!(
          f,
          "function({}, [{}], block({}))",
          name,
          params.join(", "),
          body
            .iter()
            .map(|s| s.0.to_string())
            .collect::<Vec<_>>()
            .join(", ")
        )
      }
      Statement::If(condition, then_branch, else_branch) => {
        let then_str = then_branch
          .iter()
          .map(|s| s.0.to_string())
          .collect::<Vec<_>>()
          .join(", ");

        match else_branch {
          Some(else_stmts) => {
            let else_str = else_stmts
              .iter()
              .map(|s| s.0.to_string())
              .collect::<Vec<_>>()
              .join(", ");
            write!(
              f,
              "if({}, block({}), block({}))",
              condition.0, then_str, else_str
            )
          }
          None => {
            write!(f, "if({}, block({}))", condition.0, then_str)
          }
        }
      }
      Statement::While(condition, body) => {
        write!(
          f,
          "while({}, block({}))",
          condition.0,
          body
            .iter()
            .map(|s| s.0.to_string())
            .collect::<Vec<_>>()
            .join(", ")
        )
      }
    }
  }
}

impl Statement<'_> {
  pub fn kind(&self) -> String {
    String::from(match self {
      Statement::Assignment(_, _) => "assignment",
      Statement::Block(_) => "block",
      Statement::Expression(_) => "expression",
      Statement::Function(_, _, _) => "function",
      Statement::If(_, _, _) => "if",
      Statement::While(_, _) => "while",
    })
  }
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
  Negate,
  Not,
}

impl Display for UnaryOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      UnaryOp::Negate => write!(f, "-"),
      UnaryOp::Not => write!(f, "!"),
    }
  }
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
  Add,
  Divide,
  Equal,
  GreaterThan,
  GreaterThanEqual,
  LessThan,
  LessThanEqual,
  Modulo,
  Multiply,
  NotEqual,
  Power,
  Subtract,
}

impl Display for BinaryOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      BinaryOp::Add => write!(f, "+"),
      BinaryOp::Divide => write!(f, "/"),
      BinaryOp::Equal => write!(f, "=="),
      BinaryOp::GreaterThanEqual => write!(f, ">="),
      BinaryOp::GreaterThan => write!(f, ">"),
      BinaryOp::LessThanEqual => write!(f, "<="),
      BinaryOp::LessThan => write!(f, "<"),
      BinaryOp::Modulo => write!(f, "%"),
      BinaryOp::Multiply => write!(f, "*"),
      BinaryOp::NotEqual => write!(f, "!="),
      BinaryOp::Power => write!(f, "^"),
      BinaryOp::Subtract => write!(f, "-"),
    }
  }
}

#[derive(Debug, Clone)]
pub enum Expression<'a> {
  BinaryOp(BinaryOp, Box<Spanned<Self>>, Box<Spanned<Self>>),
  Boolean(bool),
  FunctionCall(&'a str, Vec<Spanned<Self>>),
  Identifier(&'a str),
  List(Vec<Spanned<Self>>),
  ListAccess(Box<Spanned<Self>>, Box<Spanned<Self>>),
  Number(f64),
  String(&'a str),
  UnaryOp(UnaryOp, Box<Spanned<Self>>),
}

impl Display for Expression<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Expression::BinaryOp(op, lhs, rhs) => {
        write!(f, "binary_op({}, {}, {})", op, lhs.0, rhs.0)
      }
      Expression::Boolean(boolean) => write!(f, "boolean({})", boolean),
      Expression::FunctionCall(name, arguments) => {
        write!(
          f,
          "function_call({},{})",
          name,
          arguments
            .iter()
            .map(|a| a.0.to_string())
            .collect::<Vec<_>>()
            .join(", ")
        )
      }
      Expression::Identifier(identifier) => {
        write!(f, "identifier({})", identifier)
      }
      Expression::List(list) => {
        write!(
          f,
          "list({})",
          list
            .iter()
            .map(|item| item.0.to_string())
            .collect::<Vec<_>>()
            .join(", ")
        )
      }
      Expression::ListAccess(list, index) => {
        write!(f, "list_access({}, {})", list.0, index.0)
      }
      Expression::Number(number) => write!(f, "number({})", number),
      Expression::String(string) => write!(f, "string(\"{}\")", string),
      Expression::UnaryOp(op, expr) => {
        write!(f, "unary_op({}, {})", op, expr.0)
      }
    }
  }
}

impl Expression<'_> {
  pub fn kind(&self) -> String {
    String::from(match self {
      Expression::BinaryOp(_, _, _) => "binary_op",
      Expression::Boolean(_) => "boolean",
      Expression::FunctionCall(_, _) => "function_call",
      Expression::Identifier(_) => "identifier",
      Expression::List(_) => "list",
      Expression::ListAccess(_, _) => "list_access",
      Expression::Number(_) => "number",
      Expression::String(_) => "string",
      Expression::UnaryOp(_, _) => "unary_op",
    })
  }
}
