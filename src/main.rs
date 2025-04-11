use {
  chumsky::prelude::*,
  clap::Parser as Clap,
  std::{
    fmt::{self, Display, Formatter},
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
  },
};

#[derive(Debug)]
enum UnaryOp {
  Neg,
}

impl Display for UnaryOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      UnaryOp::Neg => write!(f, "-"),
    }
  }
}

#[derive(Debug)]
enum BinaryOp {
  Add,
  Div,
  Mod,
  Mul,
  Sub,
}

impl Display for BinaryOp {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      BinaryOp::Add => write!(f, "+"),
      BinaryOp::Div => write!(f, "/"),
      BinaryOp::Mod => write!(f, "%"),
      BinaryOp::Mul => write!(f, "*"),
      BinaryOp::Sub => write!(f, "-"),
    }
  }
}

#[derive(Debug)]
enum Ast<'a> {
  BinaryOp(BinaryOp, Box<Ast<'a>>, Box<Ast<'a>>),
  Call(&'a str, Vec<Ast<'a>>),
  Identifier(&'a str),
  Number(f64),
  UnaryOp(UnaryOp, Box<Ast<'a>>),
}

impl Display for Ast<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Ast::BinaryOp(op, lhs, rhs) => write!(f, "({} {} {})", op, lhs, rhs),
      Ast::Call(name, args) => write!(
        f,
        "{}({})",
        name,
        args
          .iter()
          .map(|a| a.to_string())
          .collect::<Vec<_>>()
          .join(", ")
      ),
      Ast::Identifier(name) => write!(f, "{}", name),
      Ast::Number(n) => write!(f, "{}", n),
      Ast::UnaryOp(op, rhs) => write!(f, "({} {})", op, rhs),
    }
  }
}

fn parser<'a>() -> impl Parser<'a, &'a str, Ast<'a>> {
  let ident = text::ident().padded();

  let expr = recursive(|expr| {
    let number = text::int(10).map(|s: &str| Ast::Number(s.parse().unwrap()));

    let atom = number.or(ident.map(Ast::Identifier));

    let op = |c| just(c).padded();

    let unary = op('-')
      .repeated()
      .foldr(atom, |_, rhs| Ast::UnaryOp(UnaryOp::Neg, Box::new(rhs)));

    unary
  });

  expr
}

fn eval<'a>(ast: &'a Ast<'a>) -> Result<f64, String> {
  match ast {
    Ast::Number(n) => Ok(*n),
    Ast::UnaryOp(operator, rhs) => match operator {
      UnaryOp::Neg => Ok(-eval(rhs)?),
    },
    Ast::BinaryOp(operator, lhs, rhs) => match operator {
      BinaryOp::Add => Ok(eval(lhs)? + eval(rhs)?),
      BinaryOp::Div => Ok(eval(lhs)? / eval(rhs)?),
      BinaryOp::Mul => Ok(eval(lhs)? * eval(rhs)?),
      BinaryOp::Sub => Ok(eval(lhs)? - eval(rhs)?),
      _ => todo!(),
    },
    _ => todo!(),
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
    let content = fs::read_to_string(filename).unwrap();

    let ast = parser().parse(content.trim());

    let ast = match ast.into_result() {
      Ok(ast) => ast,
      Err(error) => {
        println!("error: {:?}", error);
        std::process::exit(1);
      }
    };

    println!("{}", eval(&ast).unwrap())
  } else {
    loop {
      let mut buffer = String::new();

      print!("> ");

      io::stdout().flush().unwrap();

      if io::stdin().lock().read_line(&mut buffer).unwrap() == 0 {
        break;
      }

      {
        let parser = parser();

        let input = buffer.trim();

        if input.is_empty() {
          continue;
        }

        match parser.parse(input).into_result() {
          Ok(ast) => {
            println!("{:?}", ast);
          }
          Err(error) => {
            println!("error: {:?}", error);
          }
        }
      }
    }
  }
}
