//! Enum defining the 4 different rarities a card can come in.
use std::fmt;

use serde::{Deserialize, Serialize};

/// Enum defining the 4 different rarities a card can come in.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Special,
    Mythic,
    Bonus,
}

impl fmt::Display for Rarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rarity::Common => "common",
                Rarity::Uncommon => "uncommon",
                Rarity::Rare => "rare",
                Rarity::Special => "special",
                Rarity::Mythic => "mythic",
                Rarity::Bonus => "bonus",
            }
        )
    }
}
