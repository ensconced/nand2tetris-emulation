mod utils;

use std::convert::TryInto;

use emulator_core::computer::Computer;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn make_computer(rom: &[i16]) -> Computer {
    Computer::new(rom.try_into().expect("failed to convert slice into rom array"))
}
