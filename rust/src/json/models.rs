use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
    #[serde(alias = "0")]
    pub zero: Option<u8>,
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
    pub fn get_fragments(&self, boss_cells: &BossCells) -> u8 {
        let zero = self.zero;
        let one = self.one.or(zero);
        let two = self.two.or(one);
        let three = self.three.or(two);
        let four = self.four.or(three);
        let five = self.five.or(four);

        match boss_cells {
            BossCells::Zero => zero.unwrap_or(0),
            BossCells::One => one.unwrap_or(0),
            BossCells::Two => two.unwrap_or(0),
            BossCells::Three => three.unwrap_or(0),
            BossCells::Four => four.unwrap_or(0),
            BossCells::Five => five.unwrap_or(0),
        }
    }
}

#[derive(Clone)]
pub enum BossCells {
    Zero,
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
