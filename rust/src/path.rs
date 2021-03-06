use crate::json::models::*;
use crate::lazies;
use serde::Serialize;
use wasm_bindgen::__rt::std::process::exit;

pub(crate) fn get_paths(blacklist: &Vec<Id>, boss_cells: u8) -> (Vec<RenderablePath>, Vec<Id>) {
    get_paths_from(&*lazies::BIOMES, &*lazies::RAW_PATHS, blacklist, boss_cells)
}

fn get_paths_from(
    all_biomes: &Vec<Biome>,
    paths: &Vec<ToggleablePath>,
    blacklist: &Vec<Id>,
    boss_cells: u8,
) -> (Vec<RenderablePath>, Vec<Id>) {
    let result = apply_blacklist_and_boss_cells(paths, blacklist, boss_cells);
    biomes_paths_to_paths(all_biomes, result)
}

// todo investigate and maybe do this in a  const fn :o
fn calculate_paths(biomes: &Vec<Biome>, blacklist: &Vec<Id>) -> Vec<RenderablePath> {
    let mut result = vec![];

    let mut biomes_to_process = vec![biomes.first().expect("calc_paths biomes is empty")];
    let mut processed_biomes: Vec<&Id> = vec![];

    while !biomes_to_process.is_empty() {
        let biome = biomes_to_process.remove(0);
        println!(
            "Processing {:?}, to_process: {:?} processed: {:?}",
            biome.id, biomes_to_process, processed_biomes
        );
        let start_id = biome.id.clone();
        let row = biome.row as u8;
        let start_column = biome.column as u8;
        todo!();
        // let start_columns = calc_columns(biomes, biome.row) as u8;

        'exit: for exit in &biome.exits {
            let end_biome = get_biome(biomes, &exit.destination);
            if blacklist.contains(&end_biome.id) {
                continue 'exit;
            }
            let end_column = end_biome.column as u8;
            // let end_columns = calc_columns(biomes, end_biome.row) as u8;
            let length = calc_length(biome, end_biome);

            // todo fix tolowercase hack
            // result.push(Path {
            //     id: format!(
            //         "{}-{}",
            //         start_id.to_string().to_lowercase(),
            //         exit.destination.to_string().to_lowercase()
            //     ),
            //     start_column,
            //     start_columns,
            //     end_column,
            //     end_columns,
            //     row,
            //     length,
            //     enabled: enabled(&start_id, &end_biome.id, blacklist),
            // });
            if !processed_biomes.contains(&&end_biome.id) {
                biomes_to_process.push(end_biome);
            }
        }
        processed_biomes.push(&biome.id);
    }

    result
}

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

#[derive(Serialize, Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub struct RenderablePath {
    pub id: String,
    #[serde(rename = "startColumn")]
    pub start_column: u8,
    #[serde(rename = "startColumnOffset")]
    pub start_column_offset: u8,
    #[serde(rename = "startColumns")]
    pub start_columns: u8,
    #[serde(rename = "endColumn")]
    pub end_column: u8,
    #[serde(rename = "endtColumnOffset")]
    pub end_column_offset: u8,
    #[serde(rename = "endColumns")]
    pub end_columns: u8,
    pub row: u8,
    pub length: u8,
    pub enabled: bool,
}

#[derive(Clone, Debug)]
pub struct ToggleablePath<'b> {
    pub enabled: bool,
    pub path: Vec<&'b Biome>,
    pub minimum_boss_cells: u8,
}

impl ToggleablePath<'_> {
    fn update_minimum_boss_cells(&mut self, minimum_required_boss_cells: u8) {
        self.minimum_boss_cells =
            std::cmp::max(self.minimum_boss_cells, minimum_required_boss_cells);
    }
}

pub(crate) fn get_all_paths() {}

fn biomes_paths_to_paths<'b>(
    all_biomes: &Vec<Biome>,
    biomes: Vec<ToggleablePath>,
) -> (Vec<RenderablePath>, Vec<Id>) {
    let mut result = vec![];
    let mut reachable_biomes: Vec<Id> = all_biomes.first().iter().map(|b| b.id.clone()).collect();

    for toggleable_path in biomes {
        let ToggleablePath {
            enabled,
            path,
            minimum_boss_cells: _minimum_boss_cells,
        } = toggleable_path;
        'inner: for (i, start_biome) in path.iter().enumerate() {
            let end_biome = match path.get(i + 1) {
                Some(b) => b,
                None => break 'inner,
            };

            let start_id = start_biome.id.clone();
            let row = start_biome.row as u8;
            let start_column = start_biome.column as u8;
            let start_columns = calc_columns(all_biomes, start_biome.row) as u8;

            let end_column = end_biome.column as u8;
            let end_columns = calc_columns(all_biomes, end_biome.row) as u8;
            let length = calc_length(start_biome, end_biome);

            let new_path = RenderablePath {
                id: format!(
                    "{}-{}",
                    start_id.to_string().to_lowercase(),
                    end_biome.id.to_string().to_lowercase()
                ),
                start_column,
                start_column_offset: 0,
                start_columns,
                end_column,
                end_column_offset: 0,
                end_columns,
                row,
                length,
                enabled,
            };
            // contains check
            let existing_path: Option<(usize, &RenderablePath)> = result
                .iter()
                .enumerate()
                .find(|(_, path): &(usize, &RenderablePath)| path.id == new_path.id);
            match existing_path {
                Some((index, path)) => {
                    // if our new path is enabled, make sure the existing one is
                    if new_path.enabled {
                        match result.get_mut(index) {
                            Some(path_to_modify) => {
                                path_to_modify.enabled = true;
                            }
                            None => panic!("array elem dissapreared :o"),
                        }
                        // also flag this biome as reachable
                        if !reachable_biomes.contains(&end_biome.id) {
                            reachable_biomes.push(end_biome.id.clone())
                        }
                    }
                }
                None => {
                    result.push(new_path);
                }
            };
        }
    }

    (result, reachable_biomes)
}

fn deduplicate_paths(mut paths: Vec<RenderablePath>) -> Vec<RenderablePath> {
    paths.sort();
    paths.dedup();
    paths
}

fn apply_blacklist_and_boss_cells<'b>(
    paths: &Vec<ToggleablePath<'b>>,
    blacklist: &Vec<Id>,
    boss_cells: u8,
) -> Vec<ToggleablePath<'b>> {
    // todo change enabled instead of creating new paths
    paths
        .into_iter()
        .map(|path| {
            // {
            //     let str_path = path.path.iter().map(|b| b.id.to_string()).collect::<Vec<String>>();
            //     let str_path = str_path.join(",");
            //     crate::log(format!("path:: checking path {:?} cells: {}, against boss cells {}", str_path, path.minimum_boss_cells, boss_cells).as_str());
            // }
            if path.minimum_boss_cells > boss_cells {
                return ToggleablePath {
                    enabled: false,
                    path: path.path.clone(),
                    minimum_boss_cells: path.minimum_boss_cells.clone(),
                };
            }
            for biome in path.path.iter() {
                for blacklist_item in blacklist {
                    if &biome.id == blacklist_item {
                        return ToggleablePath {
                            enabled: false,
                            path: path.path.clone(),
                            minimum_boss_cells: path.minimum_boss_cells.clone(),
                        };
                    }
                }
            }
            ToggleablePath {
                enabled: true,
                path: path.path.clone(),
                minimum_boss_cells: path.minimum_boss_cells.clone(),
            }
        })
        .collect()
}

pub(crate) fn find_paths<'b>(
    biomes: &'b Vec<Biome>,
    id: Option<Id>,
) -> Result<Vec<ToggleablePath>, String> {
    let start = biomes.first().unwrap();
    let start = ToggleablePath {
        enabled: true,
        path: vec![start],
        minimum_boss_cells: 0,
    };
    let end = id.unwrap_or_else(|| biomes.last().unwrap().id.clone());

    let paths = find_path_rec(biomes, start, &end);

    Ok(paths)
}

fn find_path_rec<'b>(
    all_biomes: &'b Vec<Biome>,
    current_path: ToggleablePath<'b>,
    end: &Id,
) -> Vec<ToggleablePath<'b>> {
    let last_biome_in_path = current_path
        .path
        .last()
        .expect("There should be an element here");
    if &last_biome_in_path.id == end {
        return vec![current_path];
    }

    let exit_ids: Vec<(&Id, u8)> = last_biome_in_path
        .exits
        .iter()
        .map(|exit| {
            let destination_id: &Id = &exit.destination;
            let temp = exit.boss_cell_requirement.clone();
            let boss_cell_requirement: u8 = temp.unwrap_or(0);
            (destination_id, boss_cell_requirement)
        })
        .collect();

    let next_biomes: Vec<(&Biome, u8)> = all_biomes
        .iter()
        .filter_map(|biome| {
            let (contains, required_boss_cells) =
                contains_and_required_boss_cells(&exit_ids, &biome.id);
            if contains {
                Some((biome, required_boss_cells))
            } else {
                None
            }
        })
        .collect();

    let mut paths = vec![];
    for (next_biome, required_boss_cells) in next_biomes {
        let mut next_path = current_path.clone();
        next_path.update_minimum_boss_cells(required_boss_cells);
        // {
        //     let from = next_path.path.last().unwrap();
        //     let from = &from.id;
        //     crate::log(format!("Travelling from {:?} to {:?} requires {}", from, next_biome.id, required_boss_cells).as_str());
        // }
        next_path.path.push(next_biome);
        let mut new_paths = find_path_rec(all_biomes, next_path, end);
        paths.append(&mut new_paths)
    }

    paths
}

// todo maybe create and return enum
fn contains_and_required_boss_cells(exits: &Vec<(&Id, u8)>, expected_id: &Id) -> (bool, u8) {
    for (id, boss_cells) in exits {
        if &expected_id == id {
            return (true, *boss_cells);
        }
    }
    return (false, 0);
}

// fn calculate_all_paths_simple(biomes: &Vec<Biome>) -> Vec<Vec<SimplePath>> {
//     let start: &Biome = biomes.first().into_iter().collect();
//     let end: &Id = biomes.last().map(|biome|&biome.id).unwrap();
//
//     //special code to handle the the start node
//     let mut result:Vec<Vec<SimplePath>> = vec![];
//     for exit in start.exits {
//         let path = vec![SimplePath{ start: start.id.clone(), end: exit.destination.clone() }];
//         result.append(&mut calculate_all_paths_rec(biomes, path, end));
//     }
//
//     println("Paths: {:?}", result);
//
//     result
// }
//
// fn calculate_all_paths_rec(
//     biomes: &Vec<Biome>,
//     current_path: Vec<SimplePath>,
//     end: &Id,
// ) -> Vec<Vec<SimplePath>>{
//     let last_biome_in_path = current_path
//         .last()
//         .map(|path|path.start)
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
//         next_path.push(SimplePath{start: last_biome_in_path.clone(), end: next_biome.id.clone()});
//         let mut new_paths = find_path_rec2(all_biomes, next_path, end);
//         paths.append(&mut new_paths)
//     }
//
//     paths
// }
//
// pub fn find_paths2<'b>(
//     biomes: &'b Vec<Biome>,
//     start: &Id,
//     end: &Id,
// ) -> Result<Vec<Vec<&'b Biome>>, String> {
//     let start = biomes
//         .iter()
//         .find(|biome| &biome.id == start)
//         .ok_or(format!("Couldn't find start node {:?}", start))?;
//
//     let paths = find_path_rec2(biomes, vec![&start], end);
//
//     Ok(paths)
// }
//
// fn find_path_rec2<'b>(
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
//         let mut new_paths = find_path_rec2(all_biomes, next_path, end);
//         paths.append(&mut new_paths)
//     }
//
//     paths
// }

#[derive(Clone)]
struct SimplePath {
    start: Id,
    end: Id,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::models::ScrollFragments;

    #[test]
    fn calculate_all_paths() -> Result<(), String> {
        let biomes: Vec<Biome> = vec![
            (
                Id::Prisonquart,
                1,
                1,
                vec![
                    (Id::Arboretum, 0),
                    (Id::Promenade, 0),
                    (Id::Toxicsewers, 0),
                    (Id::Ossuary, 5),
                ],
            )
                .into(),
            (Id::Arboretum, 2, 1, vec![(Id::Prisondepths, 0)]).into(),
            (
                Id::Promenade,
                2,
                2,
                vec![(Id::Ossuary, 0), (Id::Corruptedprison, 0)],
            )
                .into(),
            (Id::Toxicsewers, 2, 3, vec![]).into(),
            (Id::Prisondepths, 3, 1, vec![(Id::Ossuary, 0)]).into(),
            (Id::Corruptedprison, 3, 2, vec![(Id::Ossuary, 0)]).into(),
            (Id::Ossuary, 4, 1, vec![]).into(),
        ];

        let result = find_paths(&biomes, None)?;
        // result
        //     .iter()
        //     .for_each(|path| println!("path: {:?} - {:?} required cells: {:?}", path_to_ids(&path.path), path.enabled, path.minimum_boss_cells));
        // todo check reachable biomes
        let (result, _) = get_paths_from(&biomes, &result, &vec![Id::Arboretum], 4);

        // let result = find_paths(&biomes)?;
        //
        //
        //
        // let result = apply_blacklist(&result, &vec![Id::Arboretum]);
        //
        // let result = biomes_paths_to_paths(result);

        result
            .iter()
            .for_each(|path| println!("path: {:?} - {:?}", path.id, path.enabled));
        // println!("paths: {:?}", result);

        assert_eq!(
            result,
            vec![
                RenderablePath {
                    id: "prisonquart-arboretum".to_string(),
                    start_column: 1,
                    start_column_offset: 0,
                    start_columns: 1,
                    end_column: 1,
                    end_column_offset: 0,
                    end_columns: 3,
                    row: 1,
                    length: 1,
                    enabled: false,
                },
                RenderablePath {
                    id: "arboretum-prisondepths".to_string(),
                    start_column: 1,
                    start_column_offset: 0,
                    start_columns: 3,
                    end_column: 1,
                    end_column_offset: 0,
                    end_columns: 2,
                    row: 2,
                    length: 1,
                    enabled: false,
                },
                RenderablePath {
                    id: "prisondepths-ossuary".to_string(),
                    start_column: 1,
                    start_column_offset: 0,
                    start_columns: 2,
                    end_column: 1,
                    end_column_offset: 0,
                    end_columns: 1,
                    row: 3,
                    length: 1,
                    enabled: false,
                },
                RenderablePath {
                    id: "prisonquart-promenade".to_string(),
                    start_column: 1,
                    start_column_offset: 0,
                    start_columns: 1,
                    end_column: 2,
                    end_column_offset: 0,
                    end_columns: 3,
                    row: 1,
                    length: 1,
                    enabled: true,
                },
                RenderablePath {
                    id: "promenade-corruptedprison".to_string(),
                    start_column: 2,
                    start_column_offset: 0,
                    start_columns: 3,
                    end_column: 2,
                    end_column_offset: 0,
                    end_columns: 2,
                    row: 2,
                    length: 1,
                    enabled: true,
                },
                RenderablePath {
                    id: "corruptedprison-ossuary".to_string(),
                    start_column: 2,
                    start_column_offset: 0,
                    start_columns: 2,
                    end_column: 1,
                    end_column_offset: 0,
                    end_columns: 1,
                    row: 3,
                    length: 1,
                    enabled: true,
                },
                RenderablePath {
                    id: "promenade-ossuary".to_string(),
                    start_column: 2,
                    start_column_offset: 0,
                    start_columns: 3,
                    end_column: 1,
                    end_column_offset: 0,
                    end_columns: 1,
                    row: 2,
                    length: 2,
                    enabled: true,
                },
                RenderablePath {
                    id: "prisonquart-ossuary".to_string(),
                    start_column: 1,
                    start_column_offset: 0,
                    start_columns: 1,
                    end_column: 1,
                    end_column_offset: 0,
                    end_columns: 1,
                    row: 1,
                    length: 3,
                    enabled: false
                },
            ]
        );

        Ok(())
    }

    impl From<(Id, usize, usize, Vec<(Id, u8)>)> for Biome {
        fn from((id, row, column, exits): (Id, usize, usize, Vec<(Id, u8)>)) -> Self {
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
                enabled: true,
            }
        }
    }

    fn prettyify_paths(paths: &Vec<RenderablePath>) -> Vec<String> {
        paths.iter().map(|path| path.id.clone()).collect()
    }

    fn path_to_ids<'b>(path: &Vec<&'b Biome>) -> Vec<&'b Id> {
        path.iter().map(|biome| &biome.id).collect()
    }
}
