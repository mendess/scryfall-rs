//! Enum defining the colors a mtg card border can have.
use serde::Deserialize;

/// Enum defining the colors a mtg card border can have.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum BorderColor {
    Black,
    Borderless,
    Gold,
    White,
    Silver,
}
