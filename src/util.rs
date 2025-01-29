//! Module containing utility functions and structs.
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer};
use url::Url;

pub(crate) mod stream_iterator;

/// The [scryfall](https://scryfall.com/docs/api) endpoint.
pub static ROOT_URL: Lazy<Url> = Lazy::new(|| Url::parse("https://api.scryfall.com/").unwrap());
/// The [cards](https://scryfall.com/docs/api/cards) endpoint.
pub static CARDS_URL: Lazy<Url> = Lazy::new(|| ROOT_URL.join("cards/").unwrap());
/// The [sets](https://scryfall.com/docs/api/sets) endpoint.
pub static SETS_URL: Lazy<Url> = Lazy::new(|| ROOT_URL.join("sets/").unwrap());
/// The [bulk-data](https://scryfall.com/docs/api/bulk-data) endpoint.
pub static BULK_DATA_URL: Lazy<Url> = Lazy::new(|| ROOT_URL.join("bulk-data/").unwrap());
/// The [catalog](https://scryfall.com/docs/api/catalogs) endpoint.
pub static CATALOG_URL: Lazy<Url> = Lazy::new(|| ROOT_URL.join("catalog/").unwrap());

/// The [rulings](https://scryfall.com/docs/api/rulings) path segment, which goes on the end of a
/// card URL.
pub const API_RULING: &str = "rulings/";

/// Function for use with `#[serde(deserialize_with)]` and a field that's
/// Option<T>. If deserialization fails, use `None` as the field's value and
/// don't cause an error.
pub fn deserialize_or_none<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(deserializer).map(Some).or(Ok(None))
}
