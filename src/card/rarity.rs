//! Enum defining the 4 different rarities a card can come in.
use serde::Deserialize;

/// Enum defining the 4 different rarities a card can come in.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythic,
}
