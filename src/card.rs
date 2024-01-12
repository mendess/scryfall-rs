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
mod produced_mana;
mod rarity;
mod related_card;

use std::collections::hash_map::HashMap;
use std::ops::{Index, IndexMut};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

pub use self::border_color::BorderColor;
pub use self::card_faces::CardFace;
pub use self::color::{Color, Colors, Multicolored};
pub use self::frame::Frame;
pub use self::frame_effect::FrameEffect;
pub use self::game::Game;
pub use self::layout::Layout;
pub use self::legality::Legality;
pub use self::preview::Preview;
pub use self::price::Price;
pub use self::produced_mana::{ProducedMana, UnfinityMana};
pub use self::rarity::Rarity;
pub use self::related_card::RelatedCard;
use crate::format::Format;
use crate::list::{List, ListIter};
use crate::ruling::Ruling;
use crate::search::Search;
use crate::set::{Set, SetCode, SetType};
use crate::uri::Uri;
use crate::util::CARDS_URL;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(deny_unknown_fields)]
#[allow(missing_docs)]
pub struct CardLegality {
    pub standard: Legality,
    pub modern: Legality,
    pub legacy: Legality,
    pub vintage: Legality,
    pub commander: Legality,
    pub future: Legality,
    pub pauper: Legality,
    pub pioneer: Legality,
    pub penny: Legality,
    pub duel: Legality,
    #[serde(rename = "oldschool")]
    pub old_school: Legality,
    pub historic: Legality,
    pub gladiator: Legality,
    pub brawl: Legality,
    pub premodern: Legality,
    #[serde(rename = "historicbrawl")]
    pub historic_brawl: Legality,
    #[serde(rename = "paupercommander")]
    pub pauper_commander: Legality,
    pub alchemy: Legality,
    pub explorer: Legality,
    pub predh: Legality,
    pub oathbreaker: Legality,
    pub timeless: Legality,
}

impl Index<Format> for CardLegality {
    type Output = Legality;

    fn index(&self, index: Format) -> &Self::Output {
        match index {
            Format::Standard => &self.standard,
            Format::Modern => &self.modern,
            Format::Legacy => &self.legacy,
            Format::Vintage => &self.vintage,
            Format::Commander => &self.commander,
            Format::Future => &self.future,
            Format::Pauper => &self.pauper,
            Format::Pioneer => &self.pioneer,
            Format::Penny => &self.penny,
            Format::Duel => &self.duel,
            Format::OldSchool => &self.old_school,
            Format::Historic => &self.historic,
            Format::Gladiator => &self.gladiator,
            Format::Brawl => &self.brawl,
            Format::Premodern => &self.premodern,
            Format::HistoricBrawl => &self.historic_brawl,
            Format::PauperCommander => &self.pauper_commander,
            Format::Alchemy => &self.alchemy,
            Format::Explorer => &self.explorer,
            Format::Predh => &self.predh,
            Format::Oathbreaker => &self.oathbreaker,
            Format::Timeless => &self.timeless,
        }
    }
}

impl IndexMut<Format> for CardLegality {
    fn index_mut(&mut self, index: Format) -> &mut Self::Output {
        match index {
            Format::Standard => &mut self.standard,
            Format::Modern => &mut self.modern,
            Format::Legacy => &mut self.legacy,
            Format::Vintage => &mut self.vintage,
            Format::Commander => &mut self.commander,
            Format::Future => &mut self.future,
            Format::Pauper => &mut self.pauper,
            Format::Pioneer => &mut self.pioneer,
            Format::Penny => &mut self.penny,
            Format::Duel => &mut self.duel,
            Format::OldSchool => &mut self.old_school,
            Format::Historic => &mut self.historic,
            Format::Gladiator => &mut self.gladiator,
            Format::Brawl => &mut self.brawl,
            Format::Premodern => &mut self.premodern,
            Format::HistoricBrawl => &mut self.historic_brawl,
            Format::PauperCommander => &mut self.pauper_commander,
            Format::Alchemy => &mut self.alchemy,
            Format::Explorer => &mut self.explorer,
            Format::Predh => &mut self.predh,
            Format::Oathbreaker => &mut self.oathbreaker,
            Format::Timeless => &mut self.timeless,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
/// Scryfall produces multiple sizes of images and image crops for each Card object. Links to these
/// images are available in each Card objects’ image_uris properties.
///
/// Field         | Size       | Format | Example
///  ---          | ---        | ---    | ---
/// `png`         | 745 × 1040 | PNG    | [Example Image](https://cards.scryfall.io/png/front/6/d/6da045f8-6278-4c84-9d39-025adf0789c1.png?1562404626)
/// `border_crop` | 480 × 680  | JPG    | [Example Image](https://cards.scryfall.io/border_crop/front/6/d/6da045f8-6278-4c84-9d39-025adf0789c1.jpg?1562404626)
/// `art_crop`    | Varies     | JPG    | [Example Image](https://cards.scryfall.io/art_crop/front/6/d/6da045f8-6278-4c84-9d39-025adf0789c1.jpg?1562404626)
/// `large`       | 672 × 936  | JPG    | [Example Image](https://cards.scryfall.io/large/front/6/d/6da045f8-6278-4c84-9d39-025adf0789c1.jpg?1562404626)
/// `normal`      | 488 × 680  | JPG    | [Example Image](https://cards.scryfall.io/normal/front/6/d/6da045f8-6278-4c84-9d39-025adf0789c1.jpg?1562404626)
/// `small`       | 146 × 204  | JPG    | [Example Image](https://cards.scryfall.io/small/front/6/d/6da045f8-6278-4c84-9d39-025adf0789c1.jpg?1562404626)
pub struct ImageUris {
    /// A transparent, rounded full card PNG. This is the best image to use for videos or other
    /// high-quality content.
    pub png: Url,
    /// A full card image with the rounded corners and the majority of the border cropped off.
    /// Designed for dated contexts where rounded images can’t be used.
    pub border_crop: Url,
    /// A rectangular crop of the card’s art only. Not guaranteed to be perfect for cards with
    /// outlier designs or strange frame arrangements
    pub art_crop: Url,
    ///  A large full card image
    pub large: Url,
    /// A medium-sized full card image
    pub normal: Url,
    /// A small full card image. Designed for use as thumbnail or list icon.
    pub small: Url,
}

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
    pub oracle_id: Option<Uuid>,

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
    pub cmc: Option<f32>,

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
    pub legalities: CardLegality,

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
    pub produced_mana: Option<Vec<ProducedMana>>,

    /// True if this card is on the Reserved List.
    pub reserved: bool,

    /// This card’s toughness, if any. Note that some cards have toughnesses
    /// that are not numeric, such as *.
    pub toughness: Option<String>,

    /// The type line of this card.
    pub type_line: Option<String>,
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
    pub card_back_id: Option<Uuid>,

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
    pub image_uris: Option<ImageUris>,

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
    /// # tokio_test::block_on(async {
    /// let card = Card::random().await?;
    /// println!("{}", &card.name);
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn random() -> crate::Result<Card> {
        Uri::from(CARDS_URL.join("random/")?).fetch().await
    }

    /// Returns a [`ListIter`] of the cards that match the search terms.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// use futures::stream::{self, StreamExt};
    /// use futures::future;
    /// assert!(
    /// # tokio_test::block_on(async {
    ///     Card::search("lightning").await
    ///         .unwrap()
    ///         .into_stream()
    ///         .map(Result::unwrap)
    ///         .all(|x| future::ready(x.name.to_lowercase().contains("lightning")))
    ///         .await
    /// # })
    /// )
    /// ```
    ///
    /// ```rust
    /// use scryfall::card::Card;
    /// use futures::stream::{self, StreamExt};
    /// use futures::future;
    /// assert!(
    /// # tokio_test::block_on(async {
    ///     Card::search("lightning").await
    ///         .unwrap()
    ///         .into_stream_buffered(10)
    ///         .map(Result::unwrap)
    ///         .all(|x| future::ready(x.name.to_lowercase().contains("lightning")))
    ///         .await
    /// # })
    /// )
    /// ```
    ///
    /// ```rust
    /// # use scryfall::search::prelude::*;
    /// # use futures::stream::{self, StreamExt};
    /// # use futures::future;
    /// # fn main() -> scryfall::Result<()> {
    /// use scryfall::Card;
    /// # tokio_test::block_on(async {
    /// let mut demolish = Card::search(set("war").and(collector_number(123))).await?.into_stream().map(Result::unwrap);
    /// assert!(demolish.all(|card| future::ready(&card.name == "Demolish")).await);
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// ```rust
    /// # use scryfall::search::prelude::*;
    /// # use futures::stream::{self, StreamExt};
    /// # use futures::future;
    /// # fn main() -> scryfall::Result<()> {
    /// use scryfall::Card;
    /// # tokio_test::block_on(async {
    /// let mut demolish = Card::search(set("war").and(collector_number(123))).await?.into_stream_buffered(10).map(Result::unwrap);
    /// assert!(demolish.all(|card| future::ready(&card.name == "Demolish")).await);
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    ///
    /// ```
    /// # use scryfall::search::prelude::*;
    /// use scryfall::{Card, Error};
    /// # tokio_test::block_on(async {
    /// let error = Card::search(power(gte(NumProperty::Power))).await.unwrap_err();
    /// if let Error::ScryfallError(e) = error {
    ///     assert!(e.details.contains("All of your terms were ignored"));
    ///     assert!(e.warnings.len() > 0);
    /// }
    /// # else {
    /// #     panic!("Wrong error type: {0} {0:?}", error)
    /// # }
    /// })
    /// ```
    pub async fn search(query: impl Search) -> crate::Result<ListIter<Card>> {
        let mut url = CARDS_URL.join("search/")?;
        query.write_query(&mut url)?;
        Uri::from(url).fetch_iter().await
    }

    /// Returns all cards that match a query, as a `Vec`. If there is more than
    /// one page of cards, this will involve multiple requests to Scryfall
    /// to get all the cards.
    /// ```rust
    /// # use scryfall::search::prelude::*;
    /// # fn main() -> scryfall::Result<()> {
    /// use scryfall::search::prelude::*;
    /// use scryfall::Card;
    /// # tokio_test::block_on(async {
    /// let all_six_sixes = Card::search_all(power(6).and(toughness(6))).await?;
    /// assert!(all_six_sixes.iter().any(|c| &c.name == "Colossal Dreadmaw"));
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn search_all(query: impl Search) -> crate::Result<Vec<Card>> {
        let mut url = CARDS_URL.join("search/")?;
        query.write_query(&mut url)?;
        Uri::from(url).fetch_all().await
    }

    /// Fetches a random card matching a search query.
    ///
    /// # Examples
    /// ```rust
    /// # use scryfall::Card;
    /// # fn main() -> scryfall::Result<()> {
    /// # tokio_test::block_on(async {
    /// let card = Card::search_random("t:Merfolk").await?;
    /// assert!(card.type_line.unwrap().contains("Merfolk"));
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    pub async fn search_random(query: impl Search) -> crate::Result<Card> {
        let mut url = CARDS_URL.join("random/")?;
        query.write_query(&mut url)?;
        Uri::from(url).fetch().await
    }

    /// Return a card with the exact name.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// # tokio_test::block_on(async {
    /// match Card::named("Lightning Bolt").await {
    ///     Ok(card) => assert_eq!(card.name, "Lightning Bolt"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// # })
    /// ```
    ///
    /// ```rust
    /// # use scryfall::card::Card;
    /// use scryfall::error::Error;
    /// # tokio_test::block_on(async {
    /// assert!(Card::named("Name that doesn't exist").await.is_err())
    /// # })
    /// ```
    pub async fn named(name: &str) -> crate::Result<Card> {
        let mut url = CARDS_URL.join("named")?;
        url.query_pairs_mut().append_pair("exact", name);
        Uri::from(url).fetch().await
    }

    /// Return a card using the scryfall fuzzy finder.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// # tokio_test::block_on(async {
    /// match Card::named_fuzzy("Light Bolt").await {
    ///     Ok(card) => assert_eq!(card.name, "Lightning Bolt"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// # })
    /// ```
    pub async fn named_fuzzy(query: &str) -> crate::Result<Card> {
        let mut url = CARDS_URL.join("named")?;
        url.query_pairs_mut().append_pair("fuzzy", query);
        Uri::from(url).fetch().await
    }

    /// Fetch a card by its set and number.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// # tokio_test::block_on(async {
    /// match Card::set_and_number("vma", 4).await {
    ///     Ok(card) => assert_eq!(card.name, "Black Lotus"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// # })
    /// ```
    pub async fn set_and_number(set_code: &str, number: usize) -> crate::Result<Card> {
        Uri::from(CARDS_URL.join(&format!("{}/{}", set_code, number))?)
            .fetch()
            .await
    }

    /// Fetch a card by its multiverse id.
    ///
    /// # Examples
    /// ```rust
    /// # tokio_test::block_on(async {
    /// use scryfall::card::Card;
    /// match Card::multiverse(409574).await {
    ///     Ok(card) => assert_eq!(card.name, "Strip Mine"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// # })
    /// ```
    pub async fn multiverse(multiverse_id: usize) -> crate::Result<Card> {
        Uri::from(
            CARDS_URL
                .join("multiverse/")?
                .join(&multiverse_id.to_string())?,
        )
        .fetch()
        .await
    }

    /// Fetch a card by its mtgo id.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// # tokio_test::block_on(async {
    /// match Card::mtgo(54957).await {
    ///     Ok(card) => assert_eq!(card.name, "Ghost Quarter"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// # })
    /// ```
    pub async fn mtgo(mtgo_id: usize) -> crate::Result<Card> {
        Uri::from(CARDS_URL.join("mtgo/")?.join(&mtgo_id.to_string())?)
            .fetch()
            .await
    }

    /// Fetch a card by its arena id.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// # tokio_test::block_on(async {
    /// match Card::arena(67330).await {
    ///     Ok(card) => assert_eq!(card.name, "Yargle, Glutton of Urborg"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// # })
    /// ```
    pub async fn arena(arena_id: usize) -> crate::Result<Card> {
        Uri::from(CARDS_URL.join("arena/")?.join(&arena_id.to_string())?)
            .fetch()
            .await
    }

    /// Fetch a card by its tcgplayer id.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// # tokio_test::block_on(async {
    /// match Card::tcgplayer(67330).await {
    ///     Ok(card) => assert_eq!(card.name, "Fathom Mage"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// # })
    /// ```
    pub async fn tcgplayer(tcgplayer_id: usize) -> crate::Result<Card> {
        Uri::from(
            CARDS_URL
                .join("tcgplayer/")?
                .join(&tcgplayer_id.to_string())?,
        )
        .fetch()
        .await
    }

    /// Fetch a card by its Uuid.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// # tokio_test::block_on(async {
    /// match Card::scryfall_id("0b81b329-4ef5-4b55-9fe7-9ed69477e96b".parse().unwrap()).await {
    ///     Ok(card) => assert_eq!(card.name, "Cowed by Wisdom"),
    ///     Err(e) => panic!("{:?}", e),
    /// }
    /// # })
    /// ```
    pub async fn scryfall_id(scryfall_id: Uuid) -> crate::Result<Card> {
        Uri::from(CARDS_URL.join(&scryfall_id.to_string())?)
            .fetch()
            .await
    }
}
