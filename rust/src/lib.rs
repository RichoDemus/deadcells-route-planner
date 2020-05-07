#[macro_use]
extern crate lazy_static;

use crate::json::models::*;
use js_sys::{Array, Map};
use wasm_bindgen::prelude::*;
use web_sys::console;

mod core;
mod json;
mod lazies;
mod path;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = getBiomes)]
pub fn get_biomes(blacklist: Array, boss_cells: JsValue) -> Result<Map, JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let blacklist: Vec<Id> = blacklist
        .to_vec()
        .into_iter()
        .map(|element| {
            JsValue::into_serde::<Id>(&element).expect("failed to deserialize blacklist")
        })
        .collect();

    let boss_cells: u8 = boss_cells.as_f64().expect("can't unwrap boss cells") as u8;
    console::log_1(&JsValue::from(format!("lib:: boss cells: {}", boss_cells)));

    let (biomes, paths) = core::get_biomes_and_paths(blacklist, boss_cells, None)
        .map_err(|msg| JsValue::from(msg))?;

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

pub fn log(msg: &str) {
    console::log_1(&JsValue::from(msg));
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn test_get_biomes() {
        let result = get_biomes(Array::new());
    }
}
