use {super::*, rug::float::Round};

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum RoundingMode {
  Down,
  FromZero,
  ToEven,
  ToZero,
  Up,
}

impl Display for RoundingMode {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let s = match self {
      RoundingMode::Down => "down",
      RoundingMode::FromZero => "from-zero",
      RoundingMode::ToEven => "to-even",
      RoundingMode::ToZero => "to-zero",
      RoundingMode::Up => "up",
    };
    write!(f, "{s}")
  }
}

impl FromStr for RoundingMode {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "down" => Ok(RoundingMode::Down),
      "fromzero" | "from_zero" | "from-zero" | "away_from_zero"
      | "away-from-zero" => Ok(RoundingMode::FromZero),
      "toeven" | "to_even" | "to-even" | "nearest_even" | "bankers" => {
        Ok(RoundingMode::ToEven)
      }
      "tozero" | "to_zero" | "to-zero" | "toward_zero" | "toward-zero" => {
        Ok(RoundingMode::ToZero)
      }
      "up" => Ok(RoundingMode::Up),
      _ => Err(format!("Unknown rounding mode: {s}")),
    }
  }
}

impl From<RoundingMode> for Round {
  fn from(mode: RoundingMode) -> Self {
    match mode {
      RoundingMode::Down => Round::Down,
      RoundingMode::FromZero => Round::AwayZero,
      RoundingMode::ToEven => Round::Nearest,
      RoundingMode::ToZero => Round::Zero,
      RoundingMode::Up => Round::Up,
    }
  }
}
