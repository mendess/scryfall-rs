//! Module containing utility functions and structs.
pub mod uri;

/// The [scryfall](https://scryfall.com/docs/api) endpoint.
pub const API: &str = "https://api.scryfall.com";
/// The [cards](https://scryfall.com/docs/api/cards) endpoint.
pub const API_CARDS: &str = "/cards";
/// The [sets](https://scryfall.com/docs/api/sets) endpoint.
pub const API_SETS: &str = "/sets";
/// The [rulings](https://scryfall.com/docs/api/sets) endpoint.
pub const API_RULING: &str = "/rulings";
/// The [bulk-data](https://scryfall.com/docs/api/bulk-data) endpoint.
pub const API_BULK_DATA: &str = "/bulk-data";

/// A type alias for a Uuid.
pub type Uuid = String;
