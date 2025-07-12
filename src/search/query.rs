//! This module defines the [`Query`] type, which allows for combinations
//! of [`Param`]s.

use std::fmt;

use url::Url;

use crate::search::param::Param;
use crate::search::Search;

/// A search query, composed of search parameters and boolean operations.
///
/// `Query` is an expression tree, supporting `AND`, `OR`, and `NOT` operations,
/// with the [`And`][Query::And], [`Or`][Query::Or], and [`Not`][Query::Not]
/// variants respectively. Leaf variants are [`Param`][`Query::Param`] and
/// [`Custom`][Query::Custom].
///
/// # Examples
/// ```rust
/// # use scryfall::search::prelude::*;
/// # fn main() -> scryfall::Result<()> {
/// use scryfall::card::Rarity;
/// # tokio_test::block_on(async {
/// let one_odd_eldrazi = Query::And(vec![
///     Query::Or(vec![power(9), toughness(9)]),
///     Query::Custom("t:eldrazi".to_string()),
///     set("bfz"),             // A `Param` variant.
///     rarity(Rarity::Mythic), // A `Param` variant.
///     CardIs::OddCmc.into(),
/// ])
/// .search()
/// .await?
/// .next()
/// .await
/// .unwrap()?;
/// assert_eq!(one_odd_eldrazi.name, "Void Winnower");
/// # Ok(())
/// # })
/// # }
/// ```
///
/// For information on search parameters, see the
/// [`param`][crate::search::param] module.
#[derive(Clone, PartialEq, Debug)]
pub enum Query {
    /// The returned cards must match all of the sub-queries.
    And(Vec<Query>),
    /// The returned cards must match at least one of the sub-queries.
    Or(Vec<Query>),
    /// The returned cards must not match the sub-query.
    Not(Box<Query>),
    /// The returned cards must match the specified search `Param`.
    Param(Param),
    /// A custom query, in valid [Scryfall syntax](https://scryfall.com/docs/syntax).
    ///
    /// *Note*: This variant is provided so that users of this crate can use the
    /// latest search features on scryfall.com without waiting for the crate
    /// to be updated. If you encounter a situation where this must be used,
    /// please [file an issue](https://github.com/mendess/scryfall-rs/issues/new).
    Custom(String),
}

impl Default for Query {
    fn default() -> Self {
        Query::And(vec![])
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (exprs, sep) = match &self {
            Query::And(exprs) => (exprs, " AND "),
            Query::Or(exprs) => (exprs, " OR "),
            Query::Not(expr) => return write!(f, "-{expr}"),
            Query::Param(param) => return write!(f, "{param}"),
            Query::Custom(expr) => return write!(f, "({expr})"),
        };

        use itertools::Itertools;
        // If `exprs` is empty, the output is '()', which will be ignored.
        write!(f, "({})", exprs.iter().format(sep))
    }
}

impl Search for Query {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        super::write_query_string(self, url)
    }
}

impl From<Param> for Query {
    fn from(param: Param) -> Self {
        Query::Param(param)
    }
}

macro_rules! impl_and_or {
    ($(
        $(#[$($attr:meta)*])*
        $meth:ident($Var:ident),
    )*) => {
        $(
            $(#[$($attr)*])*
            pub fn $meth(self, other: impl Into<Query>) -> Self {
                match (self, other.into()) {
                    (Query::$Var(mut a_list), Query::$Var(mut b_list)) => {
                        a_list.append(&mut b_list);
                        Query::$Var(a_list)
                    },
                    (Query::$Var(mut a_list), b) => {
                        a_list.push(b);
                        Query::$Var(a_list)
                    },
                    (a, Query::$Var(mut b_list)) => {
                        b_list.insert(0, a);
                        Query::$Var(b_list)
                    },
                    (a, b) => Query::$Var(vec![a, b]),
                }
            }
        )*
    };
}

impl Query {
    impl_and_or! {
        #[doc = "Combines `self` with `other` using the boolean AND operation."]
        and(And),
        #[doc = "Combines `self` with `other` using the boolean OR operation."]
        or(Or),
    }
}

/// Negates the specified `query`.
pub fn not(query: impl Into<Query>) -> Query {
    match query.into() {
        Query::Not(q) => *q,
        q => Query::Not(Box::new(q)),
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;

    use super::*;
    use crate::search::prelude::*;

    #[tokio::test]
    async fn even_power() -> crate::Result<()> {
        // Scryfall doesn't support "power:even", so let's do it manually.
        let normal_creatures = type_line("Creature").and(not(CardIs::Funny));
        let highest_power: u32 = SearchOptions::new()
            .query(normal_creatures.clone())
            .sort(SortOrder::Power, SortDirection::Descending)
            .search()
            .await?
            .into_stream()
            .next()
            .await
            .unwrap()?
            .power
            .and_then(|pow| pow.parse().ok())
            .unwrap_or(0);
        let query = normal_creatures.and(Query::Or((0..=highest_power).map(power).collect()));
        // There are at least 1000 cards with even power.
        assert!(query.search().await.unwrap().size_hint().0 > 1000);
        Ok(())
    }

    #[tokio::test]
    async fn even_power_buffered() -> crate::Result<()> {
        // Scryfall doesn't support "power:even", so let's do it manually.
        let normal_creatures = type_line("Creature").and(not(CardIs::Funny));
        let highest_power: u32 = SearchOptions::new()
            .query(normal_creatures.clone())
            .sort(SortOrder::Power, SortDirection::Descending)
            .search()
            .await?
            .into_stream_buffered(10)
            .next()
            .await
            .unwrap()?
            .power
            .and_then(|pow| pow.parse().ok())
            .unwrap_or(0);
        let query = normal_creatures.and(Query::Or((0..=highest_power).map(power).collect()));
        // There are at least 1000 cards with even power.
        assert!(query.search().await.unwrap().size_hint().0 > 1000);
        Ok(())
    }
}
