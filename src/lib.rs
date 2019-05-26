//! [Scryfall](https://scryfall.com) provides a REST-like API for ingesting our card data
//! programatically. The API exposes information available on the regular site in easy-to-consume
//! formats.
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
}
