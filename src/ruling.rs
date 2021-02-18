//! Rulings represent Oracle rulings, Wizards of the Coast set release notes, or
//! Scryfall notes for a particular card.
//!
//! If two cards have the same name, they will have the same set of rulings
//! objects. If a card has rulings, it usually has more than one.
//!
//! Rulings with a `Scryfall` source have been added by the Scryfall team,
//! either to provide additional context for the card, or explain how the card
//! works in an unofficial format (such as Duel Commander).

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::list::ListIter;
use crate::uri::Uri;
use crate::util::{Uuid, API_RULING, CARDS_URL};

/// A ruling object.
///
/// For documentation on its fields refer to the
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
    /// Returns a List of rulings for a card with the given Multiverse ID. If
    /// the card has multiple multiverse IDs, this method can find either of
    /// them.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::ruling::Ruling;
    /// assert!(Ruling::multiverse_id(3255).unwrap()
    /// .filter_map(Result::ok).any(|r| r.comment == "The ability is a mana ability, so it is activated and resolves as a mana ability, but it can only be activated at times when you can cast an instant. Yes, this is a bit weird."));
    /// ```
    pub fn multiverse_id(id: usize) -> crate::Result<ListIter<Self>> {
        Uri::from(
            CARDS_URL
                .join("multiverse/")?
                .join(&format!("{}/", id))?
                .join(API_RULING)?,
        )
        .fetch_iter()
    }

    /// Returns rulings for a card with the given MTGO ID (also known as the
    /// Catalog ID). The ID can either be the card’s `mtgo_id` or its
    /// `mtgo_foil_id`.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::ruling::Ruling;
    /// assert!(Ruling::mtgo_id(57934).unwrap().filter_map(Result::ok).any(|r| r.comment == "Yes, if the fourth mode is the only one remaining, you must choose it. You read the whole contract, right?"));
    /// ```
    pub fn mtgo_id(id: usize) -> crate::Result<ListIter<Self>> {
        Uri::from(
            CARDS_URL
                .join("mtgo/")?
                .join(&format!("{}/", id))?
                .join(API_RULING)?,
        )
        .fetch_iter()
    }

    /// Returns rulings for a card with the given Magic: The Gathering Arena ID.
    ///
    /// ```rust
    /// use scryfall::ruling::Ruling;
    /// assert!(Ruling::arena_id(67462).unwrap().filter_map(Result::ok).any(|r| r.comment == "Once a chapter ability has triggered, the ability on the stack won’t be affected if the Saga gains or loses counters, or if it leaves the battlefield."));
    /// ```
    pub fn arena_id(id: usize) -> crate::Result<ListIter<Self>> {
        Uri::from(
            CARDS_URL
                .join("arena/")?
                .join(&format!("{}/", id))?
                .join(API_RULING)?,
        )
        .fetch_iter()
    }

    /// Returns a List of rulings for the card with the given set code and
    /// collector number.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::ruling::Ruling;
    /// assert!(
    ///     Ruling::set_and_number("bfz", 17)
    ///         .unwrap()
    ///         .filter_map(Result::ok)
    ///         .any(|r| r.comment == "Yes, your opponent can’t even. We know.")
    /// );
    /// ```
    pub fn set_and_number(set: &str, number: u32) -> crate::Result<ListIter<Self>> {
        Uri::from(
            CARDS_URL
                .join(&format!("{}/{}/", set, number))?
                .join(API_RULING)?,
        )
        .fetch_iter()
    }

    /// Returns a List of rulings for a card with the given Scryfall ID.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::ruling::Ruling;
    /// assert!(
    ///     Ruling::uuid("f2b9983e-20d4-4d12-9e2c-ec6d9a345787".parse().unwrap())
    ///         .unwrap()
    ///         .filter_map(Result::ok)
    ///         .any(|r| r.comment == "It must flip like a coin and not like a Frisbee.")
    /// );
    /// ```
    pub fn uuid(id: Uuid) -> crate::Result<ListIter<Self>> {
        Uri::from(CARDS_URL.join(&format!("{}/", id))?.join(API_RULING)?).fetch_iter()
    }
}
