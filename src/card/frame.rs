//! Enum describing the various magic card frames
use serde::Deserialize;

/// Enum describing the various magic card frames
#[derive(Deserialize, Debug, Clone, Copy)]
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
