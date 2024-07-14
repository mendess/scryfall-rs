use serde::{Deserialize, Serialize};

/// The security stamp on this card, if any.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum SecurityStamp {
    Oval,
    Triangle,
    Acorn,
    Circle,
    Arena,
    Heart,
}
