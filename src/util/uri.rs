//! Module for handling unresolved URLs returned by the scryfall api
//!
//! Some fields of the scryfall api have URLs refering to queries that can be run to obtain more
//! information. This module abstracts the work of fetching that data.
use crate::error::Error;
use std::marker::PhantomData;

use serde::Deserialize;
use serde_json::from_reader;

/// A URI that will fetch something of a defined type `T`.
#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct URI<T>(String, PhantomData<T>);

impl<T> From<String> for URI<T> {
    fn from(s: String) -> Self {
        URI(s, PhantomData)
    }
}

impl<T> From<URI<T>> for String {
    fn from(s: URI<T>) -> Self {
        s.0
    }
}

impl<T> URI<T> {
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
    pub fn fetch(&self) -> crate::Result<T>
    where
        for<'de> T: Deserialize<'de>,
    {
        url_fetch(&self.0)
    }
}

/// A paginating URL fetcher.
///
/// Sometimes the data pointed to by a URL is paginated. In that case a `PaginatedURI` is needed to
/// iterate over the pages of data.
#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct PaginatedURI<T> {
    next: Option<URI<JsonParser<T>>>,
}

impl<T> PaginatedURI<T>
where
    for<'de> T: Deserialize<'de>,
{
    /// Creates a new `PaginatedURI` iterator from a `URI` of type `T`.
    pub fn new(url: URI<T>) -> Self {
        PaginatedURI {
            next: Some(URI(url.0, PhantomData)),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct JsonParser<T> {
    next_page: Option<URI<JsonParser<T>>>,
    data: Vec<T>,
}

impl<T> Iterator for PaginatedURI<T>
where
    for<'de> T: Deserialize<'de>,
{
    type Item = crate::Result<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(url) = self.next.take() {
            match url_fetch::<JsonParser<T>>(&String::from(url)) {
                Ok(cards) => {
                    *self = PaginatedURI {
                        next: cards.next_page,
                    };
                    Some(Ok(cards.data))
                }
                Err(error) => Some(Err(error)),
            }
        } else {
            None
        }
    }
}

/// Utility function to fetch data pointed to by a URL string.
///
/// # Examples
/// ```rust
/// use scryfall::{util::uri::url_fetch, card::Card};
/// assert_eq!(
///     url_fetch::<Card>("https://api.scryfall.com/cards/arena/67330")
///         .unwrap()
///         .name,
///     Card::arena(67330).unwrap().name)
/// ```
pub fn url_fetch<T>(url: &str) -> crate::Result<T>
where
    for<'de> T: Deserialize<'de>,
{
    let resp = reqwest::get(url)?;
    if resp.status().is_success() {
        Ok(from_reader(resp)?)
    } else {
        Err(Error::Other(format!("{:?}", resp.status())))
    }
}
