//! Module containing utility functions and structs.
pub mod uri;

/// The [scryfall](https://scryfall.com/docs/api) endpoint.
pub const API: &str = "https://api.scryfall.com";
/// The [cards](https://scryfall.com/docs/api/cards) endpoint.
pub const API_CARDS: &str = "/cards";
/// The [sets](https://scryfall.com/docs/api/sets) endpoint.
pub const API_SETS: &str = "/sets";

/// A type alias for a UUID.
pub type UUID = String;
