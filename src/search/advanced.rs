//! This module provides facilities for advanced search.
//! See the [`SearchOptions`] type for more details.

use serde::{Serialize, Serializer};
use url::Url;

use crate::search::query::Query;
use crate::search::Search;

/// Advanced searching options for Scryfall, including unique de-duplication
/// strategy, sort order, page number, and any extras to include. For
/// documentation on each option, refer to this struct's methods.
///
/// For more information, refer to the [official docs](https://scryfall.com/docs/api/cards/search).
#[derive(Serialize, Default, Debug)]
pub struct SearchOptions {
    #[serde(skip_serializing_if = "is_default")]
    unique: UniqueStrategy,
    #[serde(skip_serializing_if = "is_default")]
    order: SortOrder,
    #[serde(skip_serializing_if = "is_default")]
    dir: SortDirection,
    #[serde(skip_serializing_if = "is_default")]
    page: usize,
    #[serde(skip_serializing_if = "is_default")]
    include_extras: bool,
    #[serde(skip_serializing_if = "is_default")]
    include_multilingual: bool,
    #[serde(skip_serializing_if = "is_default")]
    include_variations: bool,
    #[serde(rename = "q", serialize_with = "serialize_query")]
    query: Query,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    value == &Default::default()
}

fn serialize_query<S: Serializer>(query: &Query, serializer: S) -> Result<S::Ok, S::Error> {
    query.to_string().serialize(serializer)
}

impl Search for SearchOptions {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        self.serialize(serde_urlencoded::Serializer::new(
            &mut url.query_pairs_mut(),
        ))?;
        Ok(())
    }
}

impl SearchOptions {
    /// Constructs a new `SearchOptions` with default values and an empty query.
    pub fn new() -> Self {
        SearchOptions {
            page: 1,
            ..Default::default()
        }
    }

    /// Constructs a new `SearchOptions` with default values and the specified
    /// query.
    pub fn with_query(query: Query) -> Self {
        SearchOptions {
            query,
            ..Self::new()
        }
    }

    /// Sets the query to use for this search.
    pub fn query(&mut self, query: Query) -> &mut Self {
        self.query = query;
        self
    }

    /// Sets the page number to start with. Page 0 is equivalent to page 1.
    pub fn page(&mut self, page: usize) -> &mut Self {
        self.page = page;
        self
    }

    /// Sets the strategy for omitting similar cards.
    pub fn unique(&mut self, unique: UniqueStrategy) -> &mut Self {
        self.unique = unique;
        self
    }

    /// Sets the sort order and direction for returned cards.
    #[inline]
    pub fn sort(&mut self, order: SortOrder, dir: SortDirection) -> &mut Self {
        self.order(order).direction(dir)
    }

    /// Sets the sort order for returned cards.
    pub fn order(&mut self, order: SortOrder) -> &mut Self {
        self.order = order;
        self
    }

    /// Sets the sort direction for returned cards.
    pub fn direction(&mut self, dir: SortDirection) -> &mut Self {
        self.dir = dir;
        self
    }

    /// If true, extra cards (tokens, planes, etc) will be included.
    pub fn extras(&mut self, include_extras: bool) -> &mut Self {
        self.include_extras = include_extras;
        self
    }

    /// If true, cards in every language supported by Scryfall will be included.
    pub fn multilingual(&mut self, include_multilingual: bool) -> &mut Self {
        self.include_multilingual = include_multilingual;
        self
    }

    /// If true, rare care variants will be included, like the
    /// [Hairy Runesword](https://scryfall.com/card/drk/107%E2%80%A0/runesword).
    pub fn variations(&mut self, include_variations: bool) -> &mut Self {
        self.include_variations = include_variations;
        self
    }
}

/// The unique parameter specifies if Scryfall should remove “duplicate” results
/// in your query.
#[derive(Serialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum UniqueStrategy {
    /// Removes duplicate gameplay objects (cards that share a name and have the
    /// same functionality). For example, if your search matches more than
    /// one print of Pacifism, only one copy of Pacifism will be returned.
    #[default]
    Cards,
    /// Returns only one copy of each unique artwork for matching cards. For
    /// example, if your search matches more than one print of Pacifism, one
    /// card with each different illustration for Pacifism will be returned,
    /// but any cards that duplicate artwork already in the results will
    /// be omitted.
    Art,
    /// Returns all prints for all cards matched (disables rollup). For example,
    /// if your search matches more than one print of Pacifism, all matching
    /// prints will be returned.
    Prints,
}

/// The order parameter determines how Scryfall should sort the returned cards.
#[derive(Serialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum SortOrder {
    /// Sort cards by name, A → Z
    #[default]
    Name,
    /// Sort cards by their set and collector number: AAA/#1 → ZZZ/#999
    Set,
    /// Sort cards by their release date: Newest → Oldest
    Released,
    /// Sort cards by their rarity: Common → Mythic
    Rarity,
    /// Sort cards by their color and color identity: WUBRG → multicolor →
    /// colorless
    Color,
    /// Sort cards by their lowest known U.S. Dollar price: 0.01 → highest, null
    /// last
    Usd,
    /// Sort cards by their lowest known TIX price: 0.01 → highest, null last
    Tix,
    /// Sort cards by their lowest known Euro price: 0.01 → highest, null last
    Eur,
    /// Sort cards by their converted mana cost: 0 → highest
    Cmc,
    /// Sort cards by their power: null → highest
    Power,
    /// Sort cards by their toughness: null → highest
    Toughness,
    /// Sort cards by their EDHREC ranking: lowest → highest
    Edhrec,
    /// Sort cards by their front-side artist name: A → Z
    Artist,
}

/// Which direction the sorting should occur:
#[derive(Serialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum SortDirection {
    /// Scryfall will automatically choose the most intuitive direction to sort
    #[default]
    Auto,
    /// Sort ascending (flip the direction of the arrows in [`SortMethod`])
    ///
    /// [`SortMethod`]: enum.SortMethod.html
    #[serde(rename = "asc")]
    Ascending,
    /// Sort descending (flip the direction of the arrows in [`SortMethod`])
    ///
    /// [`SortMethod`]: enum.SortMethod.html
    #[serde(rename = "desc")]
    Descending,
}
