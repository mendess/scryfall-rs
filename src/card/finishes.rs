use serde::{Deserialize, Serialize};

/// The finish the card can come in.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[cfg_attr(not(feature = "unknown_variants"), derive(Copy))]
#[cfg_attr(
    all(
        not(feature = "unknown_variants"),
        not(feature = "unknown_variants_slim")
    ),
    non_exhaustive
)]
#[serde(rename_all = "lowercase")]
pub enum Finishes {
    /// Nonfoil.
    Nonfoil,
    /// Foil.
    Foil,
    /// Etched foil.
    Etched,
    #[cfg_attr(
        doc,
        doc(cfg(any(feature = "unknown_variants", feature = "unknown_variants_slim")))
    )]
    #[cfg(feature = "unknown_variants")]
    /// Unknown frame effect
    Unknown(Box<str>),
    #[cfg_attr(
        doc,
        doc(cfg(any(feature = "unknown_variants", feature = "unknown_variants_slim")))
    )]
    #[cfg(all(not(feature = "unknown_variants"), feature = "unknown_variants_slim"))]
    /// Unknown frame effect
    Unknown,
}
