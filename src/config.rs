#[derive(Clone, Copy, Debug)]
pub struct Config {
  pub precision: u32,
  pub rounding_mode: rug::float::Round,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      precision: 1024,
      rounding_mode: rug::float::Round::Nearest,
    }
  }
}
