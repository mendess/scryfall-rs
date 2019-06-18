#![deny(missing_docs)]
//! [Scryfall](https://scryfall.com) provides a REST-like API for ingesting our card data
//! programatically. The API exposes information available on the regular site in easy-to-consume
//! formats.
//!
//! # Cards
//! The main way to fetch cards from this API is the [`Card`] struct.
//!
//! This allows you to get cards from `scryfall` using all of their available
//! REST Apis
//!
//! ```rust,norun
//! use scryfall::card::Card;
//! match Card::named_fuzzy("Light Bolt") {
//!     Ok(card) => assert_eq!(card.name, "Lightning Bolt"),
//!     Err(e) => panic!(format!("{:?}", e))
//! }
//! ```
//!
//! [`Card`]: card/struct.Card.html
pub mod card;
pub mod card_searcher;
pub mod catalog;
pub mod error;
pub mod format;
pub mod ruling;
pub mod set;
pub mod util;

pub use error::Result;

#[cfg(test)]
mod tests {
    use super::card::Card;

    #[test]
    fn flat_map() {
        let cards = Card::search("lightning")
            .filter_map(|x| x.ok())
            .flatten()
            .collect::<Vec<_>>();
        assert_ne!(cards.len(), 0);
        assert!(cards
            .iter()
            .all(|x| x.name.to_lowercase().contains("lightning")));
    }

    #[test]
    fn search() {
        use crate::card::Card;
        use crate::card_searcher::{
            NumericParam::CollectorNumber, Search, SearchBuilder, StringParam::Set,
        };

        let mut search = SearchBuilder::new();
        search
            .param(Box::new(CollectorNumber(123)))
            .param(Box::new(Set([b'W', b'A', b'R', 0])));
        println!("{}", (&search).to_query());
        assert_eq!(
            Card::search(&search).next().unwrap().unwrap()[0].name,
            "Demolish"
        );
    }
}
