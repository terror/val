use super::*;

#[derive(Debug, Default)]
pub(crate) struct Frame {
  pub(crate) parent: Option<Environment>,
  pub(crate) symbols: HashMap<String, Symbol>,
}
