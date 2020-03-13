use serde::Deserialize;

mod biomes;

#[serde(deny_unknown_fields)]
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct Biome {
    pub name: String,
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
#[derive(Deserialize, Debug, Default, Eq, PartialEq)]
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
#[derive(Deserialize, Debug, Eq, PartialEq)]
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

pub fn get_biomes() -> Result<Vec<Biome>, String> {
    get_biomes_from_str(*biomes::get_json())
}

fn get_biomes_from_str(json: &str) -> Result<Vec<Biome>, String> {
    serde_json::from_str(json).map_err(|err| format!("Failed to parse json: {}", err))
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
        .map(|path| {
            let (power_scrolls, dual_scrolls, fragments, cursed_chest_probabilities) =
                path.iter().fold(
                    (0, 0, 0, vec![]),
                    |(power_scrolls, dual_scrolls, fragments, mut cursed_chest_probabilites),
                     biome| {
                        cursed_chest_probabilites.push(biome.cursed_chest_chance);
                        (
                            power_scrolls + biome.power_scrolls,
                            dual_scrolls + biome.dual_power_scrolls,
                            fragments + biome.scroll_fragments.get_fragments(&boss_cells),
                            cursed_chest_probabilites,
                        )
                    },
                );
            // print!("Probabilties of {:?}", cursed_chest_probabilities);
            let scrolls_from_cursed_chests =
                calculate_scrolls_from_cursed_chests(cursed_chest_probabilities);
            // println!(" => avg scrolls: {}", scrolls_from_cursed_chests);
            //todo also add scrolls from transitions (like from Haven to Throne Room)
            if include_dual_scrolls {
                (
                    power_scrolls + dual_scrolls + fragments / 4 + scrolls_from_cursed_chests,
                    path,
                )
            } else {
                (
                    power_scrolls + fragments / 4 + scrolls_from_cursed_chests,
                    path,
                )
            }
        })
        .collect();

    paths_with_scrolls
        .sort_unstable_by(|(left_scrolls, _), (right_scrolls, _)| right_scrolls.cmp(&left_scrolls));

    paths_with_scrolls.swap_remove(0)
}

fn calculate_scrolls_from_cursed_chests(probabilities: Vec<u8>) -> u8 {
    // todo this is crap but I don't understand Poisson binomial distribution
    let sum: f64 = probabilities.into_iter().map(|p| p as f64).sum();
    (sum / 100.).round() as u8
}

#[cfg(test)]
mod tests {
    use crate::*;

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
    }

    #[test]
    fn biome_from_tuple() {
        let biome: Biome = ("name", vec!["destination1", "destination2"]).into();

        let expected = Biome {
            name: "name".to_string(),
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
        assert!(paths.is_ok());
        let paths = paths.unwrap();

        assert_eq!(185, paths.len());
    }

    #[test]
    fn should_find_path_with_most_scrolls() {
        let biomes = get_biomes().unwrap();
        let paths = find_paths(&biomes, "Prisoners' Quarters", "Throne Room");
        assert!(paths.is_ok());
        let paths = paths.unwrap();

        let (scrolls, path) = get_path_with_most_scrolls(&paths, BossCells::Five, false);

        assert_eq!(scrolls, 22, "Wrong amount of scrolls in best route");

        let path_string = path_to_names(&path);
        assert_eq!(
            path_string,
            vec![
                "Prisoners' Quarters",
                "Dilapidated Arboretum",
                "Prison Depths",
                "Ancient Sewers",
                "Insufferable Crypt",
                "Graveyard",
                "Cavern",
                "Guardian's Haven",
                "High Peak Castle",
                "Throne Room"
            ]
        );
    }

    fn path_to_names<'b>(path: &Vec<&'b Biome>) -> Vec<&'b String> {
        path.iter().map(|biome| &biome.name).collect()
    }
}
