//! This module defines [`Param`], which represents a single search parameter
//! for a Scryfall query. For combinations of parameters, see the
//! [`Query`][crate::search::query] module.
//!
//! There are two main kinds of `Param`: boolean flags and params that take a
//! value argument.
//!
//! Many properties of cards or printings are either true or false,
//! such as ['is:firstprint'][self::property::Property::IsFirstPrint] or
//! ['has:watermark'][self::property::Property::HasWatermark]. An enum with
//! all available properties can be found in the [`property`] module.
//!
//! The rest of the search parameters consist of a name and a value, such as
//! ['name:lightning'][self::value::name] or ['year:1995'][self::value::year].
//! All available value parameters are all available as helper functions defined
//! in the [`value`] module.
use std::fmt;

use url::Url;

use self::compare::CompareOp;
use self::property::Property;
use self::value::{ParamValue, ValueKind};
use crate::search::Search;
use crate::Lrc;

pub mod compare;
pub mod property;
pub mod value;

/// A filter to provide to the search to reduce the cards returned.
///
/// For more information on available parameters, refer to the
/// [official docs](https://scryfall.com/docs/syntax).
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Param(ParamImpl);

impl Param {
    fn property(prop: Property) -> Self {
        Param(ParamImpl::Property(prop))
    }

    fn value(kind: ValueKind, value: impl ToString) -> Self {
        Param(ParamImpl::Value(kind, None, value.to_string()))
    }

    fn cmp_value(kind: ValueKind, op: CompareOp, value: impl ToString) -> Self {
        Param(ParamImpl::Value(kind, Some(op), value.to_string()))
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum ParamImpl {
    Property(Property),
    Value(ValueKind, Option<CompareOp>, String),
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            ParamImpl::Property(prop) => write!(f, "{}", prop),
            ParamImpl::Value(kind, op, value) => kind.fmt_value(*op, &*value, f),
        }
    }
}

impl From<Property> for Param {
    fn from(prop: Property) -> Self {
        Param(ParamImpl::Property(prop))
    }
}

impl Search for Param {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        super::write_query_string(self, url)
    }
}
