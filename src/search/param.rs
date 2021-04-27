//! This module defines [`Param`], which represents a single search parameter
//! for a Scryfall query. For combinations of parameters, see the
//! [`Query`][crate::search::query] module.
//!
//! There are two kinds of `Param`: boolean criteria, and parameters
//! that take a value.
//!
//! Cards and printings are tagged with many different types of criteria
//! by Scryfall. Each of these represents a boolean property that the
//! card either has or does not. Searching by a criterion will only match
//! cards that have the flag. For example,
//! ['is:firstprint'][self::criteria::PrintingIs::FirstPrint] matches only
//! the first printing of a card, and
//! ['has:watermark'][self::criteria::PrintingIs::Watermark] matches printings
//! which have a watermark. For a list of all available criteria, see the
//! [`criteria`] module.
//!
//! The rest of the search parameters are comprised of a name and a value, such
//! as ['name:lightning'][self::value::name] or
//! ['year:1995'][self::value::year]. All available value parameters are all
//! available as helper functions defined in the [`value`] module.
use std::fmt;

use url::Url;

use self::compare::CompareOp;
use self::criteria::Criterion;
use self::value::ValueKind;
use crate::search::query::Query;
use crate::search::Search;

pub mod compare;
pub mod criteria;
pub mod value;

/// A filter to provide to the search to reduce the cards returned.
///
/// A `Param` can be an [exact card name][exact()], a [`Criterion`], or a
/// comparison of parameter values.
///
/// Usually `Param` does not need to be used directly, but instead is wrapped
/// in a [`Query`] so it can be combined with other `Param`s.
///
/// For more information on available parameters, refer to the
/// [official docs](https://scryfall.com/docs/syntax).
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Param(ParamImpl);

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<Criterion> for Param {
    fn from(criterion: Criterion) -> Self {
        Param::criterion(criterion)
    }
}

impl Search for Param {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        super::write_query_string(self, url)
    }
}

impl Param {
    fn exact(value: impl Into<String>) -> Self {
        Param(ParamImpl::ExactName(value.into()))
    }

    fn criterion(criterion: Criterion) -> Self {
        Param(ParamImpl::Criterion(criterion))
    }

    fn value(kind: ValueKind, value: impl ToString) -> Self {
        Param(ParamImpl::Value(kind, value.to_string()))
    }

    fn comparison(kind: ValueKind, op: CompareOp, value: impl ToString) -> Self {
        Param(ParamImpl::Comparison(kind, op, value.to_string()))
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum ParamImpl {
    ExactName(String),
    Criterion(Criterion),
    Value(ValueKind, String),
    Comparison(ValueKind, CompareOp, String),
}

impl fmt::Display for ParamImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParamImpl::Criterion(prop) => write!(f, "{}", prop),
            ParamImpl::ExactName(name) => write!(f, "!\"{}\"", name),
            ParamImpl::Value(kind, value) => kind.fmt_value(value.as_str(), f),
            ParamImpl::Comparison(kind, op, value) => kind.fmt_comparison(*op, &*value, f),
        }
    }
}

/// Matches a card whose name is exactly `name`.
pub fn exact(name: impl Into<String>) -> Query {
    Query::Param(Param::exact(name))
}
