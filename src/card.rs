mod color;
mod layout;
mod legality;
mod border_color;
mod frame;
mod game;
mod price;
mod rarity;

const API : &'static str = "https://api.scryfall.com";
const API_CARDS : &'static str = "/cards";

use color::Color;
use layout::Layout;
use legality::Legality;
use border_color::BorderColor;
use frame::Frame;
use game::Game;
use price::Price;
use rarity::Rarity;

use serde::Deserialize;
use serde_json::from_reader;

use std::collections::hash_map::HashMap;

type UUID = String;
type URI = String;
type CardResult<T> = Result<T, CardError>;

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

#[allow(dead_code)] // TODO: Remove this
#[derive(Deserialize,Debug)]
pub struct Card {
    // Core card fields
    pub id: UUID,
    pub lang: String,
    pub oracle_id: UUID,
    pub prints_search_uri: URI,
    pub rulings_uri: URI,
    pub scryfall_uri: URI,
    pub uri: URI,
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
    pub purchase_uris: HashMap<String, URI>,
    pub rarity: Rarity,
    pub related_uris: HashMap<String, URI>,
    pub released_at: String, // TODO: Change to date
    pub reprint: bool,
    pub scryfall_set_uri: URI,
    pub set_name: String,
    pub set_search_uri: URI,
    pub set_uri: URI,
    pub set: String,
    pub story_spotlight: bool,
}

pub struct Cards {
    next: Option<URI>,
}

#[derive(Deserialize)]
struct CardsJson {
    next_page: Option<URI>,
    data: Vec<Card>,
}

impl Iterator for Cards {
    type Item = CardResult<Vec<Card>>;

    fn next(&mut self) -> Option<Self::Item> {
        eprintln!("next: {:?}", self.next);
        if self.next.is_some() {
            match url_fetch::<CardsJson>(&self.next.take().unwrap()) {
                Ok(cards) => {
                    *self = Cards { next: cards.next_page };
                    Some(Ok(cards.data))
                },
                Err(error) => return Some(Err(error)),
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
            next: Some(String::from(cards)),
        }
    }

    pub fn random() -> CardResult<Self> {
        url_fetch("https://api.scryfall.com/cards/random")
    }

    pub fn search(query: &str) -> Cards {
        let query = query.replace(" ", "+");
        let search = format!("{}/{}/search?q={}", API, API_CARDS, query);
        Cards {
            next: Some(String::from(search)),
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

fn url_fetch<T>(url: &str) -> CardResult<T>
where for<'de> T: Deserialize<'de>,{
    let resp = reqwest::get(url)?;
    if resp.status().is_success() {
        Ok(from_reader(resp)?)
    } else {
        Err(CardError::Other(format!("{:?}", resp.status())))
    }
}

