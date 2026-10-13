use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Tsify)]
pub struct Range {
  pub start: u32,
  pub end: u32,
}
