use std::fmt;

use self::compare::CompareOp;
use self::property::Property;
use self::value::{ParamValue, ValueKind};
use crate::Lrc;

pub mod compare;
pub mod property;
pub mod value;

/// A filter to provide to the search to reduce the cards returned.
///
/// For more information on available parameters, refer to the
/// [official docs](https://scryfall.com/docs/syntax).
#[derive(Clone, Debug)]
pub struct Param(ParamImpl);

impl Param {
    fn property(prop: Property) -> Self {
        Param(ParamImpl::Property(prop))
    }

    fn value(kind: ValueKind, value: impl 'static + ParamValue) -> Self {
        Param(ParamImpl::Value(kind, None, Lrc::new(value)))
    }

    fn cmp_value(kind: ValueKind, op: CompareOp, value: impl 'static + ParamValue) -> Self {
        Param(ParamImpl::Value(kind, Some(op), Lrc::new(value)))
    }
}

#[derive(Clone, Debug)]
enum ParamImpl {
    Property(Property),
    Value(ValueKind, Option<CompareOp>, Lrc<dyn ParamValue>),
}

impl PartialEq for Param {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
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
