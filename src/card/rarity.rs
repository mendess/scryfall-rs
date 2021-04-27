//! Enum defining the 4 different rarities a card can come in.
use std::fmt;

use serde::{Deserialize, Serialize};

/// The rarities a card can be printed in. Aside from the usual 4 of
/// `Common`, `Uncommon`, `Rare`, and `Mythic`, there are two additional
/// rarities.
/// - `Special` is used for timeshifted cards and has a [purple symbol](https://scryfall.com/card/tsb/24/lord-of-atlantis).
/// - `Bonus` is used for the power nine in Vintage Masters, and has a
///   ["glowing" mythic symbol](https://scryfall.com/card/vma/4/black-lotus).
///
/// For the purposes of sorting and comparison, `Special` is considered above
/// `Rare` and below `Mythic`, and `Bonus` is the rarest, above `Mythic.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Rarity {
    /// Black set symbol.
    Common,
    /// Silver set symbol.
    Uncommon,
    /// Gold set symbol.
    Rare,
    /// Purple set symbol, used for timeshifted cards.
    Special,
    /// Orange set symbol.
    Mythic,
    /// "Glowing" mythic symbol, used for the power nine in VMA.
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
