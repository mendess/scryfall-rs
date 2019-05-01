mod border_color;
mod card_faces;
mod color;
mod frame;
mod frame_effect;
mod game;
mod layout;
mod legality;
mod price;
mod rarity;
mod related_card;
mod ruling;

use super::card_set::CardSet;
use super::util::uri::{url_fetch, PaginatedURI, URI};
use super::util::UUID;
use super::util::{API, API_CARDS};
use border_color::BorderColor;
use card_faces::CardFace;
use color::Color;
use frame::Frame;
use frame_effect::FrameEffect;
use game::Game;
use layout::Layout;
use legality::Legality;
use price::Price;
use rarity::Rarity;
use related_card::RelatedCard;
use ruling::Ruling;

use chrono::NaiveDate;
use serde::Deserialize;

use std::collections::hash_map::HashMap;

#[derive(Deserialize, Debug)]
pub struct Card {
    // Core card fields
    pub arena_id: Option<usize>,
    pub id: UUID,
    pub lang: String,
    pub mtgo_id: Option<usize>,
    pub mtgo_foil_id: Option<usize>,
    pub multiverse_ids: Option<Vec<usize>>,
    pub tcgplayer_id: Option<usize>,
    pub oracle_id: UUID,
    pub prints_search_uri: PaginatedURI<Card>,
    pub rulings_uri: URI<Vec<Ruling>>,
    pub scryfall_uri: String,
    pub uri: URI<Card>,
    // Gameplay Fields
    pub card_faces: Option<Vec<CardFace>>,
    pub all_parts: Option<Vec<RelatedCard>>,
    pub cmc: f32,
    pub colors: Option<Vec<Color>>,
    pub color_identity: Vec<Color>,
    pub color_indicator: Option<Vec<Color>>,
    pub edhrec_rank: Option<usize>,
    pub foil: bool,
    pub hand_modifier: Option<i32>,
    pub layout: Layout,
    pub legalities: HashMap<String, Legality>,
    pub life_modifier: Option<i32>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub nonfoil: bool,
    pub oracle_text: Option<String>,
    pub oversized: bool,
    pub power: Option<String>,
    pub reserved: bool,
    pub toughness: Option<String>,
    pub type_line: String,
    // Print Fields
    pub artist: Option<String>,
    pub border_color: BorderColor,
    pub collector_number: String,
    pub digital: bool,
    pub flavor_text: Option<String>,
    pub frame_effect: Option<FrameEffect>,
    pub frame: Frame,
    pub full_art: bool,
    pub games: Vec<Game>,
    pub highres_image: bool,
    pub illustration_id: Option<UUID>,
    pub image_uris: Option<HashMap<String, String>>,
    pub prices: Price,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub promo: bool,
    pub purchase_uris: HashMap<String, String>,
    pub rarity: Rarity,
    pub related_uris: HashMap<String, String>,
    pub released_at: NaiveDate,
    pub reprint: bool,
    pub scryfall_set_uri: String,
    pub set_name: String,
    pub set_search_uri: PaginatedURI<Card>,
    pub set_uri: URI<CardSet>,
    pub set: String,
    pub story_spotlight: bool,
    pub watermark: Option<String>,
}

#[allow(dead_code)]
impl Card {
    pub fn all() -> PaginatedURI<Card> {
        let cards = format!("{}/{}?page=1", API, API_CARDS);
        PaginatedURI::new(URI::from(cards))
    }

    pub fn random() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/cards/random")
    }

    pub fn search(query: &str) -> PaginatedURI<Card> {
        let query = query.replace(" ", "+");
        let search = format!("{}/{}/search?q={}", API, API_CARDS, query);
        PaginatedURI::new(URI::from(search))
    }

    pub fn named(query: &str) -> crate::Result<Card> {
        let query = query.replace(" ", "+");
        let named = format!("{}/{}/named?exact={}", API, API_CARDS, query);
        url_fetch(&named)
    }

    pub fn named_fuzzy(query: &str) -> crate::Result<Card> {
        let query = query.replace(" ", "+");
        let named = format!("{}/{}/named?fuzzy={}", API, API_CARDS, query);
        url_fetch(&named)
    }

    pub fn multiverse(query: &str) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/multiverse/{}", API, API_CARDS, query))
    }

    pub fn mtgo(query: &str) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/mtgo/{}", API, API_CARDS, query))
    }

    pub fn arena(query: &str) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/arena/{}", API, API_CARDS, query))
    }

    pub fn tcgplayer(query: &str) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/tcgplayer/{}", API, API_CARDS, query))
    }

    pub fn card(query: &str) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/{}", API, API_CARDS, query))
    }
}
