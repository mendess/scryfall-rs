//! This module provides an abstraction over the search parameters available in
//! Scryfall. For a complete documentation, refer to the
//! [official site](https://scryfall.com/docs/syntax).
//!
//! The [`Search`] trait defines a type that can be used to
//! search for cards. This is implemented for some of the types
//! provided by this module, and additionally implemented for string
//! types allowing for custom queries.
//!
//! # Queries
//!
//! The [`Query`][self::query::Query] object provides a mechanism for
//! constructing simple and complex Scryfall queries.
//! complex queries to Scryfall.
use url::Url;

use crate::list::ListIter;
use crate::Card;

pub mod advanced;
pub mod param;
pub mod query;

/// A type implementing `Search` can be turned into a Scryfall query. This is
/// the argument type for [`Card::search`] and
/// [`search_random`][Card::search_random].
///
/// The `scryfall` crate provides the type [`Query`][self::query::Query] for
/// specifying search expressions. For advanced search, use
/// [`SearchOptions`][self::advanced::SearchOptions] to specify sorting,
/// unique rollup, and other options.
///
/// The `Search` trait is implemented for `&str` and `String` as well,
/// supporting custom searches using [Scryfall syntax](https://scryfall.com/docs/syntax).
pub trait Search {
    /// Write this search as the query for the given `Url`.
    fn write_query(&self, url: &mut Url) -> crate::Result<()>;

    #[cfg(test)]
    fn query_string(&self) -> crate::Result<String> {
        let mut url = Url::parse("http://localhost")?;
        self.write_query(&mut url)?;
        Ok(url.query().unwrap_or_default().to_string())
    }

    /// Convenience method for passing this object to [`Card::search`].
    fn search(&self) -> crate::Result<ListIter<Card>> {
        Card::search_new(self)
    }

    /// Convenience method for passing this object to [`Card::search_all`].
    fn search_all(&self) -> crate::Result<Vec<Card>> {
        Card::search_all(self)
    }

    /// Convenience method for passing this object to [`Card::search_random`].
    fn random(&self) -> crate::Result<Card> {
        Card::search_random_new(self)
    }
}

impl<T: Search + ?Sized> Search for &T {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        <T as Search>::write_query(*self, url)
    }
}

impl<T: Search + ?Sized> Search for &mut T {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        <T as Search>::write_query(*self, url)
    }
}

#[inline]
fn write_query_string(query: &(impl ToString + ?Sized), url: &mut Url) -> crate::Result<()> {
    url.query_pairs_mut()
        .append_pair("q", query.to_string().as_str());
    Ok(())
}

impl Search for str {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        write_query_string(self, url)
    }
}

impl Search for String {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        write_query_string(self, url)
    }
}

/// This module re-exports types used for card searches into a common
/// place, so they can all be imported with a glob.
///
/// ```rust,no_run
/// use scryfall::search::prelude::*;
/// ```
pub mod prelude {
    pub use super::advanced::{SearchOptions, SortDirection, SortOrder, UniqueStrategy};
    pub use super::param::compare::{eq, gt, gte, lt, lte, neq};
    pub use super::param::functions::*;
    pub use super::param::value::NumProperty;
    pub use super::param::Param;
    pub use super::query::{not, Query};
    pub use super::Search;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use crate::Card;

    #[test]
    fn basic_search() {
        let cards = SearchOptions::new()
            .query(Query::And(vec![
                name("lightning"),
                name("helix"),
                cmc(eq(2)),
            ]))
            .unique(UniqueStrategy::Prints)
            .search()
            .unwrap()
            .map(|c| c.unwrap())
            .collect::<Vec<_>>();

        assert!(cards.len() > 1);

        for card in cards {
            assert_eq!(card.name, "Lightning Helix")
        }
    }

    #[test]
    fn random_works_with_search_options() {
        // `SearchOptions` can set more query params than the "cards/random" API method
        // accepts. Scryfall should ignore these and return a random card.
        assert!(
            SearchOptions::new()
                .query(keyword("storm"))
                .unique(UniqueStrategy::Art)
                .sort(SortOrder::Usd, SortDirection::Ascending)
                .extras(true)
                .multilingual(true)
                .variations(true)
                .random()
                .unwrap()
                .oracle_text
                .unwrap()
                .to_lowercase()
                .contains("storm")
        );
    }

    // TODO(msmorgan): Rework this.
    // #[test]
    // #[ignore]
    // fn all_properties_work() {
    //     use strum::IntoEnumIterator;
    //
    //     for p in Criterion::iter() {
    //         let query = criterion(p);
    //         query
    //             .random()
    //             .unwrap_or_else(|_| panic!("Could not get a random card with {}",
    // p));     }
    // }

    #[test]
    fn finds_alpha_lotus() {
        let mut search = SearchOptions::new();

        search
            .query(exact("Black Lotus"))
            .unique(UniqueStrategy::Prints)
            .sort(SortOrder::Released, SortDirection::Ascending);

        eprintln!("{}", search.query_string().unwrap());

        assert_eq!(
            Card::search_new(&search)
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
                .set
                .to_string(),
            "lea",
        );
    }

    #[test]
    fn rarity_comparison() {
        use crate::card::Rarity;

        // The cards with "Bonus" rarity (power nine in vma).
        let cards = SearchOptions::new()
            .query(rarity(gt(Rarity::Mythic)))
            .search()
            .unwrap()
            .collect::<Vec<_>>();

        assert!(cards.len() >= 9, "Couldn't find the Power Nine from VMA.");

        assert!(
            cards
                .into_iter()
                .map(|c| c.unwrap())
                .all(|c| c.rarity > Rarity::Mythic)
        );
    }

    #[test]
    fn numeric_property_comparison() {
        let card = Card::search_random_new(Query::And(vec![
            power(eq(NumProperty::Toughness)),
            pow_tou(eq(NumProperty::Cmc)),
            is_funny(false),
        ]))
        .unwrap();

        let power = card
            .power
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or_default();
        let toughness = card
            .toughness
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or_default();

        assert_eq!(power, toughness);
        assert_eq!(power + toughness, card.cmc as u32);

        let card = Card::search_new(pow_tou(gt(NumProperty::Year)))
            .unwrap()
            .map(|c| c.unwrap())
            .collect::<Vec<_>>();

        assert!(card.into_iter().any(|c| &c.name == "Infinity Elemental"));
    }

    #[test]
    fn query_string_sanity_check() {
        let query = cmc(4).and(name("Yargle"));
        assert_eq!(
            query.query_string().unwrap(),
            "q=%28cmc%3A4+AND+name%3A%22Yargle%22%29"
        );
    }
}
