use super::*;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum RoundingMode {
  Down,
  FromZero,
  ToEven,
  ToZero,
  Up,
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
