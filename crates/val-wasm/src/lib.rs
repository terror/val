use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
  console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}
