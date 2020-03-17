use wasm_bindgen::prelude::*;
use js_sys::Array;

mod core;
mod biomes;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_biomes() -> Result<JsValue, JsValue> {
    core::get_biomes()
        .map_err(|msg|msg.into())
        .map(|biomes|JsValue::from_serde(&biomes).expect("serialize biomes"))
}
