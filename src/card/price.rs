//! Module defining a price object containing data in various currencies.
use serde::{Deserialize, Serialize};

/// Struct defining a price object containing data in various currencies.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[allow(missing_docs)]
pub struct Price {
    pub usd: Option<String>,
    pub usd_foil: Option<String>,
    pub eur: Option<String>,
    pub tix: Option<String>,
}
