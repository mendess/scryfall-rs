//! The available magic the gathering formats.
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(not(feature = "unknown_variants"), derive(Copy))]
#[cfg_attr(
    all(
        not(feature = "unknown_variants"),
        not(feature = "unknown_variants_slim")
    ),
    non_exhaustive
)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum Format {
    Standard,
    Modern,
    Legacy,
    Vintage,
    Commander,
    Future,
    Pauper,
    Pioneer,
    Penny,
    Duel,
    OldSchool,
    Historic,
    Gladiator,
    Brawl,
    Premodern,
    PauperCommander,
    Alchemy,
    Explorer,
    Predh,
    Oathbreaker,
    Timeless,
    StandardBrawl,
    HistoricBrawl,
    #[cfg(feature = "unknown_variants")]
    Unknown(Box<str>),
    #[cfg(all(not(feature = "unknown_variants"), feature = "unknown_variants_slim"))]
    Unknown,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Format::*;
        write!(
            f,
            "{}",
            match self {
                Standard => "standard",
                Modern => "modern",
                Legacy => "legacy",
                Vintage => "vintage",
                Commander => "commander",
                Future => "future",
                Pauper => "pauper",
                Pioneer => "pioneer",
                Penny => "penny",
                Duel => "duel",
                OldSchool => "oldschool",
                Historic => "historic",
                Gladiator => "gladiator",
                Brawl => "brawl",
                Premodern => "premodern",
                PauperCommander => "paupercommander",
                Alchemy => "alchemy",
                Explorer => "explorer",
                Predh => "predh",
                Oathbreaker => "oathbreaker",
                Timeless => "timeless",
                StandardBrawl => "standardbrawl",
                HistoricBrawl => "historicbrawl",
                #[cfg(feature = "unknown_variants")]
                Unknown(s) => s,
                #[cfg(all(not(feature = "unknown_variants"), feature = "unknown_variants_slim"))]
                Unknown => "unknown-format",
            }
        )
    }
}
