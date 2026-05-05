use super::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum RoundingMode {
  Down = 4,
  FromZero = 16,
  None = 1,
  ToEven = 32,
  ToOdd = 64,
  ToZero = 8,
  Up = 2,
}

impl std::fmt::Display for RoundingMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      RoundingMode::Down => "down",
      RoundingMode::FromZero => "from-zero",
      RoundingMode::None => "none",
      RoundingMode::ToEven => "to-even",
      RoundingMode::ToOdd => "to-odd",
      RoundingMode::ToZero => "to-zero",
      RoundingMode::Up => "up",
    };
    write!(f, "{s}")
  }
}

impl std::str::FromStr for RoundingMode {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "down" => Ok(RoundingMode::Down),
      "fromzero" | "from_zero" | "from-zero" | "away_from_zero"
      | "away-from-zero" => Ok(RoundingMode::FromZero),
      "none" => Ok(RoundingMode::None),
      "toeven" | "to_even" | "to-even" | "nearest_even" | "bankers" => {
        Ok(RoundingMode::ToEven)
      }
      "toodd" | "to_odd" | "to-odd" | "nearest_odd" => Ok(RoundingMode::ToOdd),
      "tozero" | "to_zero" | "to-zero" | "toward_zero" | "toward-zero" => {
        Ok(RoundingMode::ToZero)
      }
      "up" => Ok(RoundingMode::Up),
      _ => Err(format!("Unknown rounding mode: {s}")),
    }
  }
}

impl From<RoundingMode> for astro_float::RoundingMode {
  fn from(mode: RoundingMode) -> Self {
    match mode {
      RoundingMode::Down => astro_float::RoundingMode::Down,
      RoundingMode::FromZero => astro_float::RoundingMode::FromZero,
      RoundingMode::None => astro_float::RoundingMode::None,
      RoundingMode::ToEven => astro_float::RoundingMode::ToEven,
      RoundingMode::ToOdd => astro_float::RoundingMode::ToOdd,
      RoundingMode::ToZero => astro_float::RoundingMode::ToZero,
      RoundingMode::Up => astro_float::RoundingMode::Up,
    }
  }
}
