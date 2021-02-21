use serde::{Deserialize, Serialize};

/// The frame field tracks the major edition of the card frame of used for the
/// re/print in question. The frame has gone though several major revisions in
/// Magic’s lifetime.
///
/// [Official docs](https://scryfall.com/docs/api/layouts#frames)
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Frame {
    /// The original Magic card frame, starting from Limited Edition Alpha.
    #[serde(rename = "1993")]
    Y1993,
    /// The updated classic frame starting from Mirage block.
    #[serde(rename = "1997")]
    Y1997,
    /// The “modern” Magic card frame, introduced in Eighth Edition and Mirrodin
    /// block.
    #[serde(rename = "2003")]
    Y2003,
    /// The holofoil-stamp Magic card frame, introduced in Magic 2015.
    #[serde(rename = "2015")]
    Y2015,
    /// The frame used on cards from the future.
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
