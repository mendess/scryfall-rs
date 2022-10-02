use serde::{Deserialize, Serialize};

/// The frame_effects field tracks additional frame artwork applied over a
/// particular frame. For example, there are both 2003 and 2015-frame cards with
/// the Nyx-touched effect.
///
/// [Official docs](https://scryfall.com/docs/api/layouts#frame-effects)
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum FrameEffect {
    /// The cards have a legendary crown.
    Legendary,
    /// The miracle frame effect.
    Miracle,
    /// The Nyx-touched frame effect.
    Nyxtouched,
    /// The draft-matters frame effect.
    Draft,
    /// The Devoid frame effect.
    Devoid,
    /// The Odyssey tombstone mark.
    Tombstone,
    /// A colorshifted frame.
    Colorshifted,
    /// The FNM-style inverted frame.
    Inverted,
    /// The sun and moon transform marks.
    SunMoonDfc,
    /// The compass and land transform marks.
    CompassLandDfc,
    /// The Origins and planeswalker transform marks.
    OriginPwDfc,
    /// The moon and Eldrazi transform marks.
    MoonEldraziDfc,
    /// The waxing and waning crescent moon transform marks.
    MoonReverseMoonDfc,
    /// A custom Showcase frame.
    Showcase,
    /// An extended art frame.
    ExtendedArt,
    /// The cards have a companion frame.
    Companion,
    /// The cards have an etched foil treatment.
    Etched,
    /// The cards have the snowy frame effect.
    Snow,
    /// The cards have the Lesson frame effect.
    Lesson,
    /// The cards have the Shattered Glass frame effect.
    ShatteredGlass,
    /// The cards have More Than Meets the Eye™ marks.
    ConvertDfc,
    /// The cards have fan transforming marks.
    FanDfc,
    /// The cards have the Upside Down transforming marks.
    UpsideDownDfc,

    /// A full art frame. Undocumented and unsupported for search.
    FullArt,
    /// A nyxborn card frame. Undocumented and unsupported for search.
    Nyxborn,
    /// The waxing and waning moon transform marks. Undocumented and unsupported
    /// for search.
    WaxingAndWaningMoonDfc,
    /// The booster card frame. Undocumented and unsupported for search.
    Booster,
    ///
    Textless,
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
                Showcase => "showcase",
                ExtendedArt => "extendedart",
                Companion => "companion",
                Etched => "etched",
                Snow => "snow",
                Lesson => "lesson",
                ShatteredGlass => "shatteredglass",
                ConvertDfc => "convertdfc",
                FanDfc => "fandfc",
                UpsideDownDfc => "upsidedowndfc",

                FullArt => "fullart",
                Nyxborn => "nyxborn",
                WaxingAndWaningMoonDfc => "waxingandwaningmoondfc",
                Booster => "booster",
                Textless => "textless",
            }
        )
    }
}
