use std::fmt;

use url::Url;

use crate::search::param::Param;
use crate::search::Search;

impl Search for Query {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        url.query_pairs_mut()
            .append_pair("q", self.to_string().as_str());
        Ok(())
    }
}

/// A search query, composed of search parameters and boolean operations.
///
/// For information on search parameters, see [`Param`].
// TODO(msmorgan): Move the docs from here to somewhere else?
#[derive(PartialEq, Debug)]
pub enum Query {
    /// The returned cards must match all of the sub-queries.
    And(Vec<Query>),
    /// The returned cards must match at least one of the sub-queries.
    Or(Vec<Query>),
    /// The returned cards must not match the sub-query.
    Not(Box<Query>),
    /// The returned cards must match the specified search param.
    Param(Param),
    /// Empty query, used as a default value. Attempting to search with an empty
    /// query will result in a failure response.
    #[doc(hidden)]
    Empty,
}

impl Default for Query {
    fn default() -> Self {
        Query::Empty
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (exprs, sep) = match &self {
            Query::And(exprs) => (exprs, " AND "),
            Query::Or(exprs) => (exprs, " OR "),
            Query::Not(expr) => return write!(f, "-{}", expr),
            Query::Param(param) => return write!(f, "{}", param),
            Query::Empty => return write!(f, ""),
        };

        use itertools::Itertools;
        write!(f, "({})", exprs.iter().format(sep))
    }
}

impl From<Param> for Query {
    fn from(param: Param) -> Self {
        Query::Param(param)
    }
}

impl Query {
    /// Combines this query with `other` using the boolean AND operation.
    pub fn and(self, other: Self) -> Query {
        match (self, other) {
            (Query::Empty, q) | (q, Query::Empty) => q,
            (Query::And(mut a_list), Query::And(mut b_list)) => {
                a_list.append(&mut b_list);
                Query::And(a_list)
            },
            (Query::And(mut a_list), b) => {
                a_list.push(b);
                Query::And(a_list)
            },
            (a, Query::And(mut b_list)) => {
                b_list.insert(0, a);
                Query::And(b_list)
            },
            (a, b) => Query::And(vec![a, b]),
        }
    }

    /// Combines this query with `other` using the boolean OR operation.
    pub fn or(self, other: Self) -> Query {
        match (self, other) {
            (Query::Empty, q) | (q, Query::Empty) => q,
            (Query::Or(mut a_list), Query::Or(mut b_list)) => {
                a_list.append(&mut b_list);
                Query::Or(a_list)
            },
            (Query::Or(mut a_list), b) => {
                a_list.push(b);
                Query::Or(a_list)
            },
            (a, Query::Or(mut b_list)) => {
                b_list.insert(0, a);
                Query::Or(b_list)
            },
            (a, b) => Query::Or(vec![a, b]),
        }
    }
}

/// Negates the specified `query`.
pub fn not(query: Query) -> Query {
    match query {
        Query::Not(q) => *q,
        Query::Empty => Query::Empty,
        q => Query::Not(Box::new(q)),
    }
}
