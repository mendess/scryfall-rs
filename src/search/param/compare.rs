//! This module defines the [`Compare`] type, which represents a comparison
//! operator and right-hand side of a comparison expression. Certain
//! [`ParamValue`] subtraits are implemented for `Compare<T>`, depending on
//! whether Scryfall syntax supports comparing for that.
//!
//! To construct a `Compare` instance, use the helper functions defined in this
//! module: [`lt`], [`lte`], [`gt`], [`gte`], [`eq`], and [`neq`].

use std::fmt;

use crate::search::param::value::{ParamValue, ValueKind};
use crate::search::param::Param;

/// An operator and RHS for a comparison expression of a parameter.
/// To construct an instance, use one of the helper functions from the
/// [`compare`][self] module: [`lt`], [`lte`], [`gt`], [`gte`], [`eq`], or
/// [`neq`].
///
/// # Example
///
/// ```rust
/// # use scryfall::search::prelude::*;
/// # tokio_test::block_on(async {
/// let query = cmc(gte(5)).and(type_line("planeswalker"));
/// let card = query.random().await.unwrap();
///
/// assert!(card.cmc.unwrap() as u32 >= 5);
/// assert!(card.type_line.unwrap().to_lowercase().contains("planeswalker"));
/// # })
/// ```
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Compare<T> {
    op: CompareOp,
    value: T,
}

impl<T: fmt::Display> fmt::Display for Compare<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", compare_op_str(Some(self.op)), &self.value)
    }
}

impl<T: ParamValue> ParamValue for Compare<T> {
    fn into_param(self, kind: ValueKind) -> Param {
        Param::comparison(kind, self.op, self.value)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) enum CompareOp {
    Lte,
    Lt,
    Gte,
    Gt,
    Eq,
    Neq,
}

pub(super) const fn compare_op_str(op: Option<CompareOp>) -> &'static str {
    match op {
        None => ":",
        Some(CompareOp::Lte) => "<=",
        Some(CompareOp::Lt) => "<",
        Some(CompareOp::Gte) => ">=",
        Some(CompareOp::Gt) => ">",
        Some(CompareOp::Eq) => "=",
        Some(CompareOp::Neq) => "!=",
    }
}

macro_rules! compare_fns {
    ($(
        $(#[$($attr:meta)*])*
        $meth:ident => $Variant:ident,
    )*) => {
        $(
            $(#[$($attr)*])*
            pub fn $meth<T: ParamValue>(x: T) -> Compare<T> {
                Compare {
                    op: CompareOp::$Variant,
                    value: x,
                }
            }
        )*
    };
}

compare_fns! {
    #[doc = "Less than `x`."]
    lt => Lt,
    #[doc = "Less than or equal to `x`."]
    lte => Lte,
    #[doc = "Greater than or equal to `x`."]
    gte => Gte,
    #[doc = "Greater than `x`."]
    gt => Gt,
    #[doc = "Equal to `x`."]
    eq => Eq,
    #[doc = "Not equal to `x`."]
    neq => Neq,
}
