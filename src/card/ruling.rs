use crate::util::UUID;

use serde::Deserialize;

use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
pub struct Ruling {
    oracle_id: UUID,
    source: String,
    published_at: NaiveDate,
    comment: String,
}
