//! Enum defining the exiting platforms on with a magic card can exist.
use serde::{Deserialize, Serialize};

/// Enum defining the exiting platforms on with a magic card can exist.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Game {
    Paper,
    Arena,
    Mtgo,
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Game::*;
        write!(
            f,
            "{}",
            match self {
                Paper => "paper",
                Arena => "arena",
                Mtgo => "mtgo",
            }
        )
    }
}
