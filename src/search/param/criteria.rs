//! This module defines the [`Criterion`] type, which contains all the boolean
//! properties Scryfall supports for searching cards.
use std::fmt;

use crate::search::param::Param;
use crate::search::query::Query;

/// A search criterion for filtering cards. Each card is tagged with various
/// searchable properties, representing boolean parameters. Some of the criteria
/// are true for each printing of the card (see [`CardIs`]) and others are
/// specific to certain printings (see [`PrintingIs`]).
///
/// The `Criterion` type rarely needs to be used directly, since `CardIs` and
/// `PrintingIs` both implement `Into<`[`Query`]`>`.
///
/// # Examples
///
/// ```rust
/// # use scryfall::search::prelude::*;
/// # fn main() -> scryfall::Result<()> {
/// // Find a random card with Phyrexian mana symbols, available in watermarked foil.
/// let query = Query::And(vec![
///     CardIs::Phyrexian.into(),
///     PrintingIs::Watermark.into(),
///     PrintingIs::Foil.into(),
/// ]);
/// let card: scryfall::Card = query.random()?;
///
/// assert!(
///     card.mana_cost.unwrap().contains("/P")
///         || card.oracle_text.unwrap_or_default().contains("/P")
/// );
/// assert!(card.watermark.is_some());
/// assert!(card.foil);
/// # Ok(())
/// # }
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[allow(missing_docs)]
pub enum Criterion {
    Card(CardIs),
    Printing(PrintingIs),
}

impl fmt::Display for Criterion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Criterion::Card(inner) => fmt::Display::fmt(inner, f),
            Criterion::Printing(inner) => fmt::Display::fmt(inner, f),
        }
    }
}

impl From<Criterion> for Query {
    fn from(criterion: Criterion) -> Self {
        Query::Param(Param::criterion(criterion))
    }
}

/// A search criterion applying to all printings of a card.
///
/// TODO(msmorgan): More.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum CardIs {
    /// You can filter cards that contain Phyrexian mana symbols.
    Phyrexian,
    /// You can filter cards that contain hybrid mana symbols.
    Hybrid,
    /// Find split cards.
    Split,
    /// Find flip cards.
    Flip,
    /// Find transforming cards.
    Transform,
    /// Find cards with meld.
    Meld,
    /// Find leveler cards.
    Leveler,
    /// Find cards that are cast as spells
    Spell,
    /// Find permanent cards.
    Permanent,
    /// Find historic cards.
    Historic,
    /// Find party cards.
    Party,
    /// Find cards with modal effects.
    Modal,
    /// Find vanilla creatures.
    Vanilla,
    /// Find french vanilla creatures (evergreen keywords only).
    FrenchVanilla,
    /// Find Un-cards, holiday cards, and other funny cards.
    Funny,
    /// Find cards that can be your commander.
    Commander,
    /// Find cards that can be your Brawl commander.
    Brawler,
    /// Find cards that can be your companion.
    Companion,
    /// Find cards on the reserved list.
    Reserved,

    /// Cards that have a color indicator.
    ColorIndicator,

    /// A cycling dual land, such as [Fetid Pools](https://scryfall.com/card/akh/243).
    BicycleLand,
    /// A cycling tri land, such as [Ketria Triome](https://scryfall.com/card/iko/250).
    #[doc(alias = "triome")]
    TricycleLand,
    /// A land that returns other lands to your hand, such as
    /// [Boros Garrison](https://scryfall.com/card/rav/275).
    BounceLand,
    /// A pain land that can be sacrificed to draw a card, such as
    /// [Horizon Canopy](https://scryfall.com/card/fut/177).
    CanopyLand,
    /// A land that enters tapped unless you control a basic of its color, such
    /// as [Glacial Fortress](https://scryfall.com/card/m10/226).
    CheckLand,
    /// An original dual land, such as [Tropical Island](https://scryfall.com/card/lea/283).
    DualLand,
    /// A land that enters tapped unless you control two or fewer other lands,
    /// such as [Blackcleave Cliffs](https://scryfall.com/card/som/224).
    FastLand,
    /// A fetch land, such as [Scalding Tarn](https://scryfall.com/card/zen/223).
    FetchLand,
    /// A land that filters mana into other colors, such as
    /// [Mystic Gate](https://scryfall.com/card/shm/277) or
    /// [Cascading Cataracts](https://scryfall.com/card/akh/240/cascading-cataracts).
    FilterLand,
    /// A land that enters tapped and gains 1 life, such as
    /// [Jungle Hollow](https://scryfall.com/card/ktk/235).
    GainLand,
    /// A land that costs life for colored mana, such as
    /// [Caves of Koilos](https://scryfall.com/card/apc/140).
    PainLand,
    /// A land that enters tapped and has "Scry 1", such as
    /// [Temple of Mystery](https://scryfall.com/card/ths/226).
    ScryLand,
    /// A land that enters tapped unless you reveal a basic from your hand, such
    /// as [Choked Estuary](https://scryfall.com/card/soi/270).
    ShadowLand,
    /// A land that enters tapped unless you pay 2 life, such as
    /// [Breeding Pool](https://scryfall.com/card/dis/172).
    ShockLand,
    /// A land that allows you to store up mana for later use, such as
    /// [Fungal Reaches](https://scryfall.com/card/tsp/273) or
    /// [Crucible of the Spirit Dragon](https://scryfall.com/card/frf/167).
    StorageLand,
    /// A land that turns into a creature, such as
    /// [Celestial Colonnade](https://scryfall.com/card/wwk/133),
    /// [Mutavault](https://scryfall.com/card/mor/148), or
    /// [Inkmoth Nexus](https://scryfall.com/card/mbs/145).
    #[doc(alias = "manland")]
    CreatureLand,
    /// A land that enters tapped and produces three colors, such as
    /// [Mystic Monastery](https://scryfall.com/card/ktk/236).
    TriLand,
    /// A land that enters tapped unless you control two basics in its
    /// colors, such as [Canopy Vista](https://scryfall.com/card/bfz/234).
    #[doc(alias = "tango")]
    BattleLand,

    /// The converted mana cost of this card is an even number.
    EvenCmc,
    /// The converted mana cost of this card is an odd number.
    OddCmc,
}

impl fmt::Display for CardIs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            match self {
                CardIs::ColorIndicator => "has", // Synonym for 'is'.
                CardIs::EvenCmc | CardIs::OddCmc => "cmc",
                _ => "is",
            },
            match self {
                CardIs::Phyrexian => "phyrexian",
                CardIs::Hybrid => "hybrid",
                CardIs::Split => "split",
                CardIs::Flip => "flip",
                CardIs::Transform => "transform",
                CardIs::Meld => "meld",
                CardIs::Leveler => "leveler",
                CardIs::Spell => "spell",
                CardIs::Permanent => "permanent",
                CardIs::Historic => "historic",
                CardIs::Party => "party",
                CardIs::Modal => "modal",
                CardIs::Vanilla => "vanilla",
                CardIs::FrenchVanilla => "french_vanilla",
                CardIs::Funny => "funny",
                CardIs::Commander => "commander",
                CardIs::Brawler => "brawler",
                CardIs::Companion => "companion",
                CardIs::Reserved => "reserved",

                CardIs::ColorIndicator => "indicator",

                CardIs::BicycleLand => "bicycle_land",
                CardIs::TricycleLand => "tricycle_land",
                CardIs::BounceLand => "bounce_land",
                CardIs::CanopyLand => "canopy_land",
                CardIs::CheckLand => "check_land",
                CardIs::DualLand => "dual",
                CardIs::FastLand => "fast_land",
                CardIs::FetchLand => "fetch_land",
                CardIs::FilterLand => "filter_land",
                CardIs::GainLand => "gain_land",
                CardIs::PainLand => "pain_land",
                CardIs::ScryLand => "scry_land",
                CardIs::ShadowLand => "shadow_land",
                CardIs::ShockLand => "shock_land",
                CardIs::StorageLand => "storage_land",
                CardIs::CreatureLand => "creature_land",
                CardIs::TriLand => "tri_land",
                CardIs::BattleLand => "battle_land",

                CardIs::EvenCmc => "even",
                CardIs::OddCmc => "odd",
            }
        )
    }
}

impl From<CardIs> for Query {
    fn from(card: CardIs) -> Self {
        Criterion::Card(card).into()
    }
}

/// A search criterion applying to a specific printing of a card.
///
/// TODO(msmorgan): More.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PrintingIs {
    /// Find cards that are printed for the first time in paper.
    NewCard,
    /// Find reprint cards printed at a new rarity for the first time.
    NewRarity,
    /// Find cards being printed with new illustrations.
    NewArt,
    /// Find cards being illustrated by a particular artist for the first time.
    NewArtist,
    /// Find cards being printed with brand-new flavor text using for the first
    /// time.
    NewFlavor,
    /// Find cards printed in a specific frame for the first time.
    NewFrame,
    /// Find the first printing of a card in each language.
    NewLanguage,

    /// Printings that have a watermark.
    Watermark,

    /// Find cards with full art.
    Full,
    /// Find non-foil printings of cards.
    NonFoil,
    /// Find foil printings of cards.
    Foil,
    /// Find cards in `scryfall`'s database with high-resolution images.
    HiRes,
    /// Find prints that are only available digitally (MTGO and Arena)
    Digital,
    /// Find promotional cards.
    Promo,
    /// Find cards that are Story Spotlights.
    Spotlight,
    /// Find cards that are in the Masterpiece Series.
    Masterpiece,
    /// Find cards that have only been in a single set.
    Unique,
    /// Find first printings (digital or paper).
    FirstPrint,
    /// Find reprints.
    Reprint,
}

impl fmt::Display for PrintingIs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            match self {
                PrintingIs::NewCard
                | PrintingIs::NewRarity
                | PrintingIs::NewArt
                | PrintingIs::NewArtist
                | PrintingIs::NewFlavor
                | PrintingIs::NewFrame
                | PrintingIs::NewLanguage => "new",
                PrintingIs::Watermark => "has", // Synonym for `is`.
                _ => "is",
            },
            match self {
                PrintingIs::NewCard => "card",
                PrintingIs::NewRarity => "rarity",
                PrintingIs::NewArt => "art",
                PrintingIs::NewArtist => "artist",
                PrintingIs::NewFlavor => "flavor",
                PrintingIs::NewFrame => "frame",
                PrintingIs::NewLanguage => "language",

                PrintingIs::Watermark => "watermark",

                PrintingIs::Full => "full",
                PrintingIs::Foil => "foil",
                PrintingIs::NonFoil => "nonfoil",
                PrintingIs::HiRes => "hires",
                PrintingIs::Digital => "digital",
                PrintingIs::Promo => "promo",
                PrintingIs::Spotlight => "spotlight",
                PrintingIs::FirstPrint => "first_print",
                PrintingIs::Reprint => "reprint",
                PrintingIs::Masterpiece => "masterpiece",
                PrintingIs::Unique => "unique",
            }
        )
    }
}

impl From<PrintingIs> for Query {
    fn from(printing: PrintingIs) -> Self {
        Criterion::Printing(printing).into()
    }
}
