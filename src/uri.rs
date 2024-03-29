//! Module for handling unresolved URLs returned by the scryfall api
//!
//! Some fields of the scryfall api have URLs referring to queries that can be
//! run to obtain more information. This module abstracts the work of fetching
//! that data.
use std::convert::TryFrom;
use std::marker::PhantomData;

use httpstatus::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::error::Error;
use crate::list::{List, ListIter};

/// An unresolved URI returned by the Scryfall API, or generated by this crate.
///
/// The `fetch` method handles requesting the resource from the API endpoint,
/// and deserializing it into a `T` object. If the type parameter is
/// [`List`][crate::list::List]`<_>`, then additional methods `fetch_iter`
/// and `fetch_all` are available, giving access to objects from all pages
/// of the collection.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(transparent)]
pub struct Uri<T> {
    url: Url,
    _marker: PhantomData<fn() -> T>,
}

impl<T> TryFrom<&str> for Uri<T> {
    type Error = crate::error::Error;

    fn try_from(url: &str) -> Result<Self, Self::Error> {
        Ok(Uri::from(Url::parse(url)?))
    }
}

impl<T> From<Url> for Uri<T> {
    fn from(url: Url) -> Self {
        Uri {
            url,
            _marker: PhantomData,
        }
    }
}

impl<T: DeserializeOwned> Uri<T> {
    /// Fetches a resource from the Scryfall API and deserializes it into a type
    /// `T`.
    ///
    /// # Example
    /// ```rust
    /// # use std::convert::TryFrom;
    /// #
    /// # use scryfall::card::Card;
    /// # use scryfall::uri::Uri;
    /// # tokio_test::block_on(async {
    /// let uri =
    ///     Uri::<Card>::try_from("https://api.scryfall.com/cards/named?exact=Lightning+Bolt").unwrap();
    /// let bolt = uri.fetch().await.unwrap();
    /// assert_eq!(bolt.mana_cost, Some("{R}".to_string()));
    /// # })
    /// ```
    pub async fn fetch(&self) -> crate::Result<T> {
        match self.fetch_raw().await {
            Ok(response) => match response.status().as_u16() {
                200..=299 => Ok(response.json().await?),
                status => Err(Error::HttpError(StatusCode::from(status))),
            },
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn fetch_raw(&self) -> crate::Result<reqwest::Response> {
        match reqwest::get(self.url.clone()).await {
            Ok(response) => match response.status().as_u16() {
                400..=599 => Err(Error::ScryfallError(response.json().await?)),
                _ => Ok(response),
            },
            Err(e) => Err(Error::ReqwestError(e.into(), self.url.to_string())),
        }
    }
}

impl<T: DeserializeOwned + Send + Sync + Unpin> Uri<List<T>> {
    /// Lazily iterate over items from all pages of a list. Following pages are
    /// requested once the previous page has been exhausted.
    ///
    /// # Example
    /// ```rust
    /// # use std::convert::TryFrom;
    /// #
    /// # use scryfall::Card;
    /// # use scryfall::list::List;
    /// # use scryfall::uri::Uri;
    /// use futures::stream::StreamExt;
    /// use futures::future;
    /// # tokio_test::block_on(async {
    /// let uri = Uri::<List<Card>>::try_from("https://api.scryfall.com/cards/search?q=zurgo").unwrap();
    /// assert!(
    ///     uri.fetch_iter()
    ///         .await
    ///         .unwrap()
    ///         .into_stream()
    ///         .map(Result::unwrap)
    ///         .filter(|c| future::ready(c.name.contains("Bellstriker")))
    ///         .collect::<Vec<_>>()
    ///         .await
    ///         .len()
    ///          > 0
    /// );
    /// # })
    /// ```
    ///
    /// ```rust
    /// # use std::convert::TryFrom;
    /// #
    /// # use scryfall::Card;
    /// # use scryfall::list::List;
    /// # use scryfall::uri::Uri;
    /// use futures::stream::StreamExt;
    /// use futures::future;
    /// # tokio_test::block_on(async {
    /// let uri = Uri::<List<Card>>::try_from("https://api.scryfall.com/cards/search?q=zurgo").unwrap();
    /// assert!(
    ///     uri.fetch_iter()
    ///         .await
    ///         .unwrap()
    ///         .into_stream_buffered(10)
    ///         .map(Result::unwrap)
    ///         .filter(|c| future::ready(c.name.contains("Bellstriker")))
    ///         .collect::<Vec<_>>()
    ///         .await
    ///         .len()
    ///          > 0
    /// );
    /// # })
    /// ```
    pub async fn fetch_iter(&self) -> crate::Result<ListIter<T>> {
        Ok(self.fetch().await?.into_list_iter())
    }

    /// Eagerly fetch items from all pages of a list. If any of the pages fail
    /// to load, returns an error.
    ///
    /// # Example
    /// ```rust
    /// # use std::convert::TryFrom;
    /// #
    /// # use scryfall::Card;
    /// # use scryfall::list::List;
    /// # use scryfall::uri::Uri;
    /// # tokio_test::block_on(async {
    /// let uri =
    ///     Uri::<List<Card>>::try_from("https://api.scryfall.com/cards/search?q=e:ddu&unique=prints")
    ///         .unwrap();
    /// assert_eq!(uri.fetch_all().await.unwrap().len(), 76);
    /// # })
    /// ```
    pub async fn fetch_all(&self) -> crate::Result<Vec<T>> {
        let mut items = vec![];
        let mut next_page = Some(self.fetch().await?);
        while let Some(page) = next_page {
            items.extend(page.data.into_iter());
            next_page = match page.next_page {
                Some(uri) => Some(uri.fetch().await?),
                None => None,
            };
        }
        Ok(items)
    }
}
