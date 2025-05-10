use super::*;

#[derive(Clone, Debug)]
pub struct Config {
  pub precision: u32,
  pub rounding_mode: Round,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      precision: 1024,
      rounding_mode: Round::Nearest,
    }
  }
}
