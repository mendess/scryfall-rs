//! Enum describing the various frame effects a border can have.
use serde::Deserialize;

/// Enum describing the various frame effects a border can have.
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum FrameEffect {
    Legendary,
    Miracle,
    Nyxtouched,
    Draft,
    Devoid,
    Tombstone,
    Colorshifted,
    Sunmoondfc,
    Compasslanddfc,
    Originpwdfc,
    Mooneldrazidfc,
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
                Sunmoondfc => "sunmoondfc",
                Compasslanddfc => "compasslanddfc",
                Originpwdfc => "originwdfc",
                Mooneldrazidfc => "mooneldrazidfc",
            }
        )
    }
}
