use super::*;

#[derive(Debug, Default)]
pub(crate) struct Frame {
  pub(crate) parent: Option<Environment>,
  pub(crate) symbols: HashMap<String, Value>,
}

impl Finalize for Frame {}

unsafe impl Trace for Frame {
  gc::custom_trace!(this, {
    let Self { parent, symbols } = this;
    unsafe {
      mark(parent);
      mark(symbols);
    }
  });
}
