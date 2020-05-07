pub(crate) fn get_json() -> Box<&'static str> {
    Box::new(include_str!("biomes.json"))
}
