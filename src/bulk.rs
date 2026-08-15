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

use std::io::BufReader;
use std::path::Path;

use async_compression::tokio::bufread::GzipDecoder;
use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use futures::Stream;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncRead;
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;
use uuid::Uuid;

cfg_if! {
    if #[cfg(not(feature = "bulk_caching"))] {
        use bytes::Buf;
        use flate2::read::GzDecoder;
    }
}

use crate::card::Card;
use crate::ruling::Ruling;
use crate::uri::Uri;
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
    pub jsonl_download_uri: Uri<Vec<T>>,

    /// The time when this file was last updated.
    pub updated_at: DateTime<Utc>,

    /// The size of this file in integer bytes.
    pub compressed_size: Option<usize>,

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

            async fn get_reader(&self) -> crate::Result<BufReader<std::fs::File>> {
                let cache_path = self.cache_path();
                if !cache_path.exists() {
                    self.download(&cache_path).await?;
                }
                Ok(BufReader::new(std::fs::File::open(cache_path)?))
            }

            async fn get_async_reader(&self) -> crate::Result<impl AsyncRead> {
                let cache_path = self.cache_path();
                if !cache_path.exists() {
                    self.download(&cache_path).await?;
                }

                let file = tokio::fs::File::open(&cache_path).await?;

                let raw_reader = tokio::io::BufReader::new(file);
                Ok(GzipDecoder::new(raw_reader))
            }
        } else {
            async fn get_reader(&self) -> crate::Result<BufReader<impl std::io::Read + Send>> {

                let response = self.jsonl_download_uri.fetch_raw().await?;
                let body = response.bytes().await.map_err(|e| {
                    crate::Error::ReqwestError { error: Box::new(e), url: self.jsonl_download_uri.inner().clone() }
                })?;
                Ok(BufReader::new(GzDecoder::new(body.reader())))
            }

            async fn get_async_reader(&self) -> crate::Result<impl AsyncRead> {
                let response = self.jsonl_download_uri.fetch_raw().await?;
                let stream = response.bytes_stream()
                    .map(|bytes_result| {
                        bytes_result
                            .map_err(std::io::Error::other)
                            // .map(|bytes| bytes.to_vec())
                    });

                Ok(GzipDecoder::new(StreamReader::new(stream)))
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

    /// Returns an async Stream over the objects from this bulk data download.
    ///
    /// Downloads and stores the file in the computer's temp folder if this
    /// version hasn't been downloaded yet. Otherwise uses the stored copy.
    pub async fn load_stream(&self) -> crate::Result<impl Stream<Item = crate::Result<T>>>
    where
        T: Send + 'static,
    {
        let reader = self.get_async_reader().await?;

        Ok(
            LinesStream::new(tokio::io::BufReader::new(reader).lines()).map(|line_result| {
                line_result
                    .map_err(Error::from)
                    .and_then(|line| serde_json::from_str::<T>(&line).map_err(Error::from))
            }),
        )
    }

    /// Downloads this file, saving it to `path`. Overwrites the file if it
    /// already exists.
    pub async fn download(&self, path: impl AsRef<Path>) -> crate::Result<()> {
        let path = path.as_ref();
        let response = self.jsonl_download_uri.fetch_raw().await?;

        let body = response
            .bytes_stream()
            .map(|bytes_result| bytes_result.map_err(std::io::Error::other));
        let mut file = tokio::fs::File::create(path).await?;

        tokio::io::copy(&mut StreamReader::new(body), &mut file).await?;

        Ok(())
    }
}

/// An async Stream containing one Scryfall card object for each Oracle ID on
/// Scryfall. The chosen sets for the cards are an attempt to return the most
/// up-to-date recognizable version of the card.
pub async fn oracle_cards() -> crate::Result<impl Stream<Item = crate::Result<Card>>> {
    BulkDataFile::of_type("oracle_cards")
        .await?
        .load_stream()
        .await
}

/// An async Stream of Scryfall card objects that together contain all unique
/// artworks. The chosen cards promote the best image scans.
pub async fn unique_artwork() -> crate::Result<impl Stream<Item = crate::Result<Card>>> {
    BulkDataFile::of_type("unique_artwork")
        .await?
        .load_stream()
        .await
}

/// An async Stream containing every card object on Scryfall in English or the
/// printed language if the card is only available in one language.
pub async fn default_cards() -> crate::Result<impl Stream<Item = crate::Result<Card>>> {
    BulkDataFile::of_type("default_cards")
        .await?
        .load_stream()
        .await
}

/// An async Stream of every card object on Scryfall in every language.
pub async fn all_cards() -> crate::Result<impl Stream<Item = crate::Result<Card>>> {
    BulkDataFile::of_type("all_cards")
        .await?
        .load_stream()
        .await
}

/// An async Stream of all Rulings on Scryfall. Each ruling refers to cards via an
/// `oracle_id`.
pub async fn rulings() -> crate::Result<impl Stream<Item = crate::Result<Ruling>>> {
    BulkDataFile::of_type("rulings").await?.load_stream().await
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;

    #[tokio::test]
    #[ignore]
    async fn oracle_cards() {
        let mut stream = super::oracle_cards().await.unwrap();
        while let Some(card) = stream.next().await {
            card.unwrap();
        }
    }

    #[tokio::test]
    #[ignore]
    async fn unique_artwork() {
        let mut stream = super::unique_artwork().await.unwrap();
        while let Some(card) = stream.next().await {
            card.unwrap();
        }
    }

    #[tokio::test]
    #[ignore]
    async fn default_cards() {
        let mut stream = super::default_cards().await.unwrap();
        while let Some(card) = stream.next().await {
            card.unwrap();
        }
    }

    #[tokio::test]
    #[ignore]
    async fn all_cards() {
        let mut stream = super::all_cards().await.unwrap();
        while let Some(card) = stream.next().await {
            card.unwrap();
        }
    }

    #[tokio::test]
    #[ignore]
    async fn rulings() {
        let mut stream = super::rulings().await.unwrap();
        while let Some(card) = stream.next().await {
            card.unwrap();
        }
    }
}
