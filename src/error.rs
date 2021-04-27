//! This module exposes the possible errors this crate has, and ways to interact
//! with them.
use std::{fmt, io};

use httpstatus::StatusCode;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use ureq::Error as UreqError;
use url::ParseError as UrlParseError;

/// The errors that may occur when interacting with the scryfall API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Couldn't parse the json returned from scryfall. This error should never
    /// occur. If it does, please
    /// [open an issue](https://github.com/Mendess2526/scryfall-rs/issues).
    #[error("Error deserializing json: {0}")]
    JsonError(#[from] SerdeError),

    /// Couldn't write URL query params.
    #[error("Error writing URL query: {0}")]
    UrlEncodedError(#[from] serde_urlencoded::ser::Error),

    /// A URL could not be parsed.
    #[error("Error parsing URL: {0}")]
    UrlParseError(#[from] UrlParseError),

    /// Something went wrong when making the HTTP request.
    #[error("Error making request: {0}")]
    UreqError(Box<UreqError>, String),

    /// Scryfall error. Please refer to the [official docs](https://scryfall.com/docs/api/errors).
    #[error("Scryfall error: {0}")]
    ScryfallError(ScryfallError),

    /// HTTP error with status code.
    #[error("HTTP error: {0}")]
    HttpError(StatusCode),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    /// Other.
    #[error("{0}")]
    Other(String),
}

impl From<SerdeError> for Box<Error> {
    fn from(err: SerdeError) -> Self {
        Box::new(err.into())
    }
}

impl From<UrlParseError> for Box<Error> {
    fn from(err: UrlParseError) -> Self {
        Box::new(err.into())
    }
}

/// An Error object represents a failure to find information or understand the
/// input you provided to the API.
///
/// [Official docs](https://scryfall.com/docs/api/errors)
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ScryfallError {
    /// An integer HTTP status code for this error.
    pub status: u16,

    /// A computer-friendly string representing the appropriate HTTP status
    /// code.
    pub code: String,

    /// A human-readable string explaining the error.
    pub details: String,

    /// A computer-friendly string that provides additional context for the main
    /// error. For example, an endpoint many generate HTTP 404 errors for
    /// different kinds of input. This field will provide a label for the
    /// specific kind of 404 failure, such as ambiguous.
    #[serde(rename = "type")]
    pub error_type: Option<String>,

    /// If your input also generated non-failure warnings, they will be provided
    /// as human-readable strings in this array.
    #[serde(default)]
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
