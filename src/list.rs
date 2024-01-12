//! A [`List`] object represents a requested sequence of other objects (Cards,
//! Sets, etc). List objects may be paginated, and also include information
//! about issues raised when generating the list.
//!
//! This module also defines [`ListIter`], which can iterate over the contents
//! of a `List`. If the list is paginated, the `ListIter` will request each page
//! lazily.

use futures::{future, stream, Future, Stream, StreamExt};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::vec;

use crate::uri::Uri;

/// A List object represents a requested sequence of other objects (Cards, Sets,
/// etc). List objects may be paginated, and also include information about
/// issues raised when generating the list.
///
/// ---
///
/// For more information, visit the [official docs](https://scryfall.com/docs/api/lists).
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct List<T> {
    /// An array of the requested objects, in a specific order.
    pub data: Vec<T>,

    /// True if this List is paginated and there is a page beyond the current
    /// page.
    pub has_more: bool,

    /// If there is a page beyond the current page, this field will contain a
    /// full API URI to that page. You may submit a HTTP GET request to that URI
    /// to continue paginating forward on this List.
    pub next_page: Option<Uri<List<T>>>,

    /// If this is a list of Card objects, this field will contain the total
    /// number of cards found across all pages.
    pub total_cards: Option<usize>,

    /// An array of human-readable warnings issued when generating this list, as
    /// strings. Warnings are non-fatal issues that the API discovered with your
    /// input. In general, they indicate that the List will not contain the all
    /// of the information you requested. You should fix the warnings and
    /// re-submit your request.
    pub warnings: Option<Vec<String>>,
}

impl<T: DeserializeOwned + Send + Sync + Unpin> List<T> {
    /// Creates an iterator over all the pages of this list.
    pub fn into_page_iter(self) -> PageIter<T> {
        PageIter {
            curr: Some(self),
            page_num: 1,
        }
    }

    /// Creates a ListIter from a List
    pub fn into_list_iter(self) -> ListIter<T> {
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
/// iterator will continue yielding items from those pages. As a consequence,
/// the `Item` type of this iterator is a `Result` in case those calls fail.
#[derive(Debug, Clone)]
pub struct ListIter<T> {
    inner: vec::IntoIter<T>,
    next_uri: Option<Uri<List<T>>>,
    page_num: usize,
    total: Option<usize>,
    remaining: Option<usize>,
}

impl<T: DeserializeOwned + Send + Sync + Unpin + 'static> ListIter<T> {
    /// Gets a `ListIter` for the next page of objects by requesting it from the
    /// API.
    ///
    /// # Example
    /// ```rust
    /// # use scryfall::Set;
    ///  # tokio_test::block_on(async {
    /// let page_1 = Set::code("inn").await.unwrap().cards().await.unwrap();
    /// let mut page_2 = Box::new(page_1.next_page().await.unwrap().unwrap());
    /// assert_eq!(
    ///     page_2
    ///         .next()
    ///         .await
    ///         .unwrap()
    ///         .unwrap()
    ///         .collector_number
    ///         .parse::<usize>()
    ///         .unwrap(),
    ///     page_1.into_inner().len() + 1
    /// );
    /// # })
    /// ```
    pub async fn next_page(&self) -> crate::Result<Option<Self>> {
        if let Some(uri) = self.next_uri.as_ref() {
            let mut new_iter = uri.fetch_iter().await?;
            new_iter.remaining = self.remaining.map(|r| r - self.inner.len());
            new_iter.page_num = self.page_num + 1;

            // The new total should be the same as the old total.
            debug_assert_eq!(self.total, new_iter.total);

            Ok(Some(new_iter))
        } else {
            Ok(None)
        }
    }

    /// Asynchronously returns next element of the stream
    /// Will automatically handle pagination
    /// Returns None if the Stream is exausted, Result otherwise
    pub async fn next(&mut self) -> Option<crate::Result<T>> {
        match self.inner.next() {
            Some(next) => {
                self.remaining = self.remaining.map(|r| r - 1);
                Some(Ok(next))
            },
            None => match self.next_page().await {
                Ok(Some(new_iter)) => {
                    *self = new_iter;
                    match self.inner.next() {
                        Some(next) => {
                            self.remaining = self.remaining.map(|r| r - 1);
                            Some(Ok(next))
                        },
                        None => None,
                    }
                },
                Ok(None) => None,
                Err(e) => {
                    self.next_uri = None;
                    self.remaining = Some(0);
                    Some(Err(e))
                },
            },
        }
    }

    async fn stream_next(&mut self) -> Option<impl Future<Output = crate::Result<T>>> {
        match self.inner.next() {
            Some(next) => {
                self.remaining = self.remaining.map(|r| r - 1);
                Some(future::ready(Ok(next)))
            },
            None => match self.next_page().await {
                Ok(Some(new_iter)) => {
                    *self = new_iter;
                    match self.inner.next() {
                        Some(next) => {
                            self.remaining = self.remaining.map(|r| r - 1);
                            Some(future::ready(Ok(next)))
                        },
                        None => None,
                    }
                },
                Ok(None) => None,
                Err(e) => {
                    self.next_uri = None;
                    self.remaining = Some(0);
                    Some(future::ready(Err(e)))
                },
            },
        }
    }

    /// Creates a Stream from a ListIter
    pub fn into_stream(self) -> impl Stream<Item = crate::Result<T>> + Unpin {
        Box::pin(stream::unfold(self, |mut state| async move {
            let item = state.stream_next().await;
            if let Some(val) = item {
                Some((val.await, state))
            } else {
                None
            }
        }))
    }

    /// Creates a Stream from a ListIter that is buffered by n items
    pub fn into_stream_buffered(
        self,
        buf_factor: usize,
    ) -> impl Stream<Item = crate::Result<T>> + Unpin {
        Box::pin(
            stream::unfold(self, |mut state| async move {
                let item = state.stream_next().await;
                item.map(|val| (val, state))
            })
            .buffered(buf_factor),
        )
    }

    /// Creates a Stream from a ListIter that is buffered by n items in a non-deterministic order
    pub fn into_stream_buffered_unordered(
        self,
        buf_factor: usize,
    ) -> impl Stream<Item = crate::Result<T>> + Unpin {
        Box::pin(
            stream::unfold(self, |mut state| async move {
                let item = state.stream_next().await;
                item.map(|val| (val, state))
            })
            .buffer_unordered(buf_factor),
        )
    }

    /// Returns approximate size of Listiter
    pub fn size_hint(&self) -> (usize, Option<usize>) {
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
    /// Extracts the inner [`vec::IntoIter`] that holds this page of data.
    /// Further pages will not be fetched when it gets to the end.
    ///
    /// # Examples
    /// ```rust
    /// # use scryfall::Card;
    /// # tokio_test::block_on(async {
    /// let card_names = Card::search("stormcrow").await
    ///     .unwrap()
    ///     .into_inner()
    ///     .map(|c| c.name)
    ///     .collect::<Vec<_>>();
    /// assert_eq!(card_names, ["Mindstorm Crown", "Storm Crow"]);
    /// # })
    /// ```
    pub fn into_inner(self) -> vec::IntoIter<T> {
        self.inner
    }
}

/// An iterator over the pages of a list. Before returning each page, the next
/// page is requested.
pub struct PageIter<T> {
    curr: Option<List<T>>,
    page_num: usize,
}

impl<T: DeserializeOwned + Send + Sync + Unpin> PageIter<T> {
    async fn stream_next(&mut self) -> Option<impl Future<Output = List<T>>> {
        if let Some(curr) = self.curr.take() {
            self.curr = match &curr.next_page {
                Some(uri) => match uri.fetch().await {
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
            Some(future::ready(curr))
        } else {
            None
        }
    }

    /// Creates a Stream from a PageIter
    pub fn into_stream(self) -> impl Stream<Item = List<T>> + Unpin {
        Box::pin(stream::unfold(self, |mut state| async move {
            if let Some(val) = state.stream_next().await {
                Some((val.await, state))
            } else {
                None
            }
        }))
    }

    /// Creates a Stream from a PageIter
    pub fn into_stream_buffered(self, buf_factor: usize) -> impl Stream<Item = List<T>> + Unpin {
        Box::pin(
            stream::unfold(self, |mut state| async move {
                state.stream_next().await.map(|val| (val, state))
            })
            .buffered(buf_factor),
        )
    }

    /// Creates a Stream from a PageIter
    pub fn into_stream_buffered_unordered(
        self,
        buf_factor: usize,
    ) -> impl Stream<Item = List<T>> + Unpin {
        Box::pin(
            stream::unfold(self, |mut state| async move {
                state.stream_next().await.map(|val| (val, state))
            })
            .buffer_unordered(buf_factor),
        )
    }
}
