#![cfg_attr(target_arch = "wasm32", allow(clippy::missing_safety_doc))]

#[cfg(target_arch = "wasm32")]
const ALIGN: usize = 16;

#[cfg(target_arch = "wasm32")]
const HEADER: usize = ALIGN;

pub const fn link() {}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn abort() -> ! {
  std::process::abort()
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut u8) {
  if ptr.is_null() {
    return;
  }

  let ptr = unsafe { ptr.sub(HEADER) };

  let size = unsafe { ptr.cast::<usize>().read_unaligned() };

  let Ok(layout) = std::alloc::Layout::from_size_align(size, ALIGN) else {
    abort();
  };

  unsafe {
    std::alloc::dealloc(ptr, layout);
  }
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
  let Some(size) = size.checked_add(HEADER) else {
    return std::ptr::null_mut();
  };

  let Ok(layout) = std::alloc::Layout::from_size_align(size, ALIGN) else {
    return std::ptr::null_mut();
  };

  let ptr = unsafe { std::alloc::alloc(layout) };

  if ptr.is_null() {
    return std::ptr::null_mut();
  }

  unsafe {
    ptr.cast::<usize>().write_unaligned(size);
    ptr.add(HEADER)
  }
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
  if ptr.is_null() {
    return malloc(size);
  }

  let Some(size) = size.checked_add(HEADER) else {
    return std::ptr::null_mut();
  };

  let base = unsafe { ptr.sub(HEADER) };

  let old_size = unsafe { base.cast::<usize>().read_unaligned() };

  let Ok(old_layout) = std::alloc::Layout::from_size_align(old_size, ALIGN)
  else {
    abort();
  };

  let ptr = unsafe { std::alloc::realloc(base, old_layout, size) };

  if ptr.is_null() {
    return std::ptr::null_mut();
  }

  unsafe {
    ptr.cast::<usize>().write_unaligned(size);
    ptr.add(HEADER)
  }
}
