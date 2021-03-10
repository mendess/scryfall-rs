use std::fmt;

use crate::search::param::value::{ParamValue, ValueKind};
use crate::search::param::Param;

/// An operator and RHS for a comparison expression of a parameter.
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

impl<T: 'static + ParamValue> ParamValue for Compare<T> {
    fn into_param(self, kind: ValueKind) -> Param {
        Param::cmp_value(kind, self.op, self.value)
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
            pub fn $meth<T>(x: T) -> Compare<T> {
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
