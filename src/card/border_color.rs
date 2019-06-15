//! Enum defining the colors a mtg card border can have.
use serde::{Deserialize, Serialize};

/// Enum defining the colors a mtg card border can have.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum BorderColor {
    Black,
    Borderless,
    Gold,
    White,
    Silver,
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
            }
        )
    }
}
