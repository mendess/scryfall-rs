//! This module provides a definition of a Magic: The Gathering card, as well
//! as, ways to fetch them from scryfall.
//!
//! All the card's fields are public and identical in name to the ones
//! documented in the official [scryfall page](https://scryfall.com/docs/api/cards).
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

use std::collections::hash_map::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub use self::border_color::BorderColor;
pub use self::card_faces::CardFace;
pub use self::color::{Color, Colors};
pub use self::frame::Frame;
pub use self::frame_effect::FrameEffect;
pub use self::game::Game;
pub use self::layout::Layout;
pub use self::legality::Legality;
pub use self::preview::Preview;
pub use self::price::Price;
pub use self::rarity::Rarity;
pub use self::related_card::RelatedCard;
use crate::card_searcher::Search;
use crate::list::{List, ListIter};
use crate::ruling::Ruling;
use crate::set::Set;
use crate::uri::Uri;
use crate::util::{Uuid, CARDS_URL};

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
    pub prints_search_uri: Uri<List<Card>>,
    pub rulings_uri: Uri<Vec<Ruling>>,
    pub scryfall_uri: String,
    pub uri: Uri<Card>,
    // Gameplay Fields
    pub card_faces: Option<Vec<CardFace>>,
    pub all_parts: Option<Vec<RelatedCard>>,
    pub cmc: f32,
    #[serde(default)]
    pub colors: Vec<Color>,
    pub color_identity: Vec<Color>,
    pub color_indicator: Option<Vec<Color>>,
    pub edhrec_rank: Option<usize>,
    pub foil: bool,
    pub hand_modifier: Option<String>,
    pub layout: Layout,
    // This map does not use the `Format` enum, since it would cause deserialization to fail when
    // new formats are added by Scryfall.
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
    pub border_color: BorderColor,
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
    pub set_search_uri: Uri<List<Card>>,
    pub set_uri: Uri<Set>,
    pub set: String,
    pub story_spotlight: bool,
    pub watermark: Option<String>,
    #[serde(default)]
    pub preview: Preview,
}

impl Card {
    /// Returns a [`ListIter`] of all the cards in the `scryfall` database.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use scryfall::card::Card;
    /// # #[allow(deprecated)]
    /// let cards = Card::all().unwrap().into_inner().collect::<Vec<_>>();
    /// assert!(cards.len() > 0);
    /// ```
    #[deprecated(
        since = "0.6.0",
        note = "Scryfall is deprecating this endpoint on the 30/May/2020 in favour of the bulk endpoints"
    )]
    pub fn all() -> crate::Result<ListIter<Card>> {
        let mut url = CARDS_URL.clone();
        url.query_pairs_mut().append_pair("page", "1");
        Uri::from(url).fetch_iter()
    }

    /// Fetches a random card.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::random() {
    ///     Ok(card) => println!("{}", card.name),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// ```
    pub fn random() -> crate::Result<Card> {
        Uri::from(CARDS_URL.join("random/")?).fetch()
    }

    /// Returns a [`ListIter`] of the cards that match the search terms.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// assert!(
    ///     Card::search("lightning")
    ///         .unwrap()
    ///         .filter_map(Result::ok)
    ///         .all(|x| x.name.to_lowercase().contains("lightning"))
    /// )
    /// ```
    /// ```rust
    /// use std::convert::TryFrom;
    ///
    /// use scryfall::card::Card;
    /// use scryfall::card_searcher::NumericParam::CollectorNumber;
    /// use scryfall::card_searcher::StringParam::Set;
    /// use scryfall::card_searcher::{Search, SearchBuilder};
    /// use scryfall::set::SetCode;
    ///
    /// assert!(
    ///     SearchBuilder::new()
    ///         .param(CollectorNumber(123))
    ///         .param(Set(SetCode::try_from("war").expect("Not a valid set code")))
    ///         .search()
    ///         .unwrap()
    ///         .all(|x| x.map(|x| x.name == "Demolish").unwrap_or(false))
    /// )
    /// ```
    /// ```rust
    /// use scryfall::card::Card;
    /// use scryfall::card_searcher::{ComparisonExpr, Search, SearchBuilder, StringParam};
    /// use scryfall::error::Error;
    ///
    /// let error = SearchBuilder::new()
    ///     .param(StringParam::Power(
    ///         ComparisonExpr::AtLeast,
    ///         "pow".to_string(),
    ///     ))
    ///     .search()
    ///     .unwrap_err();
    ///
    /// match error {
    ///     Error::ScryfallError(e) => {
    ///         assert!(e.details.contains("All of your terms were ignored"));
    ///         assert!(e.warnings.len() > 0);
    ///     },
    ///     other => panic!("Wrong error type: {0} {0:?}", other),
    /// };
    /// ```
    pub fn search<S: Search>(query: S) -> crate::Result<ListIter<Card>> {
        let mut url = CARDS_URL.join("search/").unwrap();
        url.set_query(Some(&query.to_query()));
        Uri::from(url).fetch_iter()
    }

    /// Return a card with the exact name.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::named("Lightning Bolt") {
    ///     Ok(card) => assert_eq!(card.name, "Lightning Bolt"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// ```
    ///
    /// ```rust
    /// # use scryfall::card::Card;
    /// use scryfall::error::Error;
    /// assert!(Card::named("Name that doesn't exist").is_err())
    /// ```
    pub fn named(name: &str) -> crate::Result<Card> {
        let mut url = CARDS_URL.join("named")?;
        url.query_pairs_mut().append_pair("exact", name);
        Uri::from(url).fetch()
    }

    /// Return a card using the scryfall fuzzy finder.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::named_fuzzy("Light Bolt") {
    ///     Ok(card) => assert_eq!(card.name, "Lightning Bolt"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// ```
    pub fn named_fuzzy(query: &str) -> crate::Result<Card> {
        let mut url = CARDS_URL.join("named")?;
        url.query_pairs_mut().append_pair("fuzzy", query);
        Uri::from(url).fetch()
    }

    /// Fetch a card by its set and number.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::set_and_number("vma", 4) {
    ///     Ok(card) => assert_eq!(card.name, "Black Lotus"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// ```
    pub fn set_and_number(set_code: &str, number: usize) -> crate::Result<Card> {
        Uri::from(CARDS_URL.join(&format!("{}/{}", set_code, number))?).fetch()
    }

    /// Fetch a card by its multiverse id.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::multiverse(409574) {
    ///     Ok(card) => assert_eq!(card.name, "Strip Mine"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// ```
    pub fn multiverse(multiverse_id: usize) -> crate::Result<Card> {
        Uri::from(
            CARDS_URL
                .join("multiverse/")?
                .join(&multiverse_id.to_string())?,
        )
        .fetch()
    }

    /// Fetch a card by its mtgo id.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::mtgo(54957) {
    ///     Ok(card) => assert_eq!(card.name, "Ghost Quarter"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// ```
    pub fn mtgo(mtgo_id: usize) -> crate::Result<Card> {
        Uri::from(CARDS_URL.join("mtgo/")?.join(&mtgo_id.to_string())?).fetch()
    }

    /// Fetch a card by its arena id.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::arena(67330) {
    ///     Ok(card) => assert_eq!(card.name, "Yargle, Glutton of Urborg"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// ```
    pub fn arena(arena_id: usize) -> crate::Result<Card> {
        Uri::from(CARDS_URL.join("arena/")?.join(&arena_id.to_string())?).fetch()
    }

    /// Fetch a card by its tcgplayer id.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::tcgplayer(67330) {
    ///     Ok(card) => assert_eq!(card.name, "Fathom Mage"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// ```
    pub fn tcgplayer(tcgplayer_id: usize) -> crate::Result<Card> {
        Uri::from(
            CARDS_URL
                .join("tcgplayer/")?
                .join(&tcgplayer_id.to_string())?,
        )
        .fetch()
    }

    /// Fetch a card by its Uuid.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// match Card::card("0b81b329-4ef5-4b55-9fe7-9ed69477e96b".parse().unwrap()) {
    ///     Ok(card) => assert_eq!(card.name, "Cowed by Wisdom"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// ```
    pub fn card(scryfall_id: Uuid) -> crate::Result<Card> {
        Uri::from(CARDS_URL.join(&scryfall_id.to_string())?).fetch()
    }
}
