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
    ///
    /// NOTE: Sometimes this is an empty string, causing the `Url`
    /// deserialization to fail. If this happens, a `None` variant is used
    /// instead.
    #[serde(deserialize_with = "crate::util::deserialize_or_none")]
    pub source_uri: Option<Url>,

    /// The name of the source that previewed this card.
    pub source: Option<String>,
}
