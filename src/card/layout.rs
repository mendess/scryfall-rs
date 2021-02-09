//! Enum describing the various layouts a magic card can have.
use serde::{Deserialize, Serialize};

/// Enum describing the various layouts a magic card can have.
///
/// [Official docs](https://scryfall.com/docs/api/layouts)
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Layout {
    Normal,
    Split,
    Flip,
    Transform,
    Meld,
    Leveler,
    Saga,
    Adventure,
    Planar,
    Scheme,
    Vanguard,
    Token,
    DoubleFacedToken,
    Emblem,
    Augment,
    Host,
    ArtSeries,
    DoubleSided,
    ModalDfc,
}
