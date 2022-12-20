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
//! # tokio_test::block_on(async {
//! match Card::named_fuzzy("Light Bolt").await {
//!     Ok(card) => assert_eq!(card.name, "Lightning Bolt"),
//!     Err(e) => panic!(format!("{:?}", e))
//! }
//! # })
//! ```
//!
//! ## Sets
//! You can also fetch information about a card set.
//!
//! The available routes for this can be seen on [`Set`]
//!
//! ```rust,no_run
//! use scryfall::set::Set;
//! # tokio_test::block_on(async {
//! assert_eq!(Set::code("mmq").await.unwrap().name, "Mercadian Masques")
//! # })
//! ```
//!
//! ## Catalogs
//! Finally `scryfall` also allows you to fetch *catalogs* which
//! are collections of Magic the Gathering data points.
//!
//! For example, one could fetch all available card names.
//! ```rust,no_run
//! use scryfall::catalog::Catalog;
//! # tokio_test::block_on(async {
//! assert!(Catalog::card_names().await.unwrap().data.len() > 0)
//! # })
//! ```
//!
//! ## Advanced Search
//!
//! One of the main features of `scryfall` is its advanced search.
//! For this the [`search`] module provides a type safe api
//! to interact and query the search engine. For advanced features like
//! sorting and collation, see [`search::advanced`].
pub mod bulk;
pub mod card;
pub mod catalog;
pub mod error;
pub mod format;
pub mod list;
pub mod ruling;
pub mod search;
pub mod set;
pub mod uri;
mod util;

/// The result type used to describe all fallible operations of the scryfall
/// crate.
pub type Result<T> = std::result::Result<T, error::Error>;

pub use card::Card;
pub use catalog::Catalog;
pub use error::Error;
pub use ruling::Ruling;
pub use set::Set;

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use futures::stream::StreamExt;

    use serde_json::{from_str, to_string};

    use crate::search::prelude::*;
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
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let handle = runtime.handle();
        handle.block_on(async move {
            Set::all()
                .await
                .unwrap()
                .into_stream()
                .map(Result::unwrap)
                .for_each(|set| async move {
                    assert!(set.code.get().len() >= 3);
                })
                .await
        });
    }

    #[test]
    #[ignore]
    fn latest_cards() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let handle = runtime.handle();
        handle.block_on(async move {
            Set::all()
                .await
                .unwrap()
                .into_stream()
                .map(Result::unwrap)
                .take(30)
                .for_each_concurrent(None, |s| async move {
                    let set_cards = set(s.code).search().await;
                    if let Err(e) = set_cards {
                        println!("Could not search for cards in '{}' - {}", s.name, e);
                    }
                })
                .await
        })
    }
}
