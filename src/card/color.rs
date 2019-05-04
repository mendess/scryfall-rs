//! Enum defining the 5 colors of magic
use serde::Deserialize;

/// Enum defining the 5 colors of magic
#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Color {
    #[serde(rename = "W")]
    White,
    #[serde(rename = "U")]
    Blue,
    #[serde(rename = "B")]
    Black,
    #[serde(rename = "R")]
    Red,
    #[serde(rename = "G")]
    Green,
}
