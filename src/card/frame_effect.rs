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
