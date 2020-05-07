use crate::core;
use crate::json::json;
use crate::json::models::Biome;
use crate::path;
use crate::path::RenderablePath;
use crate::path::ToggleablePath;

lazy_static! {
    pub static ref BIOMES: Vec<Biome> = core::get_biomes_from_str(*json::get_json()).unwrap();
    pub static ref RAW_PATHS: Vec<ToggleablePath<'static>> = {
        let biomes: &Vec<Biome> = &*BIOMES;
        path::find_paths(biomes).unwrap()
    };
}
