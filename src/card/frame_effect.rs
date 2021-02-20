//! Enum describing the various frame effects a border can have.
use serde::{Deserialize, Serialize};

/// Enum describing the various frame effects a border can have.
///
/// [Official docs](https://scryfall.com/docs/api/layouts)
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum FrameEffect {
    Legendary,
    Miracle,
    Nyxtouched,
    Draft,
    Devoid,
    Tombstone,
    Colorshifted,
    Inverted,
    SunMoonDfc,
    CompassLandDfc,
    OriginPwDfc,
    MoonEldraziDfc,
    MoonReverseMoonDfc,
    WaxingAndWaningMoonDfc,
    Showcase,
    ExtendedArt,
    Companion,
    Nyxborn,
    FullArt,
    Etched,
    Snow,
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
                SunMoonDfc => "sunmoondfc",
                CompassLandDfc => "compasslanddfc",
                OriginPwDfc => "originpwdfc",
                MoonEldraziDfc => "mooneldrazidfc",
                MoonReverseMoonDfc => "moonreversemoondfc",
                WaxingAndWaningMoonDfc => "waxingandwaningmoondfc",
                Showcase => "showcase",
                ExtendedArt => "extendedart",
                Companion => "companion",
                Nyxborn => "nyxborn",
                FullArt => "fullart",
                Etched => "etched",
                Snow => "snow",
            }
        )
    }
}
