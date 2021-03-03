//! Enum defining the 4 different rarities a card can come in.
use serde::{Deserialize, Serialize};

/// Enum defining the 4 different rarities a card can come in.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythic,
    Bonus,
    Special,
}
