//! Enum defining the exiting platforms on with a magic card can exist.
use serde::Deserialize;

/// Enum defining the exiting platforms on with a magic card can exist.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Game {
    Paper,
    Arena,
    Mtgo,
}
