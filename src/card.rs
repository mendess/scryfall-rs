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
use url::Url;
use uuid::Uuid;

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
use crate::format::Format;
use crate::list::{List, ListIter};
use crate::ruling::Ruling;
use crate::set::{Set, SetCode, SetType};
use crate::uri::Uri;
use crate::util::CARDS_URL;

/// Card objects represent individual Magic: The Gathering cards that players
/// could obtain and add to their collection (with a few minor exceptions).
///
/// ## Card Names
/// Internally, Scryfall tracks the uniqueness of “Oracle names.” (i.e. names
/// you can pick when an effect asks you to “choose a card name”). Each unique
/// Oracle name is separately available in the card names catalog.
///
/// Note that while most Oracle card names are unique, Scryfall also indexes
/// other objects such as tokens and Unstable set variants which do not always
/// have a unique name.
///
/// ## Multiface Cards
/// Magic cards can have multiple faces. The faces could be shown divided on the
/// front of the card as in split cards and flip cards, or the card can be
/// double-sided as in transform cards and double-sided tokens.
///
/// Scryfall represents multiface cards as a single object with a card_faces
/// array describing the distinct faces.
///
/// ---
///
/// For more details, see the [official documentation](https://scryfall.com/docs/api/cards).
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Card {
    // region Core Card Fields
    // =======================
    /// This card’s Arena ID, if any. A large percentage of cards are not
    /// available on Arena and do not have this ID.
    pub arena_id: Option<usize>,

    /// A unique ID for this card in Scryfall’s database.
    pub id: Uuid,

    // TODO(msmorgan): Language enum? https://scryfall.com/docs/api/languages
    /// A language code for this printing.
    pub lang: String,

    /// This card’s Magic Online ID (also known as the Catalog ID), if any. A
    /// large percentage of cards are not available on Magic Online and do not
    /// have this ID.
    pub mtgo_id: Option<usize>,

    /// This card’s foil Magic Online ID (also known as the Catalog ID), if any.
    /// A large percentage of cards are not available on Magic Online and do not
    /// have this ID.
    pub mtgo_foil_id: Option<usize>,

    /// This card’s multiverse IDs on Gatherer, if any, as an array of integers.
    /// Note that Scryfall includes many promo cards, tokens, and other esoteric
    /// objects that do not have these identifiers.
    pub multiverse_ids: Option<Vec<usize>>,

    /// This card’s ID on TCGplayer’s API, also known as the `productId`.
    pub tcgplayer_id: Option<usize>,

    /// This card’s ID on Cardmarket’s API, also known as the `idProduct`.
    pub cardmarket_id: Option<usize>,

    /// A unique ID for this card’s oracle identity. This value is consistent
    /// across reprinted card editions, and unique among different cards with
    /// the same name (tokens, Unstable variants, etc).
    pub oracle_id: Uuid,

    /// A link to where you can begin paginating all re/prints for this card on
    /// Scryfall’s API.
    pub prints_search_uri: Uri<List<Card>>,

    /// A link to this card’s rulings list on Scryfall’s API.
    pub rulings_uri: Uri<Vec<Ruling>>,

    /// A link to this card’s permapage on Scryfall’s website.
    pub scryfall_uri: Url,

    /// A link to this card object on Scryfall’s API.
    pub uri: Uri<Card>,
    // ==========================
    // endregion Core Card Fields
    //
    // region Gameplay Fields
    // ======================
    /// If this card is closely related to other cards, this property will be an
    /// array with Related Card Objects.
    pub all_parts: Option<Vec<RelatedCard>>,

    /// An array of Card Face objects, if this card is multifaced.
    pub card_faces: Option<Vec<CardFace>>,

    /// The card’s converted mana cost. Note that some funny cards have
    /// fractional mana costs.
    pub cmc: f32,

    /// This card’s color identity.
    pub color_identity: Vec<Color>,

    /// The colors in this card’s color indicator, if any. A null value for this
    /// field indicates the card does not have one.
    pub color_indicator: Option<Vec<Color>>,

    /// This card’s colors, if the overall card has colors defined by the rules.
    /// Otherwise the colors will be on the card_faces objects, see below.
    pub colors: Option<Vec<Color>>,

    /// This card’s overall rank/popularity on EDHREC. Not all cards are ranked.
    pub edhrec_rank: Option<usize>,

    /// True if this printing exists in a foil version.
    pub foil: bool,

    /// This card’s hand modifier, if it is Vanguard card. This value will
    /// contain a delta, such as -1.
    pub hand_modifier: Option<String>,

    /// An array of keywords that this card uses, such as 'Flying' and
    /// 'Cumulative upkeep'.
    pub keywords: Vec<String>,

    /// A code for this card’s layout.
    pub layout: Layout,

    /// An object describing the legality of this card across play formats.
    /// Possible legalities are legal, not_legal, restricted, and banned.
    pub legalities: HashMap<Format, Legality>,

    /// This card’s life modifier, if it is Vanguard card. This value will
    /// contain a delta, such as +2.
    pub life_modifier: Option<String>,

    /// This loyalty if any. Note that some cards have loyalties that are not
    /// numeric, such as X.
    pub loyalty: Option<String>,

    /// The mana cost for this card. This value will be any empty string "" if
    /// the cost is absent. Remember that per the game rules, a missing mana
    /// cost and a mana cost of {0} are different values. Multi-faced cards will
    /// report this value in card faces.
    pub mana_cost: Option<String>,

    /// The name of this card. If this card has multiple faces, this field will
    /// contain both names separated by ` // `.
    pub name: String,

    /// True if this printing exists in a nonfoil version.
    pub nonfoil: bool,

    /// The Oracle text for this card, if any.
    pub oracle_text: Option<String>,

    /// True if this card is oversized.
    pub oversized: bool,

    /// This card’s power, if any. Note that some cards have powers that are not
    /// numeric, such as *.
    pub power: Option<String>,

    /// Colors of mana that this card could produce.
    pub produced_mana: Option<Vec<Color>>,

    /// True if this card is on the Reserved List.
    pub reserved: bool,

    /// This card’s toughness, if any. Note that some cards have toughnesses
    /// that are not numeric, such as *.
    pub toughness: Option<String>,

    /// The type line of this card.
    pub type_line: String,
    // =========================
    // endregion Gameplay Fields
    //
    // region Print Fields
    // ===================
    /// The name of the illustrator of this card. Newly spoiled cards may not
    /// have this field yet.
    pub artist: Option<String>,

    /// Whether this card is found in boosters.
    pub booster: bool,

    /// This card’s border color: black, borderless, gold, silver, or white.
    pub border_color: BorderColor,

    /// The Scryfall ID for the card back design present on this card.
    pub card_back_id: Uuid,

    /// This card’s collector number. Note that collector numbers can contain
    /// non-numeric characters, such as letters or `★`.
    pub collector_number: String,

    /// True if you should consider avoiding use of this print downstream.
    #[serde(default)]
    pub content_warning: bool,

    /// True if this card was only released in a video game.
    pub digital: bool,

    /// The just-for-fun name printed on the card (such as for Godzilla series
    /// cards).
    pub flavor_name: Option<String>,

    /// The flavor text, if any.
    pub flavor_text: Option<String>,

    /// This card’s frame effects, if any.
    #[serde(default)]
    pub frame_effects: Vec<FrameEffect>,

    /// This card’s frame layout.
    pub frame: Frame,

    /// True if this card’s artwork is larger than normal.
    pub full_art: bool,

    /// A list of games that this card print is available in, paper, arena,
    /// and/or mtgo.
    pub games: Vec<Game>,

    /// True if this card’s imagery is high resolution.
    pub highres_image: bool,

    /// A unique identifier for the card artwork that remains consistent across
    /// reprints. Newly spoiled cards may not have this field yet.
    pub illustration_id: Option<Uuid>,

    /// An object listing available imagery for this card. See the [Card Imagery](https://scryfall.com/docs/api/images) article for more information.
    #[serde(default)]
    pub image_uris: HashMap<String, Url>,

    /// An object containing daily price information for this card, including
    /// `usd`, `usd_foil`, `eur`, and `tix` prices, as strings.
    #[serde(default)]
    pub prices: Price,

    /// The localized name printed on this card, if any.
    pub printed_name: Option<String>,

    /// The localized text printed on this card, if any.
    pub printed_text: Option<String>,

    /// The localized type line printed on this card, if any.
    pub printed_type_line: Option<String>,

    /// True if this card is a promotional print.
    pub promo: bool,

    // TODO(msmorgan): PurchaseUris struct?
    /// An object providing URIs to this card’s listing on major marketplaces.
    #[serde(default)]
    pub purchase_uris: HashMap<String, String>,

    /// This card’s rarity. One of `common`, `uncommon`, `rare`, or `mythic`.
    pub rarity: Rarity,

    // TODO(msmorgan): RelatedUris struct?
    /// An object providing URIs to this card’s listing on other Magic: The
    /// Gathering online resources.
    pub related_uris: HashMap<String, String>,

    /// The date this card was first released.
    pub released_at: NaiveDate,

    /// True if this card is a reprint.
    pub reprint: bool,

    /// A link to this card’s set on Scryfall’s website.
    pub scryfall_set_uri: String,

    /// This card’s full set name.
    pub set_name: String,

    /// A link to where you can begin paginating this card’s set on the Scryfall
    /// API.
    pub set_search_uri: Uri<List<Card>>,

    /// The type of set this printing is in.
    pub set_type: SetType,

    /// A link to this card’s set object on Scryfall’s API.
    pub set_uri: Uri<Set>,

    /// This card’s set code.
    pub set: SetCode,

    /// True if this card is a Story Spotlight.
    pub story_spotlight: bool,

    /// True if the card is printed without text.
    pub textless: bool,

    /// Whether this card is a variation of another printing.
    pub variation: bool,

    /// The printing ID of the printing this card is a variation of.
    pub variation_of: Option<Uuid>,

    /// This card’s watermark, if any.
    pub watermark: Option<String>,

    /// Information about when and where the card was originally previewed.
    #[serde(default)]
    pub preview: Preview,
    /* ======================
     * endregion Print Fields */
}

impl Card {
    /// Fetches a random card.
    ///
    /// # Examples
    /// ```rust
    /// # use scryfall::card::Card;
    /// # fn main() -> scryfall::Result<()> {
    /// let card = Card::random()?;
    /// println!("{}", &card.name);
    /// # Ok(())
    /// # }
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
    ///         .map(Result::unwrap)
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
    ///         .map(Result::unwrap)
    ///         .all(|card| card.name == "Demolish")
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
    pub fn search(query: impl Search) -> crate::Result<ListIter<Card>> {
        let mut url = CARDS_URL.join("search/")?;
        url.set_query(Some(&query.to_query()));
        Uri::from(url).fetch_iter()
    }

    /// Fetches a random card matching a search query.
    ///
    /// # Examples
    /// ```rust
    /// # use scryfall::Card;
    /// # fn main() -> scryfall::Result<()> {
    /// let card = Card::search_random("t:Merfolk")?;
    /// assert!(card.type_line.contains("Merfolk"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn search_random(query: impl Search) -> crate::Result<Card> {
        let mut url = CARDS_URL.join("random/")?;
        url.set_query(Some(&query.to_query()));
        Uri::from(url).fetch()
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
