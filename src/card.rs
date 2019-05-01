mod border_color;
mod color;
mod frame;
mod game;
mod layout;
mod legality;
mod price;
mod rarity;
mod ruling;

const API: &str = "https://api.scryfall.com";
const API_CARDS: &str = "/cards";

use super::card_set::CardSet;
use super::util::uri::{url_fetch, URI};
use super::util::UUID;
use border_color::BorderColor;
use color::Color;
use frame::Frame;
use game::Game;
use layout::Layout;
use legality::Legality;
use price::Price;
use rarity::Rarity;
use ruling::Ruling;

use serde::Deserialize;

use std::collections::hash_map::HashMap;

pub type CardResult<T> = Result<T, CardError>;

#[derive(Debug)]
pub enum CardError {
    JsonError(serde_json::Error),
    ReqwestError(reqwest::Error),
    Other(String),
}

impl From<serde_json::Error> for CardError {
    fn from(error: serde_json::Error) -> Self {
        CardError::JsonError(error)
    }
}

impl From<reqwest::Error> for CardError {
    fn from(error: reqwest::Error) -> Self {
        CardError::ReqwestError(error)
    }
}

#[derive(Deserialize, Debug)]
pub struct Card {
    // Core card fields
    pub id: UUID,
    pub lang: String,
    pub oracle_id: UUID,
    pub prints_search_uri: URI<Vec<Card>>,
    pub rulings_uri: URI<Vec<Ruling>>,
    pub scryfall_uri: String,
    pub uri: URI<Card>,
    // Gameplay Fields
    pub cmc: f32,
    pub colors: Option<Vec<Color>>,
    pub foil: bool,
    pub layout: Layout,
    pub legalities: HashMap<String, Legality>,
    pub name: String,
    pub nonfoil: bool,
    pub oracle_text: Option<String>,
    pub oversized: bool,
    pub reserved: bool,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub type_line: String,
    // Print Fields
    pub border_color: BorderColor,
    pub collector_number: String,
    pub digital: bool,
    pub frame: Frame,
    pub full_art: bool,
    pub games: Vec<Game>,
    pub highres_image: bool,
    pub prices: Price,
    pub promo: bool,
    pub purchase_uris: HashMap<String, String>,
    pub rarity: Rarity,
    pub related_uris: HashMap<String, String>,
    pub released_at: String, // TODO: Change to date
    pub reprint: bool,
    pub scryfall_set_uri: String,
    pub set_name: String,
    pub set_search_uri: Cards,
    pub set_uri: URI<CardSet>,
    pub set: String,
    pub story_spotlight: bool,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Cards {
    next: Option<URI<CardsJson>>,
}

#[derive(Deserialize, Debug)]
struct CardsJson {
    next_page: Option<URI<CardsJson>>,
    data: Vec<Card>,
}

impl Iterator for Cards {
    type Item = CardResult<Vec<Card>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(url) = self.next.take() {
            match url_fetch::<CardsJson>(&String::from(url)) {
                Ok(cards) => {
                    *self = Cards {
                        next: cards.next_page,
                    };
                    Some(Ok(cards.data))
                }
                Err(error) => Some(Err(error)),
            }
        } else {
            None
        }
    }
}

#[allow(dead_code)]
impl Card {
    pub fn all() -> Cards {
        let cards = format!("{}/{}?page=1", API, API_CARDS);
        Cards {
            next: Some(URI::from(cards)),
        }
    }

    pub fn random() -> CardResult<Self> {
        url_fetch("https://api.scryfall.com/cards/random")
    }

    pub fn search(query: &str) -> Cards {
        let query = query.replace(" ", "+");
        let search = format!("{}/{}/search?q={}", API, API_CARDS, query);
        Cards {
            next: Some(URI::from(search)),
        }
    }

    pub fn named(query: &str) -> CardResult<Card> {
        let query = query.replace(" ", "+");
        let named = format!("{}/{}/named?exact={}", API, API_CARDS, query);
        url_fetch(&named)
    }

    pub fn named_fuzzy(query: &str) -> CardResult<Card> {
        let query = query.replace(" ", "+");
        let named = format!("{}/{}/named?fuzzy={}", API, API_CARDS, query);
        url_fetch(&named)
    }

    pub fn multiverse(query: &str) -> CardResult<Card> {
        url_fetch(&format!("{}/{}/multiverse/{}", API, API_CARDS, query))
    }

    pub fn mtgo(query: &str) -> CardResult<Card> {
        url_fetch(&format!("{}/{}/mtgo/{}", API, API_CARDS, query))
    }

    pub fn arena(query: &str) -> CardResult<Card> {
        url_fetch(&format!("{}/{}/arena/{}", API, API_CARDS, query))
    }

    pub fn tcgplayer(query: &str) -> CardResult<Card> {
        url_fetch(&format!("{}/{}/tcgplayer/{}", API, API_CARDS, query))
    }

    pub fn card(query: &str) -> CardResult<Card> {
        url_fetch(&format!("{}/{}/{}", API, API_CARDS, query))
    }
}
