use super::*;

#[derive(Clone, Debug)]
pub struct Config {
  pub precision: usize,
  pub rounding_mode: RoundingMode,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      precision: 1024,
      rounding_mode: RoundingMode::ToEven,
    }
  }
}
