//! Module defining a price object containing data in various currencies.
use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

/// Struct defining a price object containing data in various currencies.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[allow(missing_docs)]
pub struct Price {
    pub usd: Option<String>,
    pub usd_foil: Option<String>,
    pub eur: Option<String>,
    pub eur_foil: Option<String>,
    pub tix: Option<String>,
}

impl Price {
    /// Creates an array of component prices that can be iterated over.
    fn to_array(&self) -> [&Option<String>; 5] {
        [
            &self.usd,
            &self.usd_foil,
            &self.eur,
            &self.eur_foil,
            &self.tix,
        ]
    }
}

/// Compares two prices as floating-point numbers.
fn compare_prices(a: &Option<String>, b: &Option<String>) -> Option<Ordering> {
    if let (Some(a), Some(b)) = (a, b) {
        if let (Ok(a), Ok(b)) = (a.parse::<f32>(), b.parse()) {
            return a.partial_cmp(&b);
        }
    }
    None
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut result = None;
        for (a, b) in self.to_array().iter().zip(other.to_array().iter()) {
            match (result, compare_prices(*a, *b)) {
                // If either ordering is `None`, use the other. Then if either is `Some(Equal)`,
                // use the other.
                (None, order)
                | (order, None)
                | (Some(Ordering::Equal), order)
                | (order, Some(Ordering::Equal)) => {
                    result = order;
                },
                // If the two orderings already agree, do nothing.
                (Some(a), Some(b)) if a == b => {},
                // Otherwise, they disagree, so these prices cannot be ordered.
                _ => return None,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_prices() {
        let a = Price::default();
        let b = Price::default();

        assert_eq!(a.partial_cmp(&b), None);
    }

    #[test]
    fn prices_agree() {
        let a = Price {
            usd: Some("5".to_string()),
            usd_foil: Some("8".to_string()),
            eur: Some("3".to_string()),
            ..Default::default()
        };
        let b = Price {
            usd: Some("10".to_string()),
            usd_foil: Some("14".to_string()),
            tix: Some("1".to_string()),
            ..Default::default()
        };

        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    }

    #[test]
    fn prices_disagree() {
        let a = Price {
            usd: Some("0.1".to_string()),
            tix: Some("15".to_string()),
            ..Default::default()
        };
        let b = Price {
            usd: Some("2".to_string()),
            tix: Some(".5".to_string()),
            ..Default::default()
        };

        assert_eq!(a.partial_cmp(&b), None);
    }

    #[test]
    fn prices_equal() {
        let a = Price {
            usd: Some("3.99".to_string()),
            tix: Some("2.1".to_string()),
            ..Default::default()
        };
        let b = Price {
            usd: Some("3.99".to_string()),
            eur: Some("4.20".to_string()),
            ..Default::default()
        };

        assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
    }
}
