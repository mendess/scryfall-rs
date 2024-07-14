use serde::{Deserialize, Serialize};

/// The finish the card can come in.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Finishes {
    /// Nonfoil.
    Nonfoil,
    /// Foil.
    Foil,
    /// Etched foil.
    Etched,
}
