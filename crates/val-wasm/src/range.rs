use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct Range {
  pub start: u32,
  pub end: u32,
}
