//! Enum defining the colors a mtg card border can have.
use serde::{Deserialize, Serialize};

/// Enum defining the colors a mtg card border can have.
#[derive(Default, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum BorderColor {
    #[default]
    Black,
    Borderless,
    Gold,
    White,
    Silver,
    Yellow,
}

impl std::fmt::Display for BorderColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use BorderColor::*;
        write!(
            f,
            "{}",
            match self {
                Black => "black",
                Borderless => "borderless",
                Gold => "gold",
                White => "white",
                Silver => "silver",
                Yellow => "yellow",
            }
        )
    }
}
