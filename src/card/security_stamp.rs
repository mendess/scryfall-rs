use serde::{Deserialize, Serialize};

/// The security stamp on this card, if any.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[cfg_attr(
    all(
        not(feature = "unknown_variants"),
        not(feature = "unknown_variants_slim")
    ),
    non_exhaustive
)]
#[cfg_attr(not(feature = "unknown_variants"), derive(Copy))]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum SecurityStamp {
    Oval,
    Triangle,
    Acorn,
    Circle,
    Arena,
    Heart,
    #[cfg_attr(
        doc,
        doc(cfg(any(feature = "unknown_variants", feature = "unknown_variants_slim")))
    )]
    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    /// Unknown frame effect
    Unknown(Box<str>),
    #[cfg_attr(
        doc,
        doc(cfg(any(feature = "unknown_variants", feature = "unknown_variants_slim")))
    )]
    #[cfg(all(not(feature = "unknown_variants"), feature = "unknown_variants_slim"))]
    #[serde(other)]
    /// Unknown frame effect
    Unknown,
}
