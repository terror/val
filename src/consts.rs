use super::*;

thread_local! {
  static CONSTS: RefCell<Consts> = RefCell::new(Consts::new().expect("failed to initialize astro_float constants"));
}

pub(crate) fn with_consts<T>(f: impl FnOnce(&mut Consts) -> T) -> T {
  CONSTS.with(|cell| {
    let mut consts = cell.borrow_mut();
    f(&mut consts)
  })
}
