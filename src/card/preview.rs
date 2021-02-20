//! Struct describing card preview information.
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;

/// Struct describing card preview information.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Preview {
    /// The date this card was previewed.
    pub previewed_at: Option<NaiveDate>,

    /// A link to the preview for this card.
    pub source_uri: Option<Url>,

    /// The name of the source that previewed this card.
    pub source: Option<String>,
}
