//! Enum describing the 4 states of legality a card can have.
use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

/// Enum describing the 4 states of legality a card can have.
#[derive(Default, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Legality {
    Legal,
    #[default]
    NotLegal,
    Restricted,
    Banned,
}

impl PartialOrd for Legality {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Legality::NotLegal, _) | (_, Legality::NotLegal) => None,
            (a, b) if a == b => Some(Ordering::Equal),
            (Legality::Legal, _) => Some(Ordering::Greater),
            (_, Legality::Legal) => Some(Ordering::Less),
            (Legality::Restricted, Legality::Banned) => Some(Ordering::Greater),
            (Legality::Banned, Legality::Restricted) => Some(Ordering::Less),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_legalities() {
        use Legality::*;
        use Ordering::*;

        let matrix = vec![
            ((Legal, Legal), Some(Equal)),
            ((Legal, NotLegal), None),
            ((Legal, Restricted), Some(Greater)),
            ((Legal, Banned), Some(Greater)),
            ((NotLegal, Legal), None),
            ((NotLegal, NotLegal), None),
            ((NotLegal, Restricted), None),
            ((NotLegal, Banned), None),
            ((Restricted, Legal), Some(Less)),
            ((Restricted, NotLegal), None),
            ((Restricted, Restricted), Some(Equal)),
            ((Restricted, Banned), Some(Greater)),
            ((Banned, Legal), Some(Less)),
            ((Banned, NotLegal), None),
            ((Banned, Restricted), Some(Less)),
            ((Banned, Banned), Some(Equal)),
        ];

        for ((a, b), order) in &matrix {
            assert_eq!(&a.partial_cmp(b), order);
        }
    }
}
