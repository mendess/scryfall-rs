//! Module for handling unresolved URLs returned by the scryfall api
//!
//! Some fields of the scryfall api have URLs refering to queries that can be run to obtain more
//! information. This module abstracts the work of fetching that data.
use crate::error::Error;

use std::marker::PhantomData;

use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A URI that will fetch something of a defined type `T`.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct URI<T>(String, PhantomData<T>);

impl<T: DeserializeOwned> From<String> for URI<T> {
    fn from(s: String) -> Self {
        URI(s, PhantomData)
    }
}

impl<T: DeserializeOwned> URI<T> {
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl<T: DeserializeOwned> AsRef<str> for URI<T> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<T: DeserializeOwned> URI<T> {
    /// Fetch the object of type `T` that this `URL` is pointing to.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::{util::uri::URI, card::Card};
    /// assert_eq!(
    ///     URI::<Card>::from("https://api.scryfall.com/cards/arena/67330".to_string())
    ///         .fetch()
    ///         .unwrap()
    ///         .name,
    ///     Card::arena(67330).unwrap().name)
    /// ```
    pub fn fetch(&self) -> crate::Result<T> {
        url_fetch(&self.0)
    }
}

/// A paginating URL fetcher.
///
/// Sometimes the data pointed to by a URL is paginated. In that case a
/// `PaginatedURI` is needed to iterate over the pages of data.
///
/// When iterating over one, every call to next returns a either a vector of `T` or
/// an error.
/// If an `Err` is returned then subsequent calls to `next()` will return `None`,
/// since the error is likely to repeat after that point.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[serde(transparent)]
pub struct PaginatedURI<T> {
    next: Option<URI<JsonParser<T>>>,
}

impl<T: DeserializeOwned> PaginatedURI<T> {
    /// Creates a new `PaginatedURI` iterator from a `URI` of type `T`.
    pub fn new(url: URI<T>) -> Self {
        PaginatedURI {
            next: Some(URI(url.0, PhantomData)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct JsonParser<T> {
    next_page: Option<URI<JsonParser<T>>>,
    data: Vec<T>,
}

impl<T: DeserializeOwned> Iterator for PaginatedURI<T> {
    type Item = crate::Result<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|url| {
            url_fetch(url).map(|cards: JsonParser<T>| {
                self.next = cards.next_page;
                cards.data
            })
        })
    }
}

// Reusable client instance to optimize multiple network calls. Used only in `url_fetch`
// thread_local!(static CLIENT: Client = Client::new());

/// Utility function to fetch data pointed to by a URL string.
///
/// # Examples
/// ```rust
/// use scryfall::{util::uri::url_fetch, card::Card};
/// assert_eq!(
///     url_fetch::<Card,_>("https://api.scryfall.com/cards/arena/67330")
///         .unwrap()
///         .name,
///     Card::arena(67330).unwrap().name)
/// ```
pub fn url_fetch<T: DeserializeOwned, I: AsRef<str>>(url: I) -> crate::Result<T> {
    thread_local!(static CLIENT: Client = Client::new());
    let resp = CLIENT.with(|c| c.get(url.as_ref()).send())?;
    if resp.status().is_success() {
        Ok(serde_json::from_reader(resp)?)
    } else if resp.status().is_client_error() {
        Err(Error::ScryfallError(serde_json::from_reader(resp)?))
    } else {
        Err(format!("{:?}", resp.status()))?
    }
}
