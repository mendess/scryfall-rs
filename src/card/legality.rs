//! Enum describing the 4 states of legality a card can have.
use serde::Deserialize;

/// Enum describing the 4 states of legality a card can have.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Legality {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}
