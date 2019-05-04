//! Module defining a ruling object.
use crate::util::UUID;

use serde::Deserialize;

use chrono::NaiveDate;

/// A ruling object.
#[derive(Debug, Deserialize, Clone)]
pub struct Ruling {
    oracle_id: UUID,
    source: String,
    published_at: NaiveDate,
    comment: String,
}
