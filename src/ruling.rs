//! Rulings represent Oracle rulings, Wizards of the Coast set release notes, or Scryfall notes for
//! a particular card.
//!
//! If two cards have the same name, they will have the same set of rulings objects. If a card has
//! rulings, it usually has more than one.
//!
//! Rulings with a `Scryfall` source have been added by the Scryfall team, either to provide
//! additional context for the card, or explain how the card works in an unofficial format (such as
//! Duel Commander).

use crate::util::uri::{PaginatedURI, URI};
use crate::util::{Uuid, API, API_CARDS, API_RULING};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A ruling object.
///
/// For documentation on it's fields refer to the
/// [ruling object](https://scryfall.com/docs/api/rulings) on the official site.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
#[allow(missing_docs)]
pub struct Ruling {
    pub oracle_id: Uuid,
    pub source: Source,
    pub published_at: NaiveDate,
    pub comment: String,
}

/// The two possible ruling sources
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Source {
    Wotc,
    Scryfall,
}

impl Ruling {
    /// Returns a List of rulings for a card with the given Multiverse ID. If the card has multiple
    /// multiverse IDs, this method can find either of them.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::ruling::Ruling;
    /// match Ruling::multiverse_id(3255).next() {
    ///     Some(rulings) => assert_eq!(
    ///         rulings
    ///             .unwrap()
    ///             .iter()
    ///             .filter(|r| r.comment == "The ability is a mana ability, so it is activated and resolves as a mana ability, but it can only be activated at times when you can cast an instant. Yes, this is a bit weird.")
    ///             .count(),
    ///         1
    ///     ),
    ///     None => panic!("Nothing")
    /// }
    /// ```
    pub fn multiverse_id(id: usize) -> PaginatedURI<Self> {
        PaginatedURI::new(URI::from(format!(
            "{}/{}/multiverse/{}/{}",
            API, API_CARDS, id, API_RULING
        )))
    }

    /// Returns rulings for a card with the given MTGO ID (also known as the Catalog ID). The ID
    /// can either be the card’s `mtgo_id` or its `mtgo_foil_id`.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::ruling::Ruling;
    /// match Ruling::mtgo_id(57934).next() {
    ///     Some(rulings) => assert_eq!(
    ///         rulings
    ///             .unwrap()
    ///             .iter()
    ///             .filter(|r| r.comment == "Yes, if the fourth mode is the only one remaining, you must choose it. You read the whole contract, right?")
    ///             .count(),
    ///         1
    ///     ),
    ///     None => panic!(),
    /// }
    /// ```
    pub fn mtgo_id(id: usize) -> PaginatedURI<Self> {
        PaginatedURI::new(URI::from(format!(
            "{}/{}/mtgo/{}/{}",
            API, API_CARDS, id, API_RULING
        )))
    }

    /// Returns rulings for a card with the given Magic: The Gathering Arena ID.
    ///
    /// ```rust
    /// use scryfall::ruling::Ruling;
    /// match Ruling::arena_id(67462).next() {
    ///     Some(rulings) => assert_eq!(
    ///         rulings
    ///             .unwrap()
    ///             .iter()
    ///             .filter(|r| r.comment == "Once a chapter ability has triggered, the ability on the stack won’t be affected if the Saga gains or loses counters, or if it leaves the battlefield.")
    ///             .count(),
    ///         1
    ///     ),
    ///     None => panic!(),
    /// }
    /// ```
    pub fn arena_id(id: usize) -> PaginatedURI<Self> {
        PaginatedURI::new(URI::from(format!(
            "{}/{}/arena/{}/{}",
            API, API_CARDS, id, API_RULING
        )))
    }

    /// Returns a List of rulings for the card with the given set code and collector number.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::ruling::Ruling;
    /// match Ruling::set_and_number("bfz", 17).next() {
    ///     Some(rulings) => assert_eq!(
    ///         rulings
    ///             .unwrap()
    ///             .iter()
    ///             .filter(|r| r.comment == "Yes, your opponent can’t even. We know.")
    ///             .count(),
    ///         1
    ///     ),
    ///     None => panic!(),
    /// }
    /// ```
    pub fn set_and_number(set: &str, number: u32) -> PaginatedURI<Self> {
        PaginatedURI::new(URI::from(format!(
            "{}/{}/{}/{}/{}",
            API, API_CARDS, set, number, API_RULING
        )))
    }

    /// Returns a List of rulings for a card with the given Scryfall ID.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::ruling::Ruling;
    /// match Ruling::uuid("f2b9983e-20d4-4d12-9e2c-ec6d9a345787".to_string()).next() {
    ///     Some(rulings) => assert_eq!(
    ///         rulings
    ///             .unwrap()
    ///             .iter()
    ///             .filter(|r| r.comment == "It must flip like a coin and not like a Frisbee.")
    ///             .count(),
    ///         1
    ///     ),
    ///     None => panic!(),
    /// }
    /// ```
    pub fn uuid(id: Uuid) -> PaginatedURI<Self> {
        PaginatedURI::new(URI::from(format!(
            "{}/{}/{}/{}",
            API, API_CARDS, id, API_RULING
        )))
    }
}
