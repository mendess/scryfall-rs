//! Scryfall provides daily exports of their card data in bulk files. Each of
//! these files is represented as a bulk_data object via the API. URLs for files
//! change their timestamp each day, and can be fetched programmatically.
//!
//! # Warning
//!
//! These bulk dumps are not paginated, this means that they will be potentially
//! stored in memory in its entirety while being iterated over.
//!
//! # Features
//!
//! With the `bulk_caching` feature enabled, bulk data files will be stored in
//! the OS temp folder. This prevents duplicate downloads if the version has
//! already been saved.
//!
//! See also: [Official Docs](https://scryfall.com/docs/api/bulk-data)

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::Path;

use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use uuid::Uuid;

use crate::card::Card;
use crate::ruling::Ruling;
use crate::uri::Uri;
use crate::util::array_stream_reader::ArrayStreamReader;
use crate::util::BULK_DATA_URL;
use crate::Error;

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
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
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
    pub compressed_size: Option<usize>,

    /// The MIME type of this file.
    pub content_type: String,

    /// The Content-Encoding encoding that will be used to transmit this file
    /// when you download it.
    pub content_encoding: String,

    /// The byte size of the bulk file.
    pub size: usize,

    #[cfg(test)]
    #[serde(rename = "object")]
    _object: String,
}

impl<T: DeserializeOwned> BulkDataFile<T> {
    cfg_if! {
        if #[cfg(feature = "bulk_caching")] {
            /// The full temp path where this file will be downloaded with `load`. The
            /// file name has the form "&lt;type&gt;-&lt;date&gt;.json".
            fn cache_path(&self) -> std::path::PathBuf {
                use heck::ToKebabCase;
                std::env::temp_dir().join(format!(
                    "{}-{}.json",
                    self.bulk_type.to_kebab_case(),
                    self.updated_at.format("%Y%m%d%H%M%S"),
                ))
            }



            async fn get_reader(&self) -> crate::Result<BufReader<File>> {
                let cache_path = self.cache_path();
                if !cache_path.exists() {
                    self.download(&cache_path).await?;
                }
                Ok(BufReader::new(File::open(cache_path)?))
            }
        } else {
            async fn get_reader(&self) -> crate::Result<BufReader<impl io::Read + Send>> {
                Ok(BufReader::new(self.download_uri.fetch_raw().await?.into_reader()))
            }
        }
    }

    /// Gets a BulkDataFile of the specified type.
    pub async fn of_type(bulk_type: &str) -> crate::Result<Self> {
        Uri::from(BULK_DATA_URL.join(bulk_type)?).fetch().await
    }

    /// Gets a BulkDataFile with the specified unique ID.
    pub async fn id(id: Uuid) -> crate::Result<Self> {
        Uri::from(BULK_DATA_URL.join(id.to_string().as_str())?)
            .fetch()
            .await
    }

    /// Loads the objects from this bulk data download into a `Vec`.
    ///
    /// Downloads and stores the file in the computer's temp folder if this
    /// version hasn't been downloaded yet. Otherwise uses the stored copy.
    pub async fn load(&self) -> crate::Result<Vec<T>> {
        Ok(serde_json::from_reader(self.get_reader().await?)?)
    }

    /// Returns an iterator over the objects from this bulk data download.
    ///
    /// Downloads and stores the file in the computer's temp folder if this
    /// version hasn't been downloaded yet. Otherwise uses the stored copy.
    pub async fn load_iter(&self) -> crate::Result<impl Iterator<Item = crate::Result<T>>> {
        let de = serde_json::Deserializer::from_reader(ArrayStreamReader::new_buffered(
            self.get_reader().await?,
        ));
        Ok(de.into_iter().map(|item| item.map_err(|e| e.into())))
    }

    /// Downloads this file, saving it to `path`. Overwrites the file if it
    /// already exists.
    pub async fn download(&self, path: impl AsRef<Path>) -> crate::Result<()> {
        let path = path.as_ref();
        let response = self.download_uri.fetch_raw().await?;
        let content = response.bytes().await.map_err(|e| Error::ReqwestError {
            error: e.into(),
            url: self.download_uri.inner().clone(),
        })?;
        io::copy(&mut content.as_ref(), &mut File::create(path)?)?;
        Ok(())
    }
}

/// An iterator containing one Scryfall card object for each Oracle ID on
/// Scryfall. The chosen sets for the cards are an attempt to return the most
/// up-to-date recognizable version of the card.
pub async fn oracle_cards() -> crate::Result<impl Iterator<Item = crate::Result<Card>>> {
    BulkDataFile::of_type("oracle_cards")
        .await?
        .load_iter()
        .await
}

/// An iterator of Scryfall card objects that together contain all unique
/// artworks. The chosen cards promote the best image scans.
pub async fn unique_artwork() -> crate::Result<impl Iterator<Item = crate::Result<Card>>> {
    BulkDataFile::of_type("unique_artwork")
        .await?
        .load_iter()
        .await
}

/// An iterator containing every card object on Scryfall in English or the
/// printed language if the card is only available in one language.
pub async fn default_cards() -> crate::Result<impl Iterator<Item = crate::Result<Card>>> {
    BulkDataFile::of_type("default_cards")
        .await?
        .load_iter()
        .await
}

/// An iterator of every card object on Scryfall in every language.
///
/// # Note
/// This currently takes about 2GB of RAM before returning 👀.
pub async fn all_cards() -> crate::Result<impl Iterator<Item = crate::Result<Card>>> {
    BulkDataFile::of_type("all_cards").await?.load_iter().await
}

/// An iterator of all Rulings on Scryfall. Each ruling refers to cards via an
/// `oracle_id`.
pub async fn rulings() -> crate::Result<impl Iterator<Item = crate::Result<Ruling>>> {
    BulkDataFile::of_type("rulings").await?.load_iter().await
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    #[ignore]
    async fn oracle_cards() {
        for card in super::oracle_cards().await.unwrap() {
            card.unwrap();
        }
    }

    #[tokio::test]
    #[ignore]
    async fn unique_artwork() {
        for card in super::unique_artwork().await.unwrap() {
            card.unwrap();
        }
    }

    #[tokio::test]
    #[ignore]
    async fn default_cards() {
        for card in super::default_cards().await.unwrap() {
            card.unwrap();
        }
    }

    #[tokio::test]
    #[ignore]
    async fn all_cards() {
        for card in super::all_cards().await.unwrap() {
            card.unwrap();
        }
    }

    #[tokio::test]
    #[ignore]
    async fn rulings() {
        for ruling in super::rulings().await.unwrap() {
            ruling.unwrap();
        }
    }

    #[test]
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
        Deserializer::from_reader(super::ArrayStreamReader::new_buffered(s.as_bytes()))
            .into_iter()
            .map(|r: serde_json::Result<Ruling>| r.unwrap())
            .for_each(drop);
    }
}
