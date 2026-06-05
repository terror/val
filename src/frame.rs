use super::*;

#[derive(Debug, Default)]
pub(crate) struct Frame<'src> {
  pub(crate) parent: Option<Environment<'src>>,
  pub(crate) symbols: HashMap<&'src str, Symbol<'src>>,
}
