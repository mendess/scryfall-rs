//! Enum describing the various layouts a magic card can have.
use serde::{Deserialize, Serialize};

/// Enum describing the various layouts a magic card can have.
///
/// [Official docs](https://scryfall.com/docs/api/layouts)
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Layout {
    Normal,
    Split,
    Flip,
    Transform,
    Meld,
    Leveler,
    Saga,
    Planar,
    Scheme,
    Vanguard,
    Token,
    DoubleFacedToken,
    Emblem,
    Augment,
    Host,
}
