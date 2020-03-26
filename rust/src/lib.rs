use crate::core::Biome;
use js_sys::Map;
use wasm_bindgen::prelude::*;
use web_sys::console;

mod biomes;
mod core;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_biomes() -> Result<Map, JsValue> {
    let (biomes, paths) = core::get_biomes_and_paths().map_err(|msg| JsValue::from(msg))?;

    println!("lib.get_biomes: biomes: {:?}", biomes);

    let map = Map::new();
    for (i, tier) in biomes.iter().enumerate() {
        if tier.is_empty() {
            continue;
        }
        map.set(
            &JsValue::from_f64(i as f64),
            &JsValue::from_serde(tier).expect("serialize biomes"),
        );
    }

    // ugliest of all hacks, todo figure out how to return both of biomes and paths in a nice way
    map.set(
        &JsValue::from("paths"),
        &JsValue::from_serde(&paths).expect("serialize biomes"),
    );

    Ok(map)
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn test_get_biomes() {
        let result = get_biomes();
    }
}
