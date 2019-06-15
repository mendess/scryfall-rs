//! Enum describing the various magic card frames
use serde::{Deserialize, Serialize};

/// Enum describing the various magic card frames
///
/// [Official docs](https://scryfall.com/docs/api/layouts)
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Frame {
    #[serde(rename = "1993")]
    Y1993,
    #[serde(rename = "1997")]
    Y1997,
    #[serde(rename = "2003")]
    Y2003,
    #[serde(rename = "2015")]
    Y2015,
    #[serde(rename = "future")]
    Future,
}

impl std::fmt::Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Frame::*;
        write!(
            f,
            "{}",
            match self {
                Y1993 => "1993",
                Y1997 => "1997",
                Y2003 => "2003",
                Y2015 => "2015",
                Future => "future",
            }
        )
    }
}
