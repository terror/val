use super::*;

#[derive(Clone, Copy, Debug)]
pub struct Config {
  pub digits: usize,
  pub precision: u32,
  pub rounding_mode: Round,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      digits: 16,
      precision: 1024,
      rounding_mode: Round::Nearest,
    }
  }
}
