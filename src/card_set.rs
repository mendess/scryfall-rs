mod set_type;

use super::card::Card;
use super::util::uri::{url_fetch, PaginatedURI, URI};
use super::util::UUID;
use super::util::{API, API_SETS};
use set_type::SetType;

use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CardSet {
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
    pub uri: URI<CardSet>,
    pub icon_svg_uri: String, //TODO: Revisit this
    pub search_uri: PaginatedURI<Card>,
}

#[allow(dead_code)]
impl CardSet {
    pub fn all() -> PaginatedURI<CardSet> {
        let sets = format!("{}/{}?page=1", API, API_SETS);
        PaginatedURI::new(URI::from(sets))
    }

    pub fn code(code: &str) -> crate::Result<CardSet> {
        url_fetch(&format!("{}/{}/{}", API, API_SETS, code))
    }

    pub fn tcgplayer(code: &str) -> crate::Result<CardSet> {
        url_fetch(&format!("{}/{}/tcgplayer/{}", API, API_SETS, code))
    }

    pub fn uuid(uuid: UUID) -> crate::Result<CardSet> {
        url_fetch(&format!("{}/{}/{}", API, API_SETS, uuid))
    }
}
