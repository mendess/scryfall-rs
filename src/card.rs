//! This module provides a defenition of a Magic: The Gathering card, as well as, ways to fetch
//! them from scryfall.
//!
//! All the card's fields are public and identic in name to the ones documented in the oficial [scryfall page](https://scryfall.com/docs/api/cards).
pub mod border_color;
pub mod card_faces;
pub mod color;
pub mod frame;
pub mod frame_effect;
pub mod game;
pub mod layout;
pub mod legality;
pub mod price;
pub mod rarity;
pub mod related_card;

use crate::card_searcher::Search;
use crate::ruling::Ruling;
use crate::set::Set;
use crate::util::uri::{url_fetch, PaginatedURI, URI};
use crate::util::UUID;
use crate::util::{API, API_CARDS};
pub use border_color::BorderColor;
pub use card_faces::CardFace;
pub use color::Color;
pub use frame::Frame;
pub use frame_effect::FrameEffect;
pub use game::Game;
pub use layout::Layout;
pub use legality::Legality;
pub use price::Price;
pub use rarity::Rarity;
pub use related_card::RelatedCard;

use chrono::NaiveDate;
use serde::Deserialize;

use std::collections::hash_map::HashMap;

/// A Card object containing all fields that `scryfall` provides,
///
/// For documentation on each field please refer to their
/// [documentation](https://scryfall.com/docs/api/cards)
#[derive(Deserialize, Debug, Clone)]
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
    pub set_uri: URI<Set>,
    pub set: String,
    pub story_spotlight: bool,
    pub watermark: Option<String>,
}

impl Card {
    /// Returns a `PaginatedURI` of all the cards in the `scryfall` database.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::all().next().unwrap() {
    ///     Ok(cards) => assert_ne!(cards.len(), 0),
    ///     Err(e) => eprintln!("{:?}", e)
    /// }
    /// ```
    pub fn all() -> PaginatedURI<Card> {
        let cards = format!("{}/{}?page=1", API, API_CARDS);
        PaginatedURI::new(URI::from(cards))
    }

    /// Fetches a random card
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::random() {
    ///     Ok(card) => println!("{}", card.name),
    ///     Err(e) => eprintln!("{:?}", e)
    /// }
    /// ```
    pub fn random() -> crate::Result<Card> {
        url_fetch("https://api.scryfall.com/cards/random")
    }

    /// Returns a `PaginatedURI` of the cards that match the search terms.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::search("Jace").next().unwrap() {
    ///     Ok(cards) => assert_ne!(cards.len(), 0),
    ///     Err(e) => eprintln!("{:?}", e)
    /// }
    /// assert!(Card::search("lightning")
    ///     .filter_map(|x| x.ok())
    ///     .flatten()
    ///     .all(|x| x.name.to_lowercase().contains("lightning")))
    /// ```
    /// ```rust
    /// use scryfall::card_searcher::{SearchBuilder, NumericParam::CollectorNumber};
    /// ```
    pub fn search<S: Search>(query: S) -> PaginatedURI<Card> {
        let query = query.to_query().replace(" ", "+");
        let search = format!("{}/{}/search?{}", API, API_CARDS, query);
        PaginatedURI::new(URI::from(search))
    }

    /// Return a card with the exact name
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// assert_eq!(Card::named("Lightning Bolt").unwrap().name, "Lightning Bolt")
    /// ```
    ///
    /// ```rust
    /// # use scryfall::card::Card;
    /// use scryfall::error::Error;
    /// assert!(Card::named("Name that doesn't exist").is_err())
    /// ```
    pub fn named(query: &str) -> crate::Result<Card> {
        let query = query.replace(" ", "+");
        let named = format!("{}/{}/named?exact={}", API, API_CARDS, query);
        url_fetch(&named)
    }

    /// Return a card using the scryfall fuzzy finder
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// assert_eq!(Card::named_fuzzy("Light Bolt").unwrap().name, "Lightning Bolt")
    /// ```
    pub fn named_fuzzy(query: &str) -> crate::Result<Card> {
        let query = query.replace(" ", "+");
        let named = format!("{}/{}/named?fuzzy={}", API, API_CARDS, query);
        url_fetch(&named)
    }

    /// Fetch a card by it's multiverse id
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// assert_eq!(Card::multiverse(409574).unwrap().name, "Strip Mine")
    /// ```
    pub fn multiverse(query: usize) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/multiverse/{}", API, API_CARDS, query))
    }

    /// Fetch a card by it's mtgo id
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// assert_eq!(Card::mtgo(54957).unwrap().name, "Ghost Quarter")
    /// ```
    pub fn mtgo(query: usize) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/mtgo/{}", API, API_CARDS, query))
    }

    /// Fetch a card by it's arena id
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// assert_eq!(Card::arena(67330).unwrap().name, "Yargle, Glutton of Urborg")
    /// ```
    pub fn arena(query: usize) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/arena/{}", API, API_CARDS, query))
    }

    /// Fetch a card by it's tcgplayer id
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// assert_eq!(Card::tcgplayer(67330).unwrap().name, "Fathom Mage")
    /// ```
    pub fn tcgplayer(query: usize) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/tcgplayer/{}", API, API_CARDS, query))
    }

    /// Fetch a card by it's UUID
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// assert_eq!(
    ///     Card::card("0b81b329-4ef5-4b55-9fe7-9ed69477e96b".to_string()).unwrap().name,
    ///     "Cowed by Wisdom")
    /// ```
    pub fn card(query: UUID) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/{}", API, API_CARDS, query))
    }
}
