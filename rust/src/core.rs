use serde::{Deserialize, Serialize};

use crate::biomes;
use core::ops;
use wasm_bindgen::__rt::core::ops::Add;

pub(crate) fn get_biomes() -> Result<Vec<Biome>, String> {
    get_biomes_from_str(*biomes::get_json())
}

#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Biome {
    pub name: String,
    pub row: usize,
    pub column: usize,
    pub power_scrolls: u8,
    pub dual_power_scrolls: u8,
    pub cursed_chest_chance: u8,
    pub scroll_fragments: ScrollFragments,
    pub gear_level: u8,
    pub exits: Vec<Exit>,
}

impl From<(&str, Vec<&str>)> for Biome {
    fn from((name, exits): (&str, Vec<&str>)) -> Self {
        let exits = exits
            .into_iter()
            .map(|exit| Exit::from(exit.to_string()))
            .collect();
        Biome {
            name: name.to_string(),
            row: 0,
            column: 0,
            power_scrolls: 0,
            dual_power_scrolls: 0,
            cursed_chest_chance: 0,
            scroll_fragments: ScrollFragments::default(),
            gear_level: 0,
            exits,
        }
    }
}

#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Default, Eq, PartialEq)]
pub struct ScrollFragments {
    #[serde(alias = "1")]
    pub one: Option<u8>,
    #[serde(alias = "2")]
    pub two: Option<u8>,
    #[serde(alias = "3")]
    pub three: Option<u8>,
    #[serde(alias = "4")]
    pub four: Option<u8>,
    #[serde(alias = "5")]
    pub five: Option<u8>,
}

impl ScrollFragments {
    fn get_fragments(&self, boss_cells: &BossCells) -> u8 {
        let one = self.one;
        let two = self.two.or(one);
        let three = self.three.or(two);
        let four = self.four.or(three);
        let five = self.five.or(four);

        match boss_cells {
            BossCells::One => one.unwrap_or(0),
            BossCells::Two => two.unwrap_or(0),
            BossCells::Three => three.unwrap_or(0),
            BossCells::Four => four.unwrap_or(0),
            BossCells::Five => five.unwrap_or(0),
        }
    }
}

pub enum BossCells {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Exit {
    pub destination: String,
    pub boss_cell_requirement: Option<u8>,
    pub power_scrolls: Option<u8>,
}

impl From<String> for Exit {
    fn from(destination: String) -> Self {
        Exit {
            destination,
            boss_cell_requirement: None,
            power_scrolls: None,
        }
    }
}

fn get_biomes_from_str(json: &str) -> Result<Vec<Biome>, String> {
    serde_json::from_str(json).map_err(|err| format!("Failed to parse json: {}", err))
}

fn take_string_return_string(s:&str) -> Box<&str> {
    Box::new(s)
}

fn take_int_return_int(i:Biome) -> Biome {
    i + 1
}

impl ops::Add<i32> for Biome {
    type Output = Biome;

    fn add(self, rhs: i32) -> Self::Output {
        unimplemented!()
    }
}

pub fn find_paths<'b>(
    biomes: &'b Vec<Biome>,
    start: &str,
    end: &str,
) -> Result<Vec<Vec<&'b Biome>>, String> {
    let start = biomes
        .iter()
        .find(|biome| biome.name == start)
        .ok_or(format!("Couldn't find start node {}", start))?;

    let paths = find_path_rec(biomes, vec![&start], end);

    Ok(paths)
}

fn find_path_rec<'b>(
    all_biomes: &'b Vec<Biome>,
    current_path: Vec<&'b Biome>,
    end: &str,
) -> Vec<Vec<&'b Biome>> {
    let last_biome_in_path = current_path
        .last()
        .expect("There should be an element here");
    if last_biome_in_path.name == end {
        return vec![current_path];
    }

    let exit_names: Vec<&String> = last_biome_in_path
        .exits
        .iter()
        .map(|exit| &exit.destination)
        .collect();
    let next_biomes: Vec<&Biome> = all_biomes
        .iter()
        .filter(|biome| exit_names.contains(&&biome.name))
        .collect();

    let mut paths = vec![];
    for next_biome in next_biomes {
        let mut next_path = current_path.clone();
        next_path.push(next_biome);
        let mut new_paths = find_path_rec(all_biomes, next_path, end);
        paths.append(&mut new_paths)
    }

    paths
}

pub fn get_path_with_most_scrolls<'b>(
    paths: &'b Vec<Vec<&'b Biome>>,
    boss_cells: BossCells,
    include_dual_scrolls: bool,
) -> (u8, &'b Vec<&'b Biome>) {
    let mut paths_with_scrolls: Vec<(u8, &Vec<&Biome>)> = paths
        .iter()
        .map(|path| (calculate_scrolls(path, &boss_cells, include_dual_scrolls), path))
        .collect();

    paths_with_scrolls
        .sort_unstable_by(|(left_scrolls, _), (right_scrolls, _)| right_scrolls.cmp(&left_scrolls));

    paths_with_scrolls.swap_remove(0)
}

fn calculate_scrolls(
    path: &Vec<&Biome> ,
    boss_cells: &BossCells,
                     include_dual_scrolls: bool,
) -> u8 {
    let (power_scrolls, dual_scrolls, fragments, cursed_chest_probabilities) =
        sum_collectibles_for_path(path, boss_cells);

    let scrolls_from_cursed_chests =
        calculate_scrolls_from_cursed_chests(cursed_chest_probabilities);

    //todo also add scrolls from transitions (like from Haven to Throne Room)
    if include_dual_scrolls {
            power_scrolls + dual_scrolls + fragments / 4 + scrolls_from_cursed_chests
    } else {
            power_scrolls + fragments / 4 + scrolls_from_cursed_chests
    }
}

fn sum_collectibles_for_path(
    path: &Vec<&Biome> ,
    boss_cells: &BossCells,
) -> (u8, u8, u8, u16) {
        path.iter().fold(
            (0, 0, 0, 0),
            |(power_scrolls, dual_scrolls, fragments, cursed_chest_probabilites),
             biome| {
                (
                    power_scrolls + biome.power_scrolls,
                    dual_scrolls + biome.dual_power_scrolls,
                    fragments + biome.scroll_fragments.get_fragments(&boss_cells),
                    cursed_chest_probabilites + biome.cursed_chest_chance as u16,
                )
            },
        )
}

fn calculate_scrolls_from_cursed_chests(probability: u16) -> u8 {
    (probability as f64 / 100.).round() as u8
}

fn calculate_collectibles_from_fragments(
    (power_scrolls, dual_scrolls, fragments, cursed_chest_probabilities): (u8, u8, u8, u16)
) -> (u8, u8, u8, u16) {
    let new_scrolls = fragments/4;
    let leftover_fragments = fragments%4;
    (power_scrolls + new_scrolls, dual_scrolls, leftover_fragments, cursed_chest_probabilities)
}

fn calculate_collectibles_from_cursed_chests(
    (power_scrolls, dual_scrolls, fragments, cursed_chest_probabilities): (u8, u8, u8, u16)
) -> (u8, u8, u8, u16) {
    let new_scrolls = (cursed_chest_probabilities/100) as u8;
    let leftover_chest_probability = cursed_chest_probabilities % 100;
    (power_scrolls + new_scrolls, dual_scrolls, fragments, leftover_chest_probability)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_gracefully_fail() {
        let fail = get_biomes_from_str("qweqeqqwe{{{{");
        assert!(fail.is_err());
        if let Err(str) = fail {
            assert_eq!(
                str,
                "Failed to parse json: expected value at line 1 column 1"
            );
        }
    }

    #[test]
    fn parse_json() {
        let biomes = get_biomes().unwrap();
        assert_eq!(biomes.len(), 25);
        println!("{:?}", biomes)
    }

    #[test]
    fn biome_from_tuple() {
        let biome: Biome = ("name", vec!["destination1", "destination2"]).into();

        let expected = Biome {
            name: "name".to_string(),
            row: 0,
            column: 0,
            power_scrolls: 0,
            dual_power_scrolls: 0,
            cursed_chest_chance: 0,
            scroll_fragments: Default::default(),
            gear_level: 0,
            exits: vec![
                Exit {
                    destination: "destination1".to_string(),
                    boss_cell_requirement: None,
                    power_scrolls: None,
                },
                Exit {
                    destination: "destination2".to_string(),
                    boss_cell_requirement: None,
                    power_scrolls: None,
                },
            ],
        };

        assert_eq!(biome, expected);
    }

    #[test]
    fn find_path_between_nodes() {
        let input: Vec<Biome> = vec![
            ("start", vec!["dead_end", "middle"]).into(),
            ("dead_end", vec![]).into(),
            ("middle", vec!["end", "extra_biome"]).into(),
            ("extra_biome", vec!["end"]).into(),
            ("end", vec![]).into(),
        ];

        let paths = find_paths(&input, "start", "end");
        assert!(paths.is_ok());
        let paths = paths.unwrap();

        let paths_string: Vec<Vec<&String>> =
            paths.into_iter().map(|path| path_to_names(&path)).collect();

        assert_eq!(
            paths_string,
            vec![
                vec!["start", "middle", "extra_biome", "end"],
                vec!["start", "middle", "end"]
            ]
        );
    }

    #[test]
    fn parse_paths_for_actual_data() {
        let biomes = get_biomes().unwrap();
        let paths = find_paths(&biomes, "Prisoners' Quarters", "Throne Room");
        // let paths = find_paths(&biomes, "Prisoners' Quarters", "Throne Room");
        assert!(paths.is_ok());
        let paths = paths.unwrap();

        // assert_eq!(185, paths.len());

        let mut result: Vec<((u8,u8,u8,u16), Vec<&String>)> = paths.into_iter()
            .map(|path|(sum_collectibles_for_path(&path, &BossCells::Five), path))
            .map(|(collectibles, path)|(calculate_collectibles_from_fragments(collectibles), path))
            .map(|(collectibles, path)|(calculate_collectibles_from_cursed_chests(collectibles), path))
            .map(|(collectibles, path)|(collectibles, path_to_names(&path)))
            .collect();

        result.sort_by(|((left_scrolls,_,_,_),_),((right_scrolls,_,_,_),_)|left_scrolls.cmp(&right_scrolls));
        result.reverse();

        for (collectibles, path) in result {
            println!("path: {:?} - {:?}", collectibles, path);
        }
    }

    #[test]
    fn should_find_path_with_most_scrolls() {
        let biomes = get_biomes().unwrap();
        let paths = find_paths(&biomes, "Prisoners' Quarters", "Throne Room");
        assert!(paths.is_ok());
        let paths = paths.unwrap();

        let (scrolls, path) = get_path_with_most_scrolls(&paths, BossCells::Five, false);

        assert_eq!(scrolls, 22, "Wrong amount of scrolls in best route");

        // let path_string = path_to_names(&path);
        // assert_eq!(
        //     path_string,
        //     vec![
        //         "Prisoners' Quarters",
        //         "Toxic Sewers",
        //         "Corrupted Prison",
        //         "Ancient Sewers",
        //         "Insufferable Crypt",
        //         "Graveyard",
        //         "Cavern",
        //         "Guardian's Haven",
        //         "High Peak Castle",
        //         "Throne Room"
        //     ]
        // );
    }

    #[test]
    fn test_update_collectibes_with_scrolls_from_scroll_fragments() {
        let input = (10,0,5,0);
        let result = calculate_collectibles_from_fragments(input);

        assert_eq!(result, (11,0,1,0))
    }

    #[test]
    fn test_calculate_collectibles_from_cursed_chests(){
        let input = (10,0,0,190);
        let result = calculate_collectibles_from_cursed_chests(input);

        assert_eq!(result, (11,0,0,90))
}

    fn path_to_names<'b>(path: &Vec<&'b Biome>) -> Vec<&'b String> {
        path.iter().map(|biome| &biome.name).collect()
    }
}
