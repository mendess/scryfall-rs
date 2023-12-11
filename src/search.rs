//! This module provides an abstraction over the search parameters available in
//! Scryfall. For a complete documentation, refer to the
//! [official site](https://scryfall.com/docs/syntax).
//!
//! The [`Search`] trait defines a type that can be used to
//! search for cards. This is implemented for some of the types
//! provided by this module, and additionally implemented for string
//! types allowing for custom queries.
//!
//! # Prelude
//!
//! For convenience, this crate provides the [`search::prelude`][prelude]
//! module, which includes all the types and functions used by `search`.
//! To use the prelude, import all of its members as in the following example.
//!
//! ```rust,no_run
//! use scryfall::search::prelude::*;
//! ```
//!
//! # Queries
//!
//! The [`Query`][self::query::Query] object provides a mechanism for
//! constructing simple and complex Scryfall queries.
//! complex queries to Scryfall.
use async_trait::async_trait;
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
#[async_trait]
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
    async fn search(&self) -> crate::Result<ListIter<Card>> {
        Card::search(self).await
    }

    /// Convenience method for passing this object to [`Card::search_all`].
    async fn search_all(&self) -> crate::Result<Vec<Card>> {
        Card::search_all(self).await
    }

    /// Convenience method for passing this object to [`Card::search_random`].
    async fn random(&self) -> crate::Result<Card> {
        Card::search_random(self).await
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
fn write_query_string<S: ToString + ?Sized>(query: &S, url: &mut Url) -> crate::Result<()> {
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
    pub use super::param::criteria::{CardIs, PrintingIs};
    pub use super::param::value::{
        artist,
        artist_count,
        banned,
        block,
        border_color,
        cheapest,
        cmc,
        collector_number,
        color,
        color_count,
        color_identity,
        color_identity_count,
        cube,
        date,
        devotion,
        eur,
        flavor_text,
        format,
        frame,
        full_oracle_text,
        game,
        illustration_count,
        in_game,
        in_language,
        in_rarity,
        in_set,
        in_set_type,
        keyword,
        language,
        loyalty,
        mana,
        name,
        oracle_text,
        paper_print_count,
        paper_set_count,
        pow_tou,
        power,
        print_count,
        produces,
        rarity,
        restricted,
        set,
        set_count,
        set_type,
        tix,
        toughness,
        type_line,
        usd,
        usd_foil,
        watermark,
        year,
        Devotion,
        NumProperty,
        Regex,
    };
    pub use super::param::{exact, Param};
    pub use super::query::{not, Query};
    pub use super::Search;
}

#[cfg(test)]
mod tests {
    use futures::stream::StreamExt;

    use super::prelude::*;
    use crate::Card;

    #[test]
    fn basic_search() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let handle = rt.handle();
        let cards = handle.block_on(async move {
            SearchOptions::new()
                .query(Query::And(vec![
                    name("lightning"),
                    name("helix"),
                    cmc(eq(2)),
                ]))
                .unique(UniqueStrategy::Prints)
                .search()
                .await
                .unwrap()
                .into_stream()
                .map(|c| c.unwrap())
                .collect::<Vec<_>>()
                .await
        });

        assert!(cards.len() > 1);

        for card in cards {
            assert_eq!(card.name, "Lightning Helix")
        }
    }

    #[test]
    fn basic_search_buffered() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let handle = rt.handle();
        let cards = handle.block_on(async move {
            SearchOptions::new()
                .query(Query::And(vec![
                    name("lightning"),
                    name("helix"),
                    cmc(eq(2)),
                ]))
                .unique(UniqueStrategy::Prints)
                .search()
                .await
                .unwrap()
                .into_stream_buffered(10)
                .map(|c| c.unwrap())
                .collect::<Vec<_>>()
                .await
        });

        assert!(cards.len() > 1);

        for card in cards {
            assert_eq!(card.name, "Lightning Helix")
        }
    }

    #[test]
    fn random_works_with_search_options() {
        // `SearchOptions` can set more query params than the "cards/random" API method
        // accepts. Scryfall should ignore these and return a random card.
        let rt = tokio::runtime::Runtime::new().unwrap();
        let handle = rt.handle();
        assert!(handle.block_on(async move {
            SearchOptions::new()
                .query(keyword("storm"))
                .unique(UniqueStrategy::Art)
                .sort(SortOrder::Usd, SortDirection::Ascending)
                .extras(true)
                .multilingual(true)
                .variations(true)
                .random()
                .await
                .unwrap()
                .oracle_text
                .unwrap()
                .to_lowercase()
                .contains("storm")
        }));
    }

    #[test]
    fn finds_alpha_lotus() {
        let mut search = SearchOptions::new();

        search
            .query(exact("Black Lotus"))
            .unique(UniqueStrategy::Prints)
            .sort(SortOrder::Released, SortDirection::Ascending);

        eprintln!("{}", search.query_string().unwrap());
        let rt = tokio::runtime::Runtime::new().unwrap();
        let handle = rt.handle();
        assert_eq!(
            handle.block_on(async move {
                Card::search(&search)
                    .await
                    .unwrap()
                    .into_stream()
                    .next()
                    .await
                    .unwrap()
                    .unwrap()
                    .set
                    .to_string()
            }),
            "lea",
        );
    }

    #[test]
    fn finds_alpha_lotus_buffered() {
        let mut search = SearchOptions::new();

        search
            .query(exact("Black Lotus"))
            .unique(UniqueStrategy::Prints)
            .sort(SortOrder::Released, SortDirection::Ascending);

        eprintln!("{}", search.query_string().unwrap());
        let rt = tokio::runtime::Runtime::new().unwrap();
        let handle = rt.handle();
        assert_eq!(
            handle.block_on(async move {
                Card::search(&search)
                    .await
                    .unwrap()
                    .into_stream_buffered(10)
                    .next()
                    .await
                    .unwrap()
                    .unwrap()
                    .set
                    .to_string()
            }),
            "lea",
        );
    }

    #[test]
    fn rarity_comparison() {
        use crate::card::Rarity;
        let rt = tokio::runtime::Runtime::new().unwrap();
        let handle = rt.handle();
        // The cards with "Bonus" rarity (power nine in vma).
        let cards = handle.block_on(async move {
            SearchOptions::new()
                .query(rarity(gt(Rarity::Mythic)))
                .search()
                .await
                .unwrap()
                .into_stream()
                .collect::<Vec<_>>()
                .await
        });

        assert!(cards.len() >= 9, "Couldn't find the Power Nine from VMA.");

        assert!(cards
            .into_iter()
            .map(|c| c.unwrap())
            .all(|c| c.rarity > Rarity::Mythic));
    }

    #[test]
    fn rarity_comparison_buffered() {
        use crate::card::Rarity;
        let rt = tokio::runtime::Runtime::new().unwrap();
        let handle = rt.handle();
        // The cards with "Bonus" rarity (power nine in vma).
        let cards = handle.block_on(async move {
            SearchOptions::new()
                .query(rarity(gt(Rarity::Mythic)))
                .search()
                .await
                .unwrap()
                .into_stream_buffered(10)
                .collect::<Vec<_>>()
                .await
        });

        assert!(cards.len() >= 9, "Couldn't find the Power Nine from VMA.");

        assert!(cards
            .into_iter()
            .map(|c| c.unwrap())
            .all(|c| c.rarity > Rarity::Mythic));
    }

    #[test]
    fn numeric_property_comparison() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let handle = rt.handle();

        let card = handle
            .block_on(async move {
                Card::search_random(Query::And(vec![
                    power(eq(NumProperty::Toughness)),
                    pow_tou(eq(NumProperty::Cmc)),
                    not(CardIs::Funny),
                ]))
                .await
            })
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
        assert_eq!(power + toughness, card.cmc.unwrap_or_default() as u32);

        let card = handle.block_on(async move {
            Card::search(pow_tou(gt(NumProperty::Year)))
                .await
                .unwrap()
                .into_stream()
                .map(|c| c.unwrap())
                .any(|c| async move { &c.name == "Infinity Elemental" })
                .await
        });

        assert!(card);
    }

    #[test]
    fn numeric_property_comparison_buffered() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let handle = rt.handle();

        let card = handle
            .block_on(async move {
                Card::search_random(Query::And(vec![
                    power(eq(NumProperty::Toughness)),
                    pow_tou(eq(NumProperty::Cmc)),
                    not(CardIs::Funny),
                ]))
                .await
            })
            .unwrap();

        let power = card
            .power
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or_default();
        let toughness = card
            .toughness
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or_default();

        assert_eq!(
            power, toughness,
            "power was not equal to toughness for card {}",
            card.name
        );
        assert_eq!(
            power + toughness,
            card.cmc.unwrap_or_default() as u32,
            "power and toughness added was not equal to cmc for card {}",
            card.name
        );

        let card = handle.block_on(async move {
            Card::search(pow_tou(gt(NumProperty::Year)))
                .await
                .unwrap()
                .into_stream_buffered(10)
                .map(|c| c.unwrap())
                .any(|c| async move { &c.name == "Infinity Elemental" })
                .await
        });

        assert!(card);
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
