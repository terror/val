use {
  ariadne::{Color, Label, Report, ReportKind, Source},
  chumsky::prelude::*,
  clap::Parser as Clap,
  std::{
    fmt::{Display, Formatter},
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
  },
};

type Span = SimpleSpan<usize>;
type Spanned<T> = (T, Span);

#[derive(Debug, Clone)]
enum UnaryOp {
  Neg,
}

#[derive(Debug, Clone)]
enum BinaryOp {
  Add,
  Div,
  Mod,
  Mul,
  Sub,
}

#[derive(Debug)]
enum Ast<'a> {
  BinaryOp(BinaryOp, Box<Spanned<Self>>, Box<Spanned<Self>>),
  Call(&'a str, Vec<Spanned<Self>>),
  Error,
  Identifier(&'a str),
  Number(f64),
  UnaryOp(UnaryOp, Box<Spanned<Self>>),
}

#[derive(Clone, Debug, PartialEq)]
enum Value<'src> {
  Null,
  Bool(bool),
  Num(f64),
  Str(&'src str),
  List(Vec<Self>),
  Func(&'src str),
}

impl Display for Value<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Null => write!(f, "null"),
      Value::Bool(b) => write!(f, "{}", b),
      Value::Num(n) => write!(f, "{}", n),
      Value::Str(s) => write!(f, "{}", s),
      Value::List(l) => write!(f, "{:?}", l),
      Value::Func(s) => write!(f, "<function: {}>", s),
    }
  }
}

#[derive(Debug)]
struct Error {
  span: Span,
  message: String,
}

impl Error {
  fn new(span: Span, message: impl Into<String>) -> Self {
    Self {
      span,
      message: message.into(),
    }
  }
}

impl Value<'_> {
  fn num(self, span: Span) -> Result<f64, Error> {
    if let Value::Num(x) = self {
      Ok(x)
    } else {
      Err(Error {
        span,
        message: format!("'{}' is not a number", self),
      })
    }
  }
}

fn parser<'a>()
-> impl Parser<'a, &'a str, Spanned<Ast<'a>>, extra::Err<Rich<'a, char>>> {
  let ident = text::ident().padded();

  let expr = recursive(|_| {
    let number = text::int(10)
      .map(|s: &str| Ast::Number(s.parse().unwrap()))
      .map_with(|ast, e| (ast, e.span()));

    let atom = number.or(
      ident
        .map(Ast::Identifier)
        .map_with(|ast, e| (ast, e.span())),
    );

    let op = |c| just(c).padded();

    let unary = op('-').repeated().foldr(atom, |_, rhs| {
      let span = rhs.1;
      (Ast::UnaryOp(UnaryOp::Neg, Box::new(rhs)), span)
    });

    unary
  });

  expr
}

fn eval<'a>(ast: &Spanned<Ast<'a>>) -> Result<Value<'a>, Error> {
  let (node, span) = ast;

  match node {
    Ast::Number(n) => Ok(Value::Num(*n)),
    Ast::UnaryOp(UnaryOp::Neg, rhs) => Ok(Value::Num(-eval(rhs)?.num(rhs.1)?)),
    Ast::BinaryOp(op, lhs, rhs) => {
      let lhs_val = eval(lhs)?;
      let rhs_val = eval(rhs)?;

      let lhs_num = lhs_val.num(lhs.1)?;
      let rhs_num = rhs_val.num(rhs.1)?;

      match op {
        BinaryOp::Add => Ok(Value::Num(lhs_num + rhs_num)),
        BinaryOp::Sub => Ok(Value::Num(lhs_num - rhs_num)),
        BinaryOp::Mul => Ok(Value::Num(lhs_num * rhs_num)),
        BinaryOp::Div => {
          if rhs_num == 0.0 {
            return Err(Error::new(rhs.1, "Division by zero"));
          }

          Ok(Value::Num(lhs_num / rhs_num))
        }
        BinaryOp::Mod => {
          if rhs_num == 0.0 {
            return Err(Error::new(rhs.1, "Modulo by zero"));
          }
          Ok(Value::Num(lhs_num % rhs_num))
        }
      }
    }
    Ast::Identifier(name) => {
      Err(Error::new(*span, format!("Undefined variable '{}'", name)))
    }
    Ast::Call(func_name, _) => Err(Error::new(
      *span,
      format!("Function '{}' is not implemented", func_name),
    )),
    Ast::Error => Err(Error::new(*span, "Syntax error")),
  }
}

fn report_error(source_id: &str, source_content: &str, error: &Error) {
  let span_range = error.span.into_range();

  let mut report =
    Report::build(ReportKind::Error, (source_id, span_range.clone()))
      .with_message(&error.message);

  report = report.with_label(
    Label::new((source_id, span_range))
      .with_message(&error.message)
      .with_color(Color::Red),
  );

  report
    .finish()
    .print((source_id, Source::from(source_content)))
    .expect("Failed to print error report");
}

fn report_parse_errors<'a>(
  source_id: &str,
  source_content: &str,
  errors: &[Rich<'a, char>],
) {
  for error in errors {
    let span_range = error.span().into_range();

    let mut report =
      Report::build(ReportKind::Error, (error.to_string(), span_range.clone()))
        .with_message(error.to_string());

    report = report.with_label(
      Label::new((source_id.to_owned(), span_range))
        .with_message(error.reason().to_string())
        .with_color(Color::Red),
    );

    for (label_text, span) in error.contexts() {
      report = report.with_label(
        Label::new((source_id.to_owned(), span.into_range()))
          .with_message(format!("while parsing this {}", label_text))
          .with_color(Color::Yellow),
      );
    }

    report
      .finish()
      .print((source_id.to_owned(), Source::from(source_content)))
      .expect("Failed to print error report");
  }
}

#[derive(Clap)]
#[clap(author, version)]
struct Arguments {
  filename: Option<PathBuf>,
}

fn main() {
  let arguments = Arguments::parse();

  if let Some(filename) = arguments.filename {
    match fs::read_to_string(&filename) {
      Ok(content) => {
        let filename_str = filename.to_string_lossy().to_string();

        let result = parser().parse(content.trim());

        match result.into_output_errors() {
          (Some(ast), errors) if errors.is_empty() => match eval(&ast) {
            Ok(value) => println!("{value}"),
            Err(error) => report_error(&filename_str, &content, &error),
          },
          (_, errors) => {
            report_parse_errors(&filename_str, &content, &errors);
          }
        }
      }
      Err(error) => {
        eprintln!("error: {error}");
        std::process::exit(1);
      }
    }
  } else {
    loop {
      let mut buffer = String::new();

      print!("> ");

      io::stdout().flush().unwrap();

      if io::stdin().lock().read_line(&mut buffer).unwrap() == 0 {
        break;
      }

      let input = buffer.trim();

      if input.is_empty() {
        continue;
      }

      let result = parser().parse(input);

      match result.into_output_errors() {
        (Some(ast), errors) if errors.is_empty() => match eval(&ast) {
          Ok(value) => println!("{}", value),
          Err(error) => report_error("<input>", input, &error),
        },
        (_, errors) => {
          report_parse_errors("<input>", input, &errors);
        }
      }
    }
  }
}
