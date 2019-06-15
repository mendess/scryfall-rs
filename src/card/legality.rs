//! Enum describing the 4 states of legality a card can have.
use serde::{Deserialize, Serialize};

/// Enum describing the 4 states of legality a card can have.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Legality {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}
