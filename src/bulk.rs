//! Scryfall provides daily exports of their card data in bulk files. Each of
//! these files is represented as a bulk_data object via the API. URLs for files
//! change their timestamp each day, and can be fetched programmatically.
//!
//! # Warning
//!
//! These bulk dumps are not paginated, this means that they will be potentially
//! stored in memory in its entirety while being iterated over.
//!
//! See also: [Official Docs](https://scryfall.com/docs/api/bulk-data)

use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use uuid::Uuid;

use crate::card::Card;
use crate::ruling::Ruling;
use crate::uri::Uri;
use crate::util::BULK_DATA_URL;

/// Scryfall provides daily exports of our card data in bulk files. Each of
/// these files is represented as a bulk_data object via the API. URLs for files
/// change their timestamp each day, and can be fetched programmatically.
///
/// ## Please note:
///
/// * Card objects in bulk data include price information, but prices should be
///   considered dangerously stale after 24 hours. Only use bulk price
///   information to track trends or provide a general estimate of card value.
///   Prices are not updated frequently enough to power a storefront or sales
///   system. You consume price information at your own risk.
/// * Updates to gameplay data (such as card names, Oracle text, mana costs,
///   etc) are much less frequent. If you only need gameplay information,
///   downloading card data once per week or right after set releases would most
///   likely be sufficient.
/// * Every card type in every product is included, including planar cards,
///   schemes, Vanguard cards, tokens, emblems, and funny cards. Make sure
///   you’ve reviewed documentation for the Card type.
///
///
/// Bulk data is only collected once every 12 hours. You can use the card API
/// methods to retrieve fresh objects instead.
#[derive(Deserialize, Debug, Clone)]
pub struct BulkDataFile<T> {
    /// A unique ID for this bulk item.
    pub id: Uuid,

    /// The Scryfall API URI for this file.
    pub uri: Uri<BulkDataFile<T>>,

    /// A computer-readable string for the kind of bulk item.
    #[serde(rename = "type")]
    pub bulk_type: String,

    /// A human-readable name for this file.
    pub name: String,

    /// A human-readable description for this file.
    pub description: String,

    /// The URI that hosts this bulk file for fetching.
    pub download_uri: Uri<Vec<T>>,

    /// The time when this file was last updated.
    pub updated_at: DateTime<Utc>,

    /// The size of this file in integer bytes.
    pub compressed_size: usize,

    /// The MIME type of this file.
    pub content_type: String,

    /// The Content-Encoding encoding that will be used to transmit this file
    /// when you download it.
    pub content_encoding: String,
}

impl<T: DeserializeOwned> BulkDataFile<T> {
    /// Gets a BulkDataFile of the specified type.
    pub fn of_type(bulk_type: &str) -> crate::Result<Self> {
        Uri::from(BULK_DATA_URL.join(bulk_type)?).fetch()
    }

    fn download(&self) -> crate::Result<Vec<T>> {
        self.download_uri.fetch()
    }
}

/// An iterator containing one Scryfall card object for each Oracle ID on
/// Scryfall. The chosen sets for the cards are an attempt to return the most
/// up-to-date recognizable version of the card.
pub fn oracle_cards() -> crate::Result<Vec<Card>> {
    BulkDataFile::of_type("oracle_cards")?.download()
}

/// An iterator of Scryfall card objects that together contain all unique
/// artworks. The chosen cards promote the best image scans.
pub fn unique_artwork() -> crate::Result<Vec<Card>> {
    BulkDataFile::of_type("unique_artwork")?.download()
}

/// An iterator containing every card object on Scryfall in English or the
/// printed language if the card is only available in one language.
pub fn default_cards() -> crate::Result<Vec<Card>> {
    BulkDataFile::of_type("default_cards")?.download()
}

/// An iterator of every card object on Scryfall in every language.
///
/// # Note
/// This currently takes about 2GB of RAM before returning 👀.
pub fn all_cards() -> crate::Result<Vec<Card>> {
    BulkDataFile::of_type("all_cards")?.download()
}

/// An iterator of all Rulings on Scryfall. Each ruling refers to cards via an
/// `oracle_id`.
pub fn rulings() -> crate::Result<Vec<Ruling>> {
    BulkDataFile::of_type("rulings")?.download()
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn oracle_cards() {
        super::oracle_cards().expect("Couldn't get the bulk object");
    }

    #[test]
    #[ignore]
    fn unique_artwork() {
        super::unique_artwork().expect("Couldn't get the bulk object");
    }

    #[test]
    #[ignore]
    fn default_cards() {
        super::default_cards().expect("Couldn't get the bulk object");
    }

    #[test]
    #[ignore]
    fn all_cards() {
        super::all_cards().expect("Couldn't get the bulk object");
    }

    #[test]
    #[ignore]
    fn rulings() {
        super::rulings().expect("Couldn't get the bulk object");
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn test_parse_list() {
        use serde_json::Deserializer;

        use crate::ruling::Ruling;
        let s = r#"[
                      {
                        "object": "ruling",
                        "oracle_id": "0004ebd0-dfd6-4276-b4a6-de0003e94237",
                        "source": "wotc",
                        "published_at": "2004-10-04",
                        "comment": "If there are two of these on the battlefield, they do not add together. The result is that only two permanents can be untapped."
                      },
                      {
                        "object": "ruling",
                        "oracle_id": "0007c283-5b7a-4c00-9ca1-b455c8dff8c3",
                        "source": "wotc",
                        "published_at": "2019-08-23",
                        "comment": "The “commander tax” increases based on how many times a commander was cast from the command zone. Casting a commander from your hand doesn’t require that additional cost, and it doesn’t increase what the cost will be the next time you cast that commander from the command zone."
                      }
                   ]"#;
        Deserializer::from_str(s)
            .into_iter()
            .map(|r: serde_json::Result<Ruling>| r.unwrap())
            .for_each(drop);
    }
}
