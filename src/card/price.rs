//! Module defining a price object containing data in various currencies.
use serde::Deserialize;

/// Struct defining a price object containing data in various currencies.
#[derive(Deserialize, Debug, Clone)]
#[allow(missing_docs)]
pub struct Price {
    pub usd: Option<String>,
    pub usd_foil: Option<String>,
    pub eur: Option<String>,
    pub tix: Option<String>,
}
