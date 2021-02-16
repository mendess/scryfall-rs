//! Struct describing card preview information.
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Struct describing card preview information.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[allow(missing_docs)]
pub struct Preview {
    pub previewed_at: Option<NaiveDate>,
    pub source_uri: Option<String>,
    pub source: Option<String>,
}
