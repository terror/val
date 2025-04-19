use super::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum RoundingMode {
  None = 1,
  Up = 2,
  Down = 4,
  ToZero = 8,
  FromZero = 16,
  ToEven = 32,
  ToOdd = 64,
}

impl std::fmt::Display for RoundingMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      RoundingMode::None => "none",
      RoundingMode::Up => "up",
      RoundingMode::Down => "down",
      RoundingMode::ToZero => "to-zero",
      RoundingMode::FromZero => "from-zero",
      RoundingMode::ToEven => "to-even",
      RoundingMode::ToOdd => "to-odd",
    };
    write!(f, "{}", s)
  }
}

impl std::str::FromStr for RoundingMode {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "none" => Ok(RoundingMode::None),
      "up" => Ok(RoundingMode::Up),
      "down" => Ok(RoundingMode::Down),
      "tozero" | "to_zero" | "to-zero" | "toward_zero" | "toward-zero" => {
        Ok(RoundingMode::ToZero)
      }
      "fromzero" | "from_zero" | "from-zero" | "away_from_zero"
      | "away-from-zero" => Ok(RoundingMode::FromZero),
      "toeven" | "to_even" | "to-even" | "nearest_even" | "bankers" => {
        Ok(RoundingMode::ToEven)
      }
      "toodd" | "to_odd" | "to-odd" | "nearest_odd" => Ok(RoundingMode::ToOdd),
      _ => Err(format!("Unknown rounding mode: {}", s)),
    }
  }
}

impl From<RoundingMode> for Round {
  fn from(mode: RoundingMode) -> Self {
    match mode {
      RoundingMode::None => Round::Nearest,
      RoundingMode::Up => Round::Up,
      RoundingMode::Down => Round::Down,
      RoundingMode::ToZero => Round::Zero,
      RoundingMode::FromZero => Round::AwayZero,
      RoundingMode::ToEven => Round::Nearest,
      RoundingMode::ToOdd => Round::Nearest,
    }
  }
}
