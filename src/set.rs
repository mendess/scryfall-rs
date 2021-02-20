//! A Set object represents a group of related Magic cards. All Card objects on
//! Scryfall belong to exactly one set.
//!
//! Due to Magic’s long and complicated history, Scryfall includes many
//! un-official sets as a way to group promotional or outlier cards together.
//! Such sets will likely have a code that begins with `p` or `t`, such as
//! `pcel` or `tori`.
//!
//! Official sets always have a three-letter set code, such as `zen`.
mod set_code;
mod set_type;

use chrono::NaiveDate;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};

pub use self::set_code::SetCode;
pub use self::set_type::SetType;
use crate::card::Card;
use crate::list::{List, ListIter};
use crate::uri::Uri;
use crate::util::SETS_URL;
use uuid::Uuid;

/// A Set object containing all fields that `scryfall` provides.
///
/// For documentation on each field please refer to their
/// [documentation](https://scryfall.com/docs/api/sets)
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct Set {
    pub id: Uuid,
    pub code: SetCode,
    pub mtgo_code: Option<String>,
    pub tcgplayer_id: Option<u64>,
    pub name: String,
    pub set_type: SetType,
    pub released_at: Option<NaiveDate>,
    pub block_code: Option<String>,
    pub block: Option<String>,
    pub parent_set_code: Option<String>,
    pub card_count: usize,
    pub digital: bool,
    pub foil_only: bool,
    pub scryfall_uri: String,
    pub uri: Uri<Set>,
    pub icon_svg_uri: String,
    pub search_uri: Uri<List<Card>>,
}

impl Set {
    /// Returns a [`ListIter`] of all the sets in the `scryfall` database.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::set::Set;
    /// let sets = Set::all().unwrap().into_inner().collect::<Vec<_>>();
    /// assert!(sets.len() > 0);
    /// ```
    pub fn all() -> crate::Result<ListIter<Set>> {
        let mut url = SETS_URL.clone();
        url.query_pairs_mut().append_pair("page", "1");
        Uri::from(url).fetch_iter()
    }

    /// Returns a `Set` with the given set code.
    ///
    /// The code can be either the `code` or the `mtgo_code` for the set.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::set::Set;
    /// assert_eq!(Set::code("mmq").unwrap().name, "Mercadian Masques")
    /// ```
    pub fn code(code: &str) -> crate::Result<Set> {
        Uri::from(SETS_URL.join(&percent_encode(code.as_bytes(), NON_ALPHANUMERIC).to_string())?)
            .fetch()
    }

    /// Returns a `Set` with the given `tcgplayer_id`.
    ///
    /// Also known as the `groupId` on [TCGplayer’s API](https://docs.tcgplayer.com/docs).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scryfall::set::Set;
    /// assert_eq!(Set::tcgplayer(1909).unwrap().name, "Amonkhet Invocations")
    /// ```
    pub fn tcgplayer<T: std::fmt::Display>(code: T) -> crate::Result<Set> {
        Uri::from(
            SETS_URL
                .join("tcgplayer/")?
                .join(&percent_encode(code.to_string().as_bytes(), NON_ALPHANUMERIC).to_string())?,
        )
        .fetch()
    }

    /// Returns a Set with the given Scryfall `uuid`.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::set::Set;
    /// assert_eq!(
    ///     Set::uuid("2ec77b94-6d47-4891-a480-5d0b4e5c9372".parse().unwrap())
    ///         .unwrap()
    ///         .name,
    ///     "Ultimate Masters"
    /// )
    /// ```
    pub fn uuid(uuid: Uuid) -> crate::Result<Set> {
        Uri::from(SETS_URL.join(&uuid.to_string())?).fetch()
    }

    /// Returns an iterator over the cards of the set.
    pub fn cards(&self) -> crate::Result<ListIter<Card>> {
        self.search_uri.fetch_iter()
    }
}
