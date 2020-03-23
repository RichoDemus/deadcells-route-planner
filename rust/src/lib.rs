use wasm_bindgen::prelude::*;
use js_sys::{Array, Map};
use crate::core::Biome;

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
pub fn get_biomes() -> Result<Map, JsValue> {
    let biomes = core::get_biomes()
        .map_err(|msg|JsValue::from(msg))?;


    let init: Vec<Vec<Biome>> = (0..=11).map(|i|(vec![])).collect();

    let biomes = biomes.into_iter()
        .map(|biome|(biome.tier, biome))
        .fold(init, |mut acc, (tier, biome)|{
            let mut biomes = acc.get_mut(tier
            ).expect(format!("No such tier: {}", tier).as_str());
            biomes.push(biome);
            acc
        });

    println!("biomes: {:?}", biomes);

    let map = Map::new();
    for (i,tier) in biomes.iter().enumerate() {
        map.set(&JsValue::from_f64(i as f64), &JsValue::from_serde(tier).expect("serialize biomes"));
    }

    Ok(map)


    // core::get_biomes()
    //     .map_err(|msg|msg.into())
    //     .map(|biomes|JsValue::from_serde(&biomes).expect("serialize biomes"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_biomes() {
        let result = get_biomes();
    }
}