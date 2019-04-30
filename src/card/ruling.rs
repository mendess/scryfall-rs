use crate::util::UUID;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Ruling {
    oracle_id: UUID,
    source: String,
    published_at: String,
    comment: String,
}
