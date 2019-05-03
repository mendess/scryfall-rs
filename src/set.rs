mod set_type;

use super::card::Card;
use super::util::uri::{url_fetch, PaginatedURI, URI};
use super::util::UUID;
use super::util::{API, API_SETS};
use set_type::SetType;

use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Set {
    pub id: UUID,
    pub code: String,
    pub name: String,
    pub set_type: SetType,
    pub released_at: Option<NaiveDate>,
    pub block_code: Option<String>,
    pub block: Option<String>,
    pub parent_set_code: Option<String>,
    pub card_count: usize,
    pub digital: bool,
    pub foil_only: bool,
    pub scryfall_uri: String,
    pub uri: URI<Set>,
    pub icon_svg_uri: String, //TODO: Revisit this
    pub search_uri: PaginatedURI<Card>,
}

#[allow(dead_code)]
impl Set {
    pub fn all() -> PaginatedURI<Set> {
        let sets = format!("{}/{}?page=1", API, API_SETS);
        PaginatedURI::new(URI::from(sets))
    }

    pub fn code(code: &str) -> crate::Result<Set> {
        url_fetch(&format!("{}/{}/{}", API, API_SETS, code))
    }

    pub fn tcgplayer(code: &str) -> crate::Result<Set> {
        url_fetch(&format!("{}/{}/tcgplayer/{}", API, API_SETS, code))
    }

    pub fn uuid(uuid: UUID) -> crate::Result<Set> {
        url_fetch(&format!("{}/{}/{}", API, API_SETS, uuid))
    }
}
