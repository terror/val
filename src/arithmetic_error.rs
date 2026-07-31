use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArithmeticError {
  DivisionByZero,
  ModuloByZero,
  ZeroToNegativePower,
}

impl Display for ArithmeticError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(match self {
      Self::DivisionByZero => "Division by zero",
      Self::ModuloByZero => "Modulo by zero",
      Self::ZeroToNegativePower => "Zero cannot be raised to a negative power",
    })
  }
}

impl std::error::Error for ArithmeticError {}
