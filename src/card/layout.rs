//! Enum describing the various layouts a magic card can have.
use serde::Deserialize;

/// Enum describing the various layouts a magic card can have.
///
/// [Official docs](https://scryfall.com/docs/api/layouts)
#[derive(Deserialize, Debug, Clone, Copy)]
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
