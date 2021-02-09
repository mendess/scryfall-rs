//! This module exposes the possible errors this crate has, and ways to interact
//! with them.
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use ureq::Error as UreqError;

use std::fmt;

/// The result type used to describe all falible operations of the scryfall crate.
pub type Result<T> = std::result::Result<T, Error>;

/// The errors that may occur when interacting with the scryfall API.
#[derive(Debug)]
pub enum Error {
    /// Couldn't parse the json returned from scryfall. This error should never
    /// occur. If it does, please
    /// [open an issue](https://github.com/Mendess2526/scryfall-rs/issues).
    JsonError(SerdeError),
    /// Something went wrong when making the HTTP request.
    UreqError(UreqError),
    /// Scryfall error. Please refer to the [official docs](https://scryfall.com/docs/api/errors).
    ScryfallError(ScryfallError),
    /// Other.
    Other(String),
}

impl std::error::Error for Error {}

/// An Error object represents a failure to find information or understand the input you provided
/// to the API.
///
/// [Official docs](https://scryfall.com/docs/api/errors)
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ScryfallError {
    /// A human-readable string explaining the error.
    pub details: String,
    /// If your input also generated non-failure warnings, they will be provided as human-readable
    /// strings in this array.
    #[serde(default = "Default::default")]
    pub warnings: Vec<String>,
}

#[doc(hidden)]
impl From<SerdeError> for Error {
    fn from(error: SerdeError) -> Self {
        Error::JsonError(error)
    }
}

#[doc(hidden)]
impl From<UreqError> for Error {
    fn from(error: UreqError) -> Self {
        Error::UreqError(error)
    }
}

#[doc(hidden)]
impl From<ScryfallError> for Error {
    fn from(error: ScryfallError) -> Self {
        Error::ScryfallError(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::Other(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            JsonError(e) => write!(f, "Error deserializing json: {}", e),
            UreqError(e) => write!(f, "Error making request: {}", e),
            ScryfallError(e) => write!(
                f,
                "Scryfall error:\n\tdetails: {}{}",
                e.details,
                if e.warnings.is_empty() {
                    String::new()
                } else {
                    format!(
                        "\n\twarnings:\n{}",
                        e.warnings.iter().map(|w| format!("\t\t{}", w)).join("\n")
                    )
                }
            ),
            Other(s) => write!(f, "{}", s),
        }
    }
}
