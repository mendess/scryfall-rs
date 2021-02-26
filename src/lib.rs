#![deny(missing_docs)]
//! [Scryfall](https://scryfall.com) provides a REST-like API for ingesting our card data
//! programatically. The API exposes information available on the regular site
//! in easy-to-consume formats.
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
//! Finally `scryfall` also allows you to fetch *catalogs* which
//! are collections of Magic the Gathering data points.
//!
//! For example, one could fetch all available card names.
//! ```rust,no_run
//! use scryfall::catalog::Catalog;
//! assert!(Catalog::card_names().unwrap().data.len() > 0)
//! ```
//!
//! ## Advanced Search
//! One of the main features of `scryfall` is its advanced search.
//! For this the [`card_searcher`] module provides a type safe api
//! to interact and query the search engine.

pub mod bulk;
pub mod card;
pub mod card_searcher;
pub mod catalog;
pub mod error;
pub mod format;
pub mod list;
pub mod ruling;
pub mod set;
pub mod uri;
mod util;

/// The result type used to describe all fallible operations of the scryfall
/// crate.
pub type Result<T> = std::result::Result<T, error::Error>;

pub use card::Card;
pub use catalog::Catalog;
pub use ruling::Ruling;
pub use set::Set;

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use rayon::prelude::*;
    use serde_json::{from_str, to_string};

    use crate::card_searcher::{SearchBuilder, StringParam};
    use crate::set::{Set, SetCode};

    #[test]
    fn set_code_serde_test() {
        let instance = SetCode::try_from("war").unwrap();
        let new_instance: SetCode = from_str(&to_string(&instance).unwrap()).unwrap();
        assert_eq!(new_instance, instance);

        let instance = SetCode::try_from("wwar").unwrap();
        let new_instance: SetCode = from_str(&to_string(&instance).unwrap()).unwrap();
        assert_eq!(new_instance, instance)
    }

    #[test]
    #[ignore]
    fn all_sets() {
        for set in Set::all().unwrap().map(Result::unwrap) {
            assert!(set.code.get().len() >= 3);
        }
    }

    #[test]
    #[ignore]
    fn latest_cards() {
        Set::all()
            .unwrap()
            .map(Result::unwrap)
            .take(30)
            .par_bridge()
            .for_each(|set| {
                let set_cards = SearchBuilder::new()
                    .param(StringParam::Set(set.code))
                    .search();
                if let Err(e) = set_cards {
                    panic!("Could not search for cards in '{}' - {}", set.name, e);
                }
            })
    }
}
