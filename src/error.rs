//! This module exposes the possible errors this crate has, and ways to interact
//! with them.
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use ureq::Error as UreqError;

use std::fmt;

/// The errors that may occur when interacting with the scryfall API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Couldn't parse the json returned from scryfall. This error should never
    /// occur. If it does, please
    /// [open an issue](https://github.com/Mendess2526/scryfall-rs/issues).
    #[error("Error deserializing json: {0}")]
    JsonError(#[from] SerdeError),

    /// Something went wrong when making the HTTP request.
    #[error("Error making request: {0}")]
    UreqError(#[from] UreqError),

    /// Scryfall error. Please refer to the [official docs](https://scryfall.com/docs/api/errors).
    #[error("Scryfall error: {0}")]
    ScryfallError(ScryfallError),

    /// HTTP error with status code and message.
    #[error("HTTP error: {0} {1}")]
    HttpError(u16, String),

    /// Other.
    #[error("{0}")]
    Other(String),
}

/// An Error object represents a failure to find information or understand the input you provided
/// to the API.
///
/// [Official docs](https://scryfall.com/docs/api/errors)
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ScryfallError {
    /// A human-readable string explaining the error.
    pub details: String,
    /// If your input also generated non-failure warnings, they will be provided as human-readable
    /// strings in this array.
    #[serde(default = "Default::default")]
    pub warnings: Vec<String>,
}

impl fmt::Display for ScryfallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n\tdetails:{}{}",
            self.details,
            if self.warnings.is_empty() {
                String::new()
            } else {
                format!(
                    "\n\twarnings:\n{}",
                    self.warnings
                        .iter()
                        .map(|w| format!("\t\t{}", w))
                        .join("\n")
                )
            }
        )
    }
}
