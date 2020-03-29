use crate::core::Biome;
use js_sys::{Array, Map};
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
    let biomes = core::get_biomes().map_err(|msg| JsValue::from(msg))?;

    // todo dont hardcore '14'
    let init: Vec<Vec<Biome>> = (1..14).map(|i| (vec![])).collect();

    let biomes =
        biomes
            .into_iter()
            .map(|biome| (biome.row, biome))
            .fold(init, |mut acc, (tier, biome)| {
                let mut biomes = acc.get_mut(tier);
                let mut biomes = match biomes {
                    Some(b) => b,
                    None => {
                        console::log_1(&JsValue::from(format!("no row at {}", tier)));
                        panic!()
                    }
                };
                biomes.push(biome);
                acc
            });

    println!("biomes: {:?}", biomes);

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
