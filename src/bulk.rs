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

use std::path::Path;

use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use futures::{Stream, TryStreamExt};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;
use uuid::Uuid;

cfg_if! {
    if #[cfg(feature = "bulk_caching")] {
        use tokio::io::BufReader;
        use futures::stream::BoxStream;
        use crate::uri;
    }
}

use crate::card::Card;
use crate::ruling::Ruling;
use crate::uri::{Jsonl, Uri};
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

    /// The time when this file was last updated.
    pub updated_at: DateTime<Utc>,

    /// The size of this file in integer bytes.
    pub compressed_size: Option<usize>,

    /// The URI that hosts this bulk as jsonl.gz for fetching.
    pub jsonl_download_uri: Uri<Jsonl<T>>,

    #[cfg(test)]
    #[serde(rename = "object")]
    _object: String,
}

impl<T: DeserializeOwned + 'static> BulkDataFile<T> {
    cfg_if! {
        if #[cfg(feature = "bulk_caching")] {
            /// The full temp path where this file will be downloaded with `load`. The
            /// file name has the form "&lt;type&gt;-&lt;date&gt;.json".
            fn cache_path(&self) -> std::path::PathBuf {
                use heck::ToKebabCase;
                std::env::temp_dir().join(format!(
                    "{}-{}.jsonl.gz",
                    self.bulk_type.to_kebab_case(),
                    self.updated_at.format("%Y%m%d%H%M%S"),
                ))
            }

            async fn get_stream(&self) -> crate::Result<impl Stream<Item = crate::Result<T>>> {
                let cache_path = self.cache_path();
                match tokio::fs::File::open(&cache_path).await {
                    Ok(cache) => Ok(Box::pin(uri::fetch_stream_jsonl(BufReader::new(cache))?) as BoxStream<crate::Result<T>>),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                        let mut file = tokio::fs::File::create(cache_path).await?;
                        let uri = self.jsonl_download_uri.clone();
                        tokio::spawn(async move {
                            let response = uri.fetch_raw().await.expect("failed to fetch bulk file");
                            let mut reader = BufReader::new(StreamReader::new(
                                response.bytes_stream().map_err(std::io::Error::other),
                            ));
                            tokio::io::copy(&mut reader, &mut file).await.expect("failed while downloading bulk file for caching")
                        });
                        Ok(Box::pin(self.jsonl_download_uri.fetch_stream_jsonl().await?))
                    }
                    Err(e) => Err(e.into()),
                }
            }
        } else {
            async fn get_stream(&self) -> crate::Result<impl Stream<Item = crate::Result<T>>> {
                self.jsonl_download_uri.fetch_stream_jsonl().await
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
        self.get_stream().await?.try_collect().await
    }

    /// Returns an async Stream over the objects from this bulk data download.
    ///
    /// Downloads and stores the file in the computer's temp folder if this
    /// version hasn't been downloaded yet. Otherwise uses the stored copy.
    pub async fn load_stream(&self) -> crate::Result<impl Stream<Item = crate::Result<T>>>
    where
        T: Send + 'static,
    {
        self.get_stream().await
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
        .jsonl_download_uri
        .fetch_stream_jsonl()
        .await
}

/// An async Stream of Scryfall card objects that together contain all unique
/// artworks. The chosen cards promote the best image scans.
pub async fn unique_artwork() -> crate::Result<impl Stream<Item = crate::Result<Card>>> {
    BulkDataFile::of_type("unique_artwork")
        .await?
        .jsonl_download_uri
        .fetch_stream_jsonl()
        .await
}

/// An async Stream containing every card object on Scryfall in English or the
/// printed language if the card is only available in one language.
pub async fn default_cards() -> crate::Result<impl Stream<Item = crate::Result<Card>>> {
    BulkDataFile::of_type("default_cards")
        .await?
        .jsonl_download_uri
        .fetch_stream_jsonl()
        .await
}

/// An async Stream of every card object on Scryfall in every language.
pub async fn all_cards() -> crate::Result<impl Stream<Item = crate::Result<Card>>> {
    BulkDataFile::of_type("all_cards")
        .await?
        .jsonl_download_uri
        .fetch_stream_jsonl()
        .await
}

/// An async Stream of all Rulings on Scryfall. Each ruling refers to cards via an
/// `oracle_id`.
pub async fn rulings() -> crate::Result<impl Stream<Item = crate::Result<Ruling>>> {
    BulkDataFile::of_type("rulings")
        .await?
        .jsonl_download_uri
        .fetch_stream_jsonl()
        .await
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;

    use std::pin::pin;

    #[tokio::test]
    #[ignore]
    async fn oracle_cards() {
        let mut stream = pin!(super::oracle_cards().await.unwrap());
        let mut i = 0;
        while let Some(card) = stream.next().await {
            card.unwrap();
            i += 1;
        }
        if i == 0 {
            panic!("empty stream");
        }
    }

    #[tokio::test]
    #[ignore]
    async fn unique_artwork() {
        let mut stream = pin!(super::unique_artwork().await.unwrap());
        while let Some(card) = stream.next().await {
            card.unwrap();
        }
    }

    #[tokio::test]
    #[ignore]
    async fn default_cards() {
        let mut stream = pin!(super::default_cards().await.unwrap());
        while let Some(card) = stream.next().await {
            card.unwrap();
        }
    }

    #[tokio::test]
    #[ignore]
    async fn all_cards() {
        let mut stream = pin!(super::all_cards().await.unwrap());
        while let Some(card) = stream.next().await {
            card.unwrap();
        }
    }

    #[tokio::test]
    #[ignore]
    async fn rulings() {
        let mut stream = pin!(super::rulings().await.unwrap());
        while let Some(card) = stream.next().await {
            card.unwrap();
        }
    }
}
