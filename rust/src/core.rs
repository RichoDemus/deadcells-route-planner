use serde::{Deserialize, Serialize};

use crate::lazies;
use crate::biomes;
use crate::path;
use std::fmt;
use std::fmt::Debug;
use crate::path::Path;

pub fn get_biomes() -> Result<Vec<Biome>, String> {
    get_biomes_from_str(*biomes::get_json())
}

pub(crate) fn get_biomes_and_paths(
    blacklist: Vec<Id>,
    biomes: Option<Vec<Biome>>,
) -> Result<(Vec<Vec<Biome>>, Vec<Path>), String> {
    let biomes: Vec<Biome> = biomes
        .unwrap_or_else(|| {
            let b: &Vec<Biome> = &*lazies::BIOMES;
            b.clone()
        });

    let (paths, reachable_biomes) = path::get_paths(&blacklist);

    let biomes = filter_reachable_biomes(biomes, &reachable_biomes);
    let biomes = order_biomes_by_tier(biomes)?;

    Ok((biomes, paths))
}

// todo remove clone and use borrowed biomes everywhere
#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Biome {
    pub id: Id,
    pub name: String,
    pub row: usize,
    pub column: usize,
    pub power_scrolls: u8,
    pub dual_power_scrolls: u8,
    pub cursed_chest_chance: u8,
    pub scroll_fragments: ScrollFragments,
    pub gear_level: u8,
    pub exits: Vec<Exit>,
    #[serde(skip_deserializing)]
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Id {
    #[serde(rename = "prisonquart")]
    Prisonquart,
    #[serde(rename = "arboretum")]
    Arboretum,
    #[serde(rename = "promenade")]
    Promenade,
    #[serde(rename = "toxicsewers")]
    Toxicsewers,
    #[serde(rename = "prisondepths")]
    Prisondepths,
    #[serde(rename = "corruptedprison")]
    Corruptedprison,
    #[serde(rename = "morass")]
    Morass,
    #[serde(rename = "ossuary")]
    Ossuary,
    #[serde(rename = "ramparts")]
    Ramparts,
    #[serde(rename = "ancientsewers")]
    Ancientsewers,
    #[serde(rename = "nest")]
    Nest,
    #[serde(rename = "bridge")]
    Bridge,
    #[serde(rename = "crypt")]
    Crypt,
    #[serde(rename = "stilt")]
    Stilt,
    #[serde(rename = "slumbering")]
    Slumbering,
    #[serde(rename = "graveyard")]
    Graveyard,
    #[serde(rename = "clocktower")]
    Clocktower,
    #[serde(rename = "sepulcher")]
    Sepulcher,
    #[serde(rename = "cavern")]
    Cavern,
    #[serde(rename = "clockroom")]
    Clockroom,
    #[serde(rename = "haven")]
    Haven,
    #[serde(rename = "castle")]
    Castle,
    #[serde(rename = "throne")]
    Throne,
    #[serde(rename = "lab")]
    Lab,
    #[serde(rename = "observatory")]
    Observatory,
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[serde(deny_unknown_fields)]
#[derive(Serialize, Deserialize, Debug, Default, Eq, PartialEq, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Exit {
    pub destination: Id,
    pub boss_cell_requirement: Option<u8>,
    pub power_scrolls: Option<u8>,
}

impl From<Id> for Exit {
    fn from(destination: Id) -> Self {
        Exit {
            destination,
            boss_cell_requirement: None,
            power_scrolls: None,
        }
    }
}

pub(crate) fn get_biomes_from_str(json: &str) -> Result<Vec<Biome>, String> {
    serde_json::from_str(json).map_err(|err| format!("Failed to parse json: {}", err))
}

// todo this shouldn't return result, it can only error due to code error
fn order_biomes_by_tier(biomes: Vec<Biome>) -> Result<Vec<Vec<Biome>>, String> {
    // todo dont hardcore '14'
    let init: Vec<Vec<Biome>> = (0..14).map(|_| (vec![])).collect();

    let biomes: Vec<Vec<Biome>> = biomes
        .into_iter()
        .map(|biome| (biome.row, biome))
        .try_fold(init, |mut acc, (tier, biome)| {
            let biomes = acc.get_mut(tier - 1);
            match biomes {
                Some(b) => {
                    b.push(biome);
                    Ok(acc)
                }
                None => Err(format!("no row at {}", tier)),
            }
        })?;

    let biomes: Vec<Vec<Biome>> = biomes.into_iter().filter(|tier| !tier.is_empty()).collect();

    Ok(biomes)
}

// pub fn find_paths<'b>(
//     biomes: &'b Vec<Biome>,
//     start: &Id,
//     end: &Id,
// ) -> Result<Vec<Vec<&'b Biome>>, String> {
//     let start = biomes
//         .iter()
//         .find(|biome| &biome.id == start)
//         .ok_or(format!("Couldn't find start node {:?}", start))?;
//
//     let paths = find_path_rec(biomes, vec![&start], end);
//
//     Ok(paths)
// }
//
// fn find_path_rec<'b>(
//     all_biomes: &'b Vec<Biome>,
//     current_path: Vec<&'b Biome>,
//     end: &Id,
// ) -> Vec<Vec<&'b Biome>> {
//     let last_biome_in_path = current_path
//         .last()
//         .expect("There should be an element here");
//     if &last_biome_in_path.id == end {
//         return vec![current_path];
//     }
//
//     let exit_ids: Vec<&Id> = last_biome_in_path
//         .exits
//         .iter()
//         .map(|exit| &exit.destination)
//         .collect();
//     let next_biomes: Vec<&Biome> = all_biomes
//         .iter()
//         .filter(|biome| exit_ids.contains(&&biome.id))
//         .collect();
//
//     let mut paths = vec![];
//     for next_biome in next_biomes {
//         let mut next_path = current_path.clone();
//         next_path.push(next_biome);
//         let mut new_paths = find_path_rec(all_biomes, next_path, end);
//         paths.append(&mut new_paths)
//     }
//
//     paths
// }

pub fn get_path_with_most_scrolls<'b>(
    paths: &'b Vec<Vec<&'b Biome>>,
    boss_cells: BossCells,
    include_dual_scrolls: bool,
) -> (u8, &'b Vec<&'b Biome>) {
    let mut paths_with_scrolls: Vec<(u8, &Vec<&Biome>)> = paths
        .iter()
        .map(|path| {
            (
                calculate_scrolls(path, &boss_cells, include_dual_scrolls),
                path,
            )
        })
        .collect();

    paths_with_scrolls
        .sort_unstable_by(|(left_scrolls, _), (right_scrolls, _)| right_scrolls.cmp(&left_scrolls));

    paths_with_scrolls.swap_remove(0)
}

fn calculate_scrolls(path: &Vec<&Biome>, boss_cells: &BossCells, include_dual_scrolls: bool) -> u8 {
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

fn sum_collectibles_for_path(path: &Vec<&Biome>, boss_cells: &BossCells) -> (u8, u8, u8, u16) {
    path.iter().fold(
        (0, 0, 0, 0),
        |(power_scrolls, dual_scrolls, fragments, cursed_chest_probabilites), biome| {
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
    (power_scrolls, dual_scrolls, fragments, cursed_chest_probabilities): (u8, u8, u8, u16),
) -> (u8, u8, u8, u16) {
    let new_scrolls = fragments / 4;
    let leftover_fragments = fragments % 4;
    (
        power_scrolls + new_scrolls,
        dual_scrolls,
        leftover_fragments,
        cursed_chest_probabilities,
    )
}

fn calculate_collectibles_from_cursed_chests(
    (power_scrolls, dual_scrolls, fragments, cursed_chest_probabilities): (u8, u8, u8, u16),
) -> (u8, u8, u8, u16) {
    let new_scrolls = (cursed_chest_probabilities / 100) as u8;
    let leftover_chest_probability = cursed_chest_probabilities % 100;
    (
        power_scrolls + new_scrolls,
        dual_scrolls,
        fragments,
        leftover_chest_probability,
    )
}

// todo investigate and maybe do this in a  const fn :o
fn calculate_paths_old(biomes: &Vec<Biome>, blacklist: &Vec<Id>) -> Vec<Path> {
    let mut result = vec![];

    fn calc_columns(biomes: &Vec<Biome>, row: usize) -> usize {
        biomes.into_iter().filter(|biome| biome.row == row).count()
    }

    fn calc_length(start: &Biome, end: &Biome) -> u8 {
        (end.row - start.row) as u8
    }

    fn get_biome<'b>(biomes: &'b Vec<Biome>, id: &Id) -> &'b Biome {
        biomes
            .into_iter()
            .find(|biome| &biome.id == id)
            .expect(format!("No biome with id {:?}", id).as_str())
    }

    fn enabled(start_id: &Id, end_id: &Id, blacklist: &Vec<Id>) -> bool {
        return if blacklist.contains(&start_id) || blacklist.contains(&end_id) {
            false
        } else {
            true
        };
    }

    'biome: for biome in biomes {
        let start_id = biome.id.clone();
        let row = biome.row as u8;
        let start_column = biome.column as u8;
        let start_columns = calc_columns(biomes, biome.row) as u8;

        'exit: for exit in &biome.exits {
            let end_biome = get_biome(biomes, &exit.destination);
            let end_column = end_biome.column as u8;
            let end_columns = calc_columns(biomes, end_biome.row) as u8;
            let length = calc_length(biome, end_biome);

            // todo fix tolowercase hack
            result.push(Path {
                id: format!(
                    "{}-{}",
                    start_id.to_string().to_lowercase(),
                    exit.destination.to_string().to_lowercase()
                ),
                start_column,
                start_columns,
                end_column,
                end_columns,
                row,
                length,
                enabled: enabled(&start_id, &end_biome.id, blacklist),
            });
        }
    }

    result
}

fn filter_reachable_biomes(biomes: Vec<Biome>, whitelist: &Vec<Id>) -> Vec<Biome> {
    biomes
        .into_iter()
        .map(|mut biome| {
            biome.enabled = whitelist.contains(&biome.id);
            biome
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::Path;

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
    fn test_id_serde() {
        let json = serde_json::to_string(&Id::Castle).unwrap();
        assert_eq!(json, String::from("\"castle\""));

        let id: Id = serde_json::from_str("\"castle\"").unwrap();
        assert_eq!(id, Id::Castle);
    }

    #[test]
    fn parse_json() {
        let biomes = get_biomes().unwrap();
        assert_eq!(biomes.len(), 25);
    }

    #[test]
    fn biome_from_tuple() {
        let biome: Biome = (Id::Prisonquart, vec![Id::Castle, Id::Crypt]).into();

        let expected = Biome {
            id: Id::Prisonquart,
            name: Id::Prisonquart.to_string(),
            row: 0,
            column: 0,
            power_scrolls: 0,
            dual_power_scrolls: 0,
            cursed_chest_chance: 0,
            scroll_fragments: Default::default(),
            gear_level: 0,
            exits: vec![
                Exit {
                    destination: Id::Castle,
                    boss_cell_requirement: None,
                    power_scrolls: None,
                },
                Exit {
                    destination: Id::Crypt,
                    boss_cell_requirement: None,
                    power_scrolls: None,
                },
            ],
            enabled: true,
        };

        assert_eq!(biome, expected);
    }

    #[test]
    fn find_path_between_nodes() {
        let input: Vec<Biome> = vec![
            (Id::Prisonquart, vec![Id::Corruptedprison, Id::Bridge]).into(),
            (Id::Corruptedprison, vec![]).into(),
            (Id::Bridge, vec![Id::Throne, Id::Slumbering]).into(),
            (Id::Slumbering, vec![Id::Throne]).into(),
            (Id::Throne, vec![]).into(),
        ];

        let paths = path::find_paths(&input);
        assert!(paths.is_ok());
        let paths = paths.unwrap();

        let paths_string: Vec<Vec<&Id>> =
            paths.into_iter().map(|path| path_to_ids(&path)).collect();

        // todo maybe clone to remove &
        assert_eq!(
            paths_string,
            vec![
                vec![&Id::Prisonquart, &Id::Bridge, &Id::Slumbering, &Id::Throne],
                vec![&Id::Prisonquart, &Id::Bridge, &Id::Throne]
            ]
        );
    }

    #[test]
    fn parse_paths_for_actual_data() {
        let biomes = get_biomes().unwrap();
        let paths = path::find_paths(&biomes);
        // let paths = find_paths(&biomes, "Prisoners' Quarters", "Throne Room");
        assert!(paths.is_ok());
        let paths = paths.unwrap();

        // assert_eq!(185, paths.len());

        let mut result: Vec<((u8, u8, u8, u16), Vec<&String>)> = paths
            .into_iter()
            .map(|path| (sum_collectibles_for_path(&path, &BossCells::Five), path))
            .map(|(collectibles, path)| (calculate_collectibles_from_fragments(collectibles), path))
            .map(|(collectibles, path)| {
                (
                    calculate_collectibles_from_cursed_chests(collectibles),
                    path,
                )
            })
            .map(|(collectibles, path)| (collectibles, path_to_names(&path)))
            .collect();

        result.sort_by(
            |((left_scrolls, _, _, _), _), ((right_scrolls, _, _, _), _)| {
                left_scrolls.cmp(&right_scrolls)
            },
        );
        result.reverse();

        // for (collectibles, path) in result {
        //     println!("path: {:?} - {:?}", collectibles, path);
        // }
    }

    #[test]
    fn should_find_path_with_most_scrolls() {
        let biomes = get_biomes().unwrap();
        let paths = path::find_paths(&biomes);
        assert!(paths.is_ok());
        let paths = paths.unwrap();

        let (scrolls, _path) = get_path_with_most_scrolls(&paths, BossCells::Five, false);

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
        let input = (10, 0, 5, 0);
        let result = calculate_collectibles_from_fragments(input);

        assert_eq!(result, (11, 0, 1, 0))
    }

    #[test]
    fn test_calculate_collectibles_from_cursed_chests() {
        let input = (10, 0, 0, 190);
        let result = calculate_collectibles_from_cursed_chests(input);

        assert_eq!(result, (11, 0, 0, 90))
    }

    #[test]
    fn test_order_biomes_by_row() {
        let biomes = vec![
            (Id::Prisonquart, 1, 1).into(),
            (Id::Promenade, 2, 1).into(),
            (Id::Toxicsewers, 2, 2).into(),
        ];

        let result = order_biomes_by_tier(biomes);
        let result = result.unwrap();

        assert_eq!(
            result,
            vec![
                vec![(Id::Prisonquart, 1, 1).into()],
                vec![(Id::Promenade, 2, 1).into(), (Id::Toxicsewers, 2, 2).into()]
            ]
        );
    }

    // #[test]
    fn test_get_biomes_and_paths() {
        let input: Vec<Biome> = vec![
            (Id::Prisonquart, 1, 1, vec![Id::Arboretum]).into(),
            (Id::Arboretum, 2, 1, vec![Id::Prisondepths, Id::Morass]).into(),
            (Id::Promenade, 2, 2, vec![]).into(),
            (Id::Prisondepths, 3, 1, vec![Id::Morass]).into(),
            (Id::Morass, 4, 1, vec![]).into(),
        ];

        let (biomes, paths) = get_biomes_and_paths(vec![], Some(input)).unwrap();

        assert_eq!(
            biomes,
            vec![
                vec![(Id::Prisonquart, 1, 1, vec![Id::Arboretum]).into()],
                vec![
                    (Id::Arboretum, 2, 1, vec![Id::Prisondepths, Id::Morass]).into(),
                    (Id::Promenade, 2, 2, vec![]).into()
                ],
                vec![(Id::Prisondepths, 3, 1, vec![Id::Morass]).into()],
                vec![(Id::Morass, 4, 1, vec![]).into()],
            ]
        );

        assert_eq!(
            paths,
            vec![
                Path {
                    id: format!(
                        "{}-{}",
                        Id::Prisonquart.to_string().to_lowercase(),
                        Id::Arboretum.to_string().to_lowercase()
                    ),
                    start_column: 1,
                    start_columns: 1,
                    end_column: 1,
                    end_columns: 2,
                    row: 1,
                    length: 1,
                    enabled: true,
                },
                Path {
                    id: format!(
                        "{}-{}",
                        Id::Arboretum.to_string().to_lowercase(),
                        Id::Prisondepths.to_string().to_lowercase()
                    ),
                    start_column: 1,
                    start_columns: 2,
                    end_column: 1,
                    end_columns: 1,
                    row: 2,
                    length: 1,
                    enabled: true,
                },
                Path {
                    id: format!(
                        "{}-{}",
                        Id::Arboretum.to_string().to_lowercase(),
                        Id::Morass.to_string().to_lowercase()
                    ),
                    start_column: 1,
                    start_columns: 2,
                    end_column: 1,
                    end_columns: 1,
                    row: 2,
                    length: 2,
                    enabled: true,
                },
                Path {
                    id: format!(
                        "{}-{}",
                        Id::Prisondepths.to_string().to_lowercase(),
                        Id::Morass.to_string().to_lowercase()
                    ),
                    start_column: 1,
                    start_columns: 1,
                    end_column: 1,
                    end_columns: 1,
                    row: 3,
                    length: 1,
                    enabled: true,
                },
            ]
        );
    }

    // #[test]
    fn test_get_biomes_and_paths_one_blacklisted() {
        let input: Vec<Biome> = vec![
            (Id::Prisonquart, 1, 1, vec![Id::Arboretum]).into(),
            (Id::Arboretum, 2, 1, vec![Id::Prisondepths, Id::Morass]).into(),
            (Id::Promenade, 2, 2, vec![]).into(),
            (Id::Prisondepths, 3, 1, vec![Id::Morass]).into(),
            (Id::Morass, 4, 1, vec![]).into(),
        ];

        let (biomes, paths) = get_biomes_and_paths(vec![Id::Prisondepths], Some(input)).unwrap();

        assert_eq!(
            biomes,
            vec![
                vec![(Id::Prisonquart, 1, 1, vec![Id::Arboretum]).into()],
                vec![
                    (Id::Arboretum, 2, 1, vec![Id::Prisondepths, Id::Morass]).into(),
                    (Id::Promenade, 2, 2, vec![]).into()
                ],
                vec![(Id::Prisondepths, 3, 1, vec![Id::Morass], false).into()],
                vec![(Id::Morass, 4, 1, vec![]).into()],
            ]
        );

        assert_eq!(
            paths,
            vec![
                Path {
                    id: format!(
                        "{}-{}",
                        Id::Prisonquart.to_string().to_lowercase(),
                        Id::Arboretum.to_string().to_lowercase()
                    ),
                    start_column: 1,
                    start_columns: 1,
                    end_column: 1,
                    end_columns: 2,
                    row: 1,
                    length: 1,
                    enabled: true,
                },
                Path {
                    id: format!(
                        "{}-{}",
                        Id::Arboretum.to_string().to_lowercase(),
                        Id::Prisondepths.to_string().to_lowercase()
                    ),
                    start_column: 1,
                    start_columns: 2,
                    end_column: 1,
                    end_columns: 1,
                    row: 2,
                    length: 1,
                    enabled: false,
                },
                Path {
                    id: format!(
                        "{}-{}",
                        Id::Arboretum.to_string().to_lowercase(),
                        Id::Morass.to_string().to_lowercase()
                    ),
                    start_column: 1,
                    start_columns: 2,
                    end_column: 1,
                    end_columns: 1,
                    row: 2,
                    length: 2,
                    enabled: true,
                },
                Path {
                    id: format!(
                        "{}-{}",
                        Id::Prisondepths.to_string().to_lowercase(),
                        Id::Morass.to_string().to_lowercase()
                    ),
                    start_column: 1,
                    start_columns: 1,
                    end_column: 1,
                    end_columns: 1,
                    row: 3,
                    length: 1,
                    enabled: false,
                },
            ]
        );
    }

    // #[test]
    fn test_get_biomes_and_paths_disable_transitive_paths() {
        let input: Vec<Biome> = vec![
            (Id::Morass, 1, 1, vec![Id::Nest]).into(),
            (Id::Nest, 1, 1, vec![Id::Stilt]).into(),
            (Id::Stilt, 1, 1, vec![]).into(),
        ];

        let (biomes, paths) = get_biomes_and_paths(vec![Id::Nest], Some(input)).unwrap();

        assert_eq!(
            biomes,
            vec![
                vec![(Id::Morass, 1, 1, vec![Id::Nest], false).into()],
                vec![(Id::Nest, 1, 1, vec![Id::Stilt], false).into()],
                vec![(Id::Stilt, 3, 1, vec![], false).into()],
            ]
        );

        assert_eq!(
            paths,
            vec![
                Path {
                    id: format!(
                        "{}-{}",
                        Id::Morass.to_string().to_lowercase(),
                        Id::Nest.to_string().to_lowercase()
                    ),
                    start_column: 1,
                    start_columns: 1,
                    end_column: 1,
                    end_columns: 1,
                    row: 1,
                    length: 1,
                    enabled: false,
                },
                Path {
                    id: format!(
                        "{}-{}",
                        Id::Nest.to_string().to_lowercase(),
                        Id::Stilt.to_string().to_lowercase()
                    ),
                    start_column: 1,
                    start_columns: 1,
                    end_column: 1,
                    end_columns: 1,
                    row: 1,
                    length: 1,
                    enabled: false,
                },
            ]
        );
    }


    fn path_to_names<'b>(path: &Vec<&'b Biome>) -> Vec<&'b String> {
        path.iter().map(|biome| &biome.name).collect()
    }

    fn path_to_ids<'b>(path: &Vec<&'b Biome>) -> Vec<&'b Id> {
        path.iter().map(|biome| &biome.id).collect()
    }

    impl From<(Id, Vec<Id>)> for Biome {
        fn from((id, exits): (Id, Vec<Id>)) -> Self {
            let name = id.to_string();
            let exits = exits.into_iter().map(|exit| Exit::from(exit)).collect();
            Biome {
                id,
                name,
                row: 0,
                column: 0,
                power_scrolls: 0,
                dual_power_scrolls: 0,
                cursed_chest_chance: 0,
                scroll_fragments: ScrollFragments::default(),
                gear_level: 0,
                exits,
                enabled: true,
            }
        }
    }

    impl From<(Id, usize, usize)> for Biome {
        fn from((id, row, column): (Id, usize, usize)) -> Self {
            let name = id.to_string();
            Biome {
                id,
                name,
                row,
                column,
                power_scrolls: 0,
                dual_power_scrolls: 0,
                cursed_chest_chance: 0,
                scroll_fragments: ScrollFragments::default(),
                gear_level: 0,
                exits: vec![],
                enabled: true,
            }
        }
    }

    impl From<(Id, usize, usize, Vec<Id>, bool)> for Biome {
        fn from((id, row, column, exits, enabled): (Id, usize, usize, Vec<Id>, bool)) -> Self {
            let name = id.to_string();
            let exits = exits.into_iter().map(|exit| Exit::from(exit)).collect();
            Biome {
                id,
                name,
                row,
                column,
                power_scrolls: 0,
                dual_power_scrolls: 0,
                cursed_chest_chance: 0,
                scroll_fragments: ScrollFragments::default(),
                gear_level: 0,
                exits,
                enabled,
            }
        }
    }
}
