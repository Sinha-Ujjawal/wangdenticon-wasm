use wasm_bindgen::prelude::*;

mod wangdenticon;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn generate(name: &str, gridsize: u8, invert: bool, size: usize) -> String {
    wangdenticon::generate(name, gridsize, invert, size)
}
