//! This module provides a defenition of a Magic: The Gathering card, as well as, ways to fetch
//! them from scryfall.
//!
//! All the card's fields are public and identic in name to the ones documented in the oficial
//! [scryfall page](https://scryfall.com/docs/api/cards).
mod border_color;
mod card_faces;
mod color;
mod frame;
mod frame_effect;
mod game;
mod layout;
mod legality;
mod preview;
mod price;
mod rarity;
mod related_card;

use crate::card_searcher::Search;
use crate::ruling::Ruling;
use crate::set::Set;
use crate::util::uri::{url_fetch, PaginatedURI, URI};
use crate::util::Uuid;
use crate::util::{API, API_CARDS};
#[doc(inline)]
pub use border_color::BorderColour;
#[doc(inline)]
pub use card_faces::CardFace;
#[doc(inline)]
pub use color::{Colour, Colours};
#[doc(inline)]
pub use frame::Frame;
#[doc(inline)]
pub use frame_effect::FrameEffect;
#[doc(inline)]
pub use game::Game;
#[doc(inline)]
pub use layout::Layout;
#[doc(inline)]
pub use legality::Legality;
#[doc(inline)]
pub use preview::Preview;
#[doc(inline)]
pub use price::Price;
#[doc(inline)]
pub use rarity::Rarity;
#[doc(inline)]
pub use related_card::RelatedCard;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use std::collections::hash_map::HashMap;

/// A Card object containing all fields that `scryfall` provides,
///
/// For documentation on each field please refer to their
/// [documentation](https://scryfall.com/docs/api/cards)
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct Card {
    // Core card fields
    pub arena_id: Option<usize>,
    pub id: Uuid,
    pub lang: String,
    pub mtgo_id: Option<usize>,
    pub mtgo_foil_id: Option<usize>,
    pub multiverse_ids: Option<Vec<usize>>,
    pub tcgplayer_id: Option<usize>,
    pub oracle_id: Uuid,
    pub prints_search_uri: PaginatedURI<Card>,
    pub rulings_uri: URI<Vec<Ruling>>,
    pub scryfall_uri: String,
    pub uri: URI<Card>,
    // Gameplay Fields
    pub card_faces: Option<Vec<CardFace>>,
    pub all_parts: Option<Vec<RelatedCard>>,
    pub cmc: f32,
    #[serde(default)]
    pub colors: Vec<Colour>,
    pub color_identity: Vec<Colour>,
    pub color_indicator: Option<Vec<Colour>>,
    pub edhrec_rank: Option<usize>,
    pub foil: bool,
    pub hand_modifier: Option<String>,
    pub layout: Layout,
    pub legalities: HashMap<String, Legality>,
    pub life_modifier: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub nonfoil: bool,
    pub oracle_text: Option<String>,
    pub oversized: bool,
    pub power: Option<String>,
    pub reserved: bool,
    pub toughness: Option<String>,
    #[serde(default)]
    pub type_line: Option<String>,
    // Print Fields
    pub artist: Option<String>,
    pub border_color: BorderColour,
    pub collector_number: String,
    pub digital: bool,
    pub flavor_text: Option<String>,
    #[serde(default)]
    pub frame_effects: Vec<FrameEffect>,
    pub frame: Frame,
    pub full_art: bool,
    pub games: Vec<Game>,
    pub highres_image: bool,
    pub illustration_id: Option<Uuid>,
    pub image_uris: Option<HashMap<String, String>>,
    #[serde(default)]
    pub prices: Price,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub promo: bool,
    #[serde(default)]
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
    #[serde(default)]
    pub preview: Preview,
}

impl Card {
    /// Returns a [`PaginatedURI`] of all the cards in the `scryfall` database.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::all().next().unwrap() {
    ///     Ok(cards) => assert_ne!(cards.len(), 0),
    ///     Err(e) => eprintln!("{:?}", e)
    /// }
    /// ```
    /// [`PaginatedURI`]: ../util/uri/struct.PaginatedURI.html
    #[deprecated(
        since = "0.6.0",
        note = "Scryfall is deprecating this endpoint on the 30/May/2020 in favour of the bulk endpoints"
    )]
    pub fn all() -> PaginatedURI<Card> {
        let cards = format!("{}/{}?page=1", API, API_CARDS);
        PaginatedURI::new(URI::from(cards))
    }

    /// Fetches a random card.
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

    /// Returns a [`PaginatedURI`] of the cards that match the search terms.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// assert!(Card::search("lightning")
    ///     .filter_map(Result::ok)
    ///     .flatten()
    ///     .all(|x| x.name.to_lowercase().contains("lightning")))
    /// ```
    /// ```rust
    /// use scryfall::card::Card;
    /// use scryfall::card_searcher::{
    ///     NumericParam::CollectorNumber, Search, SearchBuilder, StringParam::Set,
    /// };
    /// use scryfall::set::SetCode;
    /// use std::convert::TryFrom;
    ///
    /// assert!(SearchBuilder::new()
    ///     .param(CollectorNumber(123))
    ///     .param(Set(SetCode::try_from("war").expect("Not a valid set code")))
    ///     .search()
    ///     .all(|x| x.map(|x| x[0].name == "Demolish").unwrap_or(false)))
    /// ```
    /// ```rust
    /// use scryfall::card::Card;
    /// use scryfall::card_searcher::{
    ///     ComparisonExpr, Search, SearchBuilder, StringParam,
    /// };
    /// use scryfall::error::Error;
    ///
    /// let error = SearchBuilder::new()
    ///     .param(StringParam::Power(ComparisonExpr::AtLeast, "pow".to_string()))
    ///     .search()
    ///     .find_map(Result::err);
    /// match error {
    ///     Some(Error::ScryfallError(e)) => {
    ///             assert!(e.details.contains("All of your terms were ignored"));
    ///             assert!(e.warnings.len() > 0);
    ///         },
    ///     _ => ()
    /// };
    /// ```
    /// [`PaginatedURI`]: ../util/uri/struct.PaginatedURI.html
    pub fn search<S: Search>(query: S) -> PaginatedURI<Card> {
        let query = query.to_query().replace(" ", "+");
        let search = format!("{}/{}/search?{}", API, API_CARDS, query);
        PaginatedURI::new(URI::from(search))
    }

    /// Return a card with the exact name.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::named("Lightning Bolt") {
    ///     Ok(card) => assert_eq!(card.name, "Lightning Bolt"),
    ///     Err(e) => panic!(format!("{:?}", e)),
    /// }
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

    /// Return a card using the scryfall fuzzy finder.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::named_fuzzy("Light Bolt") {
    ///     Ok(card) => assert_eq!(card.name, "Lightning Bolt"),
    ///     Err(e) => panic!(format!("{:?}", e))
    /// }
    /// ```
    pub fn named_fuzzy(query: &str) -> crate::Result<Card> {
        let query = query.replace(" ", "+");
        let named = format!("{}/{}/named?fuzzy={}", API, API_CARDS, query);
        url_fetch(&named)
    }

    /// Fetch a card by it's multiverse id.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::multiverse(409574) {
    ///     Ok(card) => assert_eq!(card.name, "Strip Mine"),
    ///     Err(e) => panic!(format!("{:?}", e)),
    /// }
    /// ```
    pub fn multiverse(query: usize) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/multiverse/{}", API, API_CARDS, query))
    }

    /// Fetch a card by it's mtgo id.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::mtgo(54957) {
    ///     Ok(card) => assert_eq!(card.name, "Ghost Quarter"),
    ///     Err(e) => panic!(format!("{:?}", e)),
    /// }
    /// ```
    pub fn mtgo(query: usize) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/mtgo/{}", API, API_CARDS, query))
    }

    /// Fetch a card by it's arena id.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::arena(67330) {
    ///     Ok(card) => assert_eq!(card.name, "Yargle, Glutton of Urborg"),
    ///     Err(e) => panic!(format!("{:?}", e)),
    /// }
    /// ```
    pub fn arena(query: usize) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/arena/{}", API, API_CARDS, query))
    }

    /// Fetch a card by it's tcgplayer id.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::tcgplayer(67330) {
    ///     Ok(card) => assert_eq!(card.name, "Fathom Mage"),
    ///     Err(e) => panic!(format!("{:?}", e)),
    /// }
    /// ```
    pub fn tcgplayer(query: usize) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/tcgplayer/{}", API, API_CARDS, query))
    }

    /// Fetch a card by it's Uuid.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::card("0b81b329-4ef5-4b55-9fe7-9ed69477e96b".to_string()) {
    ///     Ok(card) => assert_eq!(card.name, "Cowed by Wisdom"),
    ///     Err(e) => panic!(format!("{:?}", e)),
    /// }
    /// ```
    pub fn card(query: Uuid) -> crate::Result<Card> {
        url_fetch(&format!("{}/{}/{}", API, API_CARDS, query))
    }
}
