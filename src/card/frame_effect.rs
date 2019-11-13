//! Enum describing the various frame effects a border can have.
use serde::{Deserialize, Serialize};

/// Enum describing the various frame effects a border can have.
///
/// [Official docs](https://scryfall.com/docs/api/layouts)
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum FrameEffect {
    Legendary,
    Miracle,
    Nyxtouched,
    Draft,
    Devoid,
    Tombstone,
    Colorshifted,
    Inverted,
    Sunmoondfc,
    Compasslanddfc,
    Originpwdfc,
    Mooneldrazidfc,
    Moonreversemoondfc,
    Showcase,
    Extendedart,
}

impl std::fmt::Display for FrameEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use FrameEffect::*;
        write!(
            f,
            "{}",
            match self {
                Legendary => "legendary",
                Miracle => "miracle",
                Nyxtouched => "nyxtouched",
                Draft => "draft",
                Devoid => "devoid",
                Tombstone => "tombstone",
                Colorshifted => "colorshifted",
                Inverted => "inverted",
                Sunmoondfc => "sunmoondfc",
                Compasslanddfc => "compasslanddfc",
                Originpwdfc => "originwdfc",
                Mooneldrazidfc => "mooneldrazidfc",
                Moonreversemoondfc => "Moonreversemoondfc",
                Showcase => "showcase",
                Extendedart => "extendedart",
            }
        )
    }
}
