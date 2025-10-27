#[derive(Clone, Debug)]
pub struct Config {
  pub precision: usize,
  pub rounding_mode: astro_float::RoundingMode,
  pub digits: Option<usize>,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      precision: 1024,
      rounding_mode: astro_float::RoundingMode::ToEven,
      digits: None,
    }
  }
}
