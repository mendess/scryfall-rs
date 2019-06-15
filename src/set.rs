//! A Set object represents a group of related Magic cards. All Card objects on Scryfall belong to
//! exactly one set.
//!
//! Due to Magic’s long and complicated history, Scryfall includes many un-official sets as a way
//! to group promotional or outlier cards together. Such sets will likely have a four-letter code
//! that begins with p or t, such as pcel or tori.
//!
//! Official sets always have a three-letter set code, such as zen
mod set_type;

use super::card::Card;
use super::util::uri::{url_fetch, PaginatedURI, URI};
use super::util::UUID;
use super::util::{API, API_SETS};
use set_type::SetType;

use chrono::NaiveDate;
use serde::Deserialize;

/// A Set object containing all fields that `scryfall` provides.
///
/// For documentation on each field please refer to their
/// [documentation](https://scryfall.com/docs/api/sets)
#[derive(Debug, Deserialize, Clone)]
#[allow(missing_docs)]
pub struct Set {
    pub id: UUID,
    pub code: String,
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
    pub uri: URI<Set>,
    pub icon_svg_uri: String, //TODO: Revisit this
    pub search_uri: PaginatedURI<Card>,
}

impl Set {
    /// Returns a [`PaginatedURI`] of all the sets in the `scryfall` database.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::set::Set;
    /// match Set::all().next().unwrap() {
    ///     Ok(sets) => assert_ne!(sets.len(), 0),
    ///     Err(e) => eprintln!("{:?}", e)
    /// }
    /// ```
    pub fn all() -> PaginatedURI<Set> {
        let sets = format!("{}/{}?page=1", API, API_SETS);
        PaginatedURI::new(URI::from(sets))
    }

    /// Returns a [`Set`] with the given set code.
    ///
    /// The code can be either the `code` or the `mtgo_code` for the set.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::set::Set;
    /// assert_eq!(Set::code("mmq").unwrap().name, "Mercadian Masques")
    /// ```
    pub fn code(code: &str) -> crate::Result<Set> {
        url_fetch(&format!("{}/{}/{}", API, API_SETS, code))
    }

    /// Returns a [`Set`] with the given `tcgplayer_id`.
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
        url_fetch(&format!("{}/{}/tcgplayer/{}", API, API_SETS, code))
    }

    /// Returns a Set with the given Scryfall `uuid`.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::set::Set;
    /// assert_eq!(
    ///     Set::uuid("2ec77b94-6d47-4891-a480-5d0b4e5c9372".to_string()).unwrap().name,
    ///     "Ultimate Masters")
    /// ```
    pub fn uuid(uuid: UUID) -> crate::Result<Set> {
        url_fetch(&format!("{}/{}/{}", API, API_SETS, uuid))
    }
}
