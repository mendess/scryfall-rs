#![deny(missing_docs)]
//! [Scryfall](https://scryfall.com) provides a REST-like API for ingesting our card data
//! programatically. The API exposes information available on the regular site in easy-to-consume
//! formats.
//!
//! ## Cards
//! The main way to fetch cards from this API is the [`Card`] struct.
//!
//! This allows you to get cards from `scryfall` using all of their available
//! REST Apis
//!
//! ```rust,no_run
//! use scryfall::card::Card;
//! match Card::named_fuzzy("Light Bolt") {
//!     Ok(card) => assert_eq!(card.name, "Lightning Bolt"),
//!     Err(e) => panic!(format!("{:?}", e))
//! }
//! ```
//!
//! ## Sets
//! You can also fetch information about a card set.
//!
//! The available routes for this can be seen on [`Set`]
//!
//! ```rust,no_run
//! use scryfall::set::Set;
//! assert_eq!(Set::code("mmq").unwrap().name, "Mercadian Masques")
//! ```
//!
//! ## Catalogs
//! Finally `scryfall` also allows you to fetch *catalogs* witch
//! are collections of Magic the Gathering data points.
//!
//! For example, one could fetch all available card names.
//! ```rust,no_run
//! use scryfall::catalog::Catalog;
//! assert!(Catalog::card_names().unwrap().data.len() > 0)
//! ```
//!
//! ## Advanced Search
//! One of the main features of `scryfall` is it's advanced search.
//! For this the [`card_searcher`] module provides a type safe api
//! to interact and query the search engine.
//!
//! [`Card`]: card/struct.Card.html
//! [`Set`]: set/struct.Set.html
//! [`card_searcher`]: card_searcher/index.html
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
    use crate::card::Card;
    use crate::set::set_code::SetCode;
    use crate::set::Set;
    use crate::util::uri::{PaginatedURI, URI};
    use serde_json::{from_str, to_string};
    use std::convert::TryFrom;

    #[test]
    fn set_code_serde_test() {
        let instance = SetCode::try_from("war").unwrap();
        let new_instance: SetCode = from_str(&to_string(&instance).unwrap()).unwrap();
        assert_eq!(new_instance, instance);

        let instance = SetCode::try_from("wwar").unwrap();
        let new_instance: SetCode = from_str(&to_string(&instance).unwrap()).unwrap();
        assert_eq!(new_instance, instance)
    }

    // #[test]
    // fn all_cards() {
    //     let mut page = 1;
    //     for cards in Card::all() {
    //         if cards.is_err() {
    //             eprintln!("{:?}", cards);
    //             eprintln!("{}", page);
    //         }
    //         page += 1;
    //         cards.unwrap();
    //     }
    // }

    // #[test]
    // fn all_sets() {
    //     let mut page = 1;
    //     for sets in Set::all() {
    //         if sets.is_err() {
    //             eprintln!("{:?}", sets);
    //             eprintln!("{}", page);
    //         }
    //         page += 1;
    //         sets.unwrap();
    //     }
    // }
}
