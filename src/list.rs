//! A [`List`] object represents a requested sequence of other objects (Cards,
//! Sets, etc). List objects may be paginated, and also include information
//! about issues raised when generating the list.
//!
//! This module also defines [`ListIter`], which can iterate over the contents
//! of a `List`. If the list is paginated, the `ListIter` will request each page
//! lazily.

use std::vec;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::uri::Uri;

/// A list object.
///
/// For documentation on its fields refer to the [list object](https://scryfall.com/docs/api/lists)
/// on the official site.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
#[allow(missing_docs)]
pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub next_page: Option<Uri<List<T>>>,
    pub total_cards: Option<usize>,
    pub warnings: Option<Vec<String>>,
}

impl<T: DeserializeOwned> List<T> {
    /// Creates an iterator over all the pages of this list.
    pub fn into_page_iter(self) -> PageIter<T> {
        PageIter {
            curr: Some(self),
            page_num: 1,
        }
    }
}

impl<T: DeserializeOwned> IntoIterator for List<T> {
    type IntoIter = ListIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        // `has_more` is assumed to be redundant.
        debug_assert!(self.has_more == self.next_page.is_some());

        ListIter {
            inner: self.data.into_iter(),
            next_uri: self.next_page,
            page_num: 1,
            total: self.total_cards,
            remaining: self.total_cards,
        }
    }
}

/// An iterator that moves objects out of a list.
///
/// This struct is created by the `into_iter` method on `List`.
///
/// Upon reaching the end of a page, further pages will be requested and the
/// iterator will continue yielding items from those pages. Therefore the
/// associated `Item` type must be a [`crate::Result`]`<T>` as this request can
/// fail.
#[derive(Debug, Clone)]
pub struct ListIter<T> {
    inner: vec::IntoIter<T>,
    next_uri: Option<Uri<List<T>>>,
    page_num: usize,
    total: Option<usize>,
    remaining: Option<usize>,
}

impl<T> ListIter<T> {
    /// Extracts the inner [`vec::IntoIter`] that holds this page of data. The
    /// resulting iterator has items of type `T` instead of
    /// `crate::Result<T>`.
    ///
    /// # Examples
    /// ```rust
    /// # use scryfall::Card;
    /// let card_names = Card::search("stormcrow")
    ///     .unwrap()
    ///     .into_inner()
    ///     .map(|c| c.name)
    ///     .collect::<Vec<_>>();
    /// assert_eq!(card_names, ["Mindstorm Crown", "Storm Crow"]);
    /// ```
    pub fn into_inner(self) -> vec::IntoIter<T> {
        self.inner
    }
}

impl<T: DeserializeOwned> ListIter<T> {
    /// Gets a `ListIter` for the next page of objects by requesting it from the
    /// API.
    ///
    /// # Example
    /// ```rust
    /// # use scryfall::Set;
    /// let page_1 = Set::code("inn").unwrap().cards().unwrap();
    /// let mut page_2 = page_1.next_page().unwrap().unwrap();
    /// assert_eq!(
    ///     page_2
    ///         .next()
    ///         .unwrap()
    ///         .collector_number
    ///         .parse::<usize>()
    ///         .unwrap(),
    ///     page_1.into_inner().len() + 1
    /// );
    /// ```
    pub fn next_page(&self) -> crate::Result<Option<Self>> {
        if let Some(uri) = self.next_uri.as_ref() {
            let mut new_iter = uri.fetch_iter()?;
            new_iter.remaining = self.remaining.map(|r| r - self.inner.len());
            new_iter.page_num = self.page_num + 1;

            // The new total should be the same as the old total.
            debug_assert_eq!(self.total, new_iter.total);

            Ok(Some(new_iter))
        } else {
            Ok(None)
        }
    }
}

impl<T: DeserializeOwned> Iterator for ListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(next) => {
                self.remaining = self.remaining.map(|r| r - 1);
                Some(next)
            },
            None => match self.next_page() {
                Ok(Some(new_iter)) => {
                    *self = new_iter;
                    self.next()
                },
                Ok(None) => None,
                Err(e) => {
                    eprintln!("Error retrieving page {} - {}", self.page_num + 1, e);
                    self.next_uri = None;
                    self.remaining = Some(0);
                    None
                },
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if let Some(len) = self.remaining {
            (len, Some(len))
        } else {
            let len = self.inner.len();
            (
                len,
                if self.next_uri.is_some() {
                    None
                } else {
                    Some(len)
                },
            )
        }
    }
}

/// An iterator over the pages of a list. Before returning each page, the next
/// page is requested.
pub struct PageIter<T> {
    curr: Option<List<T>>,
    page_num: usize,
}

impl<T: DeserializeOwned> Iterator for PageIter<T> {
    type Item = List<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.curr.take() {
            self.curr = match &curr.next_page {
                Some(uri) => match uri.fetch() {
                    Ok(page) => {
                        self.page_num += 1;
                        Some(page)
                    },
                    Err(e) => {
                        eprintln!("Error fetching page {} - {}", self.page_num + 1, e);
                        None
                    },
                },
                None => None,
            };
            Some(curr)
        } else {
            None
        }
    }
}
