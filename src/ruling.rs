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
use uuid::Uuid;

use crate::list::ListIter;
use crate::uri::Uri;
use crate::util::{API_RULING, CARDS_URL};

/// Rulings represent Oracle rulings, Wizards of the Coast set release notes, or
/// Scryfall notes for a particular card.
// If two cards have the same name, they will have the same set of rulings objects. If a card has
// rulings, it usually has more than one.
//
// Rulings with a scryfall source have been added by the Scryfall team, either to provide additional
// context for the card, or explain how the card works in an unofficial format (such as Duel
// Commander).
/// ---
///
/// For more information, refer to the [official docs](https://scryfall.com/docs/api/rulings).
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Ruling {
    /// A unique ID for the oracle identity of the card this ruling is about.
    /// This value is consistent across reprinted card editions, and unique
    /// among different cards with the same name (tokens, Unstable variants,
    /// etc).
    pub oracle_id: Uuid,

    /// A computer-readable string indicating which company produced this
    /// ruling, either wotc or scryfall.
    pub source: Source,

    /// The date when the ruling or note was published.
    pub published_at: NaiveDate,

    /// The text of the ruling.
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
    /// assert!(
    ///     Ruling::multiverse_id(3255)
    ///         .unwrap()
    ///         .map(Result::unwrap)
    ///         .any(|r| r.comment.ends_with("Yes, this is a bit weird."))
    /// );
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
    /// assert!(
    ///     Ruling::mtgo_id(57934)
    ///         .unwrap()
    ///         .map(Result::unwrap)
    ///         .any(|r| r.comment.ends_with("You read the whole contract, right?"))
    /// );
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
    /// assert!(
    ///     Ruling::arena_id(67462)
    ///         .unwrap()
    ///         .map(Result::unwrap)
    ///         .any(|r| {
    ///             r.comment
    ///                 .starts_with("Once a chapter ability has triggered,")
    ///         })
    /// );
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
    ///         .map(Result::unwrap)
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
    ///         .map(Result::unwrap)
    ///         .any(|r| r.comment == "It must flip like a coin and not like a Frisbee.")
    /// );
    /// ```
    pub fn uuid(id: Uuid) -> crate::Result<ListIter<Self>> {
        Uri::from(CARDS_URL.join(&format!("{}/", id))?.join(API_RULING)?).fetch_iter()
    }
}
