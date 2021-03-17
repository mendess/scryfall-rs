//! This module defines the [`Criterion`] type, which contains all the boolean
//! properties Scryfall supports for searching cards.
use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) enum Has {
    /// Cards that have a color indicator.
    ColorIndicator,
    /// Cards that have a watermark.
    Watermark,
}

impl fmt::Display for Has {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Has::ColorIndicator => "indicator",
                Has::Watermark => "watermark",
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) enum New {
    /// Find cards that are printed for the first time in paper.
    Card,
    /// Find reprint cards printed at a new rarity for the first time.
    Rarity,
    /// Find cards being printed with new illustrations.
    Art,
    /// Find cards being illustrated by a particular artist for the first time.
    Artist,
    /// Find cards being printed with brand-new flavor text using for the first
    /// time.
    Flavor,
    /// Find cards printed in a specific frame for the first time.
    Frame,
    /// Find the first printing of a card in each language.
    Language,
}

impl fmt::Display for New {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                New::Card => "card",
                New::Rarity => "rarity",
                New::Art => "art",
                New::Flavor => "flavor",
                New::Artist => "artist",
                New::Frame => "frame",
                New::Language => "language",
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) enum Is {
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

impl fmt::Display for Is {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Is::Phyrexian => "phyrexian",
                Is::Hybrid => "hybrid",
                Is::Split => "split",
                Is::Flip => "flip",
                Is::Transform => "transform",
                Is::Meld => "meld",
                Is::Leveler => "leveler",
                Is::Spell => "spell",
                Is::Permanent => "permanent",
                Is::Historic => "historic",
                Is::Party => "party",
                Is::Modal => "modal",
                Is::Vanilla => "vanilla",
                Is::FrenchVanilla => "french_vanilla",
                Is::Funny => "funny",
                Is::Full => "full",
                Is::Foil => "foil",
                Is::NonFoil => "nonfoil",
                Is::Commander => "commander",
                Is::Brawler => "brawler",
                Is::Companion => "companion",
                Is::Reserved => "reserved",
                Is::HiRes => "hires",
                Is::Digital => "digital",
                Is::Promo => "promo",
                Is::Spotlight => "spotlight",
                Is::FirstPrint => "first_print",
                Is::Reprint => "reprint",
                Is::Masterpiece => "masterpiece",
                Is::Unique => "unique",
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) enum SoldIn {
    /// Find cards that were sold in boosters.
    Booster,
    /// Find cards that were sold in planeswalker decks.
    PlaneswalkerDeck,
    /// Find cards that were given away in leagues.
    League,
    /// Find cards that were given away as buy a box promos.
    BuyABox,
    /// Find cards that were given away in gift boxes.
    GiftBox,
    /// Find cards that were given away in intro packs.
    IntroPack,
    /// Find cards that were given away in game days.
    GameDay,
    /// Find cards that were given away in pre-releases.
    Prerelease,
    /// Find cards that were given away in releases.
    Release,
}

impl fmt::Display for SoldIn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SoldIn::Booster => "booster",
                SoldIn::PlaneswalkerDeck => "planeswalker_deck",
                SoldIn::League => "league",
                SoldIn::BuyABox => "buyabox",
                SoldIn::GiftBox => "giftbox",
                SoldIn::IntroPack => "intro_pack",
                SoldIn::GameDay => "gameday",
                SoldIn::Prerelease => "prerelease",
                SoldIn::Release => "release",
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) enum LandFamily {
    /// A cycling dual land, such as [Fetid Pools](https://scryfall.com/card/akh/243).
    Bicycle,
    /// A cycling tri land, such as [Ketria Triome](https://scryfall.com/card/iko/250).
    Tricycle,
    /// A land that returns other lands to your hand, such as
    /// [Boros Garrison](https://scryfall.com/card/rav/275).
    Bounce,
    /// A pain land that can be sacrificed to draw a card, such as
    /// [Horizon Canopy](https://scryfall.com/card/fut/177).
    Canopy,
    /// A land that enters tapped unless you control a basic of its color, such
    /// as [Glacial Fortress](https://scryfall.com/card/m10/226).
    Check,
    /// An original dual land, such as [Tropical Island](https://scryfall.com/card/lea/283).
    Dual,
    /// A land that enters tapped unless you control two or fewer other lands,
    /// such as [Blackcleave Cliffs](https://scryfall.com/card/som/224).
    Fast,
    /// A fetch land, such as [Scalding Tarn](https://scryfall.com/card/zen/223).
    Fetch,
    /// A land that filters mana into other colors, such as
    /// [Mystic Gate](https://scryfall.com/card/shm/277) or
    /// [Cascading Cataracts](https://scryfall.com/card/akh/240/cascading-cataracts).
    Filter,
    /// A land that enters tapped and gains 1 life, such as
    /// [Jungle Hollow](https://scryfall.com/card/ktk/235).
    Gain,
    /// A land that costs life for colored mana, such as
    /// [Caves of Koilos](https://scryfall.com/card/apc/140).
    Pain,
    /// A land that enters tapped and has "Scry 1", such as
    /// [Temple of Mystery](https://scryfall.com/card/ths/226).
    Scry,
    /// A land that enters tapped unless you reveal a basic from your hand, such
    /// as [Choked Estuary](https://scryfall.com/card/soi/270).
    Shadow,
    /// A land that enters tapped unless you pay 2 life, such as
    /// [Breeding Pool](https://scryfall.com/card/dis/172).
    Shock,
    /// A land that allows you to store up mana for later use, such as
    /// [Fungal Reaches](https://scryfall.com/card/tsp/273) or
    /// [Crucible of the Spirit Dragon](https://scryfall.com/card/frf/167).
    Storage,
    /// A land that turns into a creature, such as
    /// [Celestial Colonnade](https://scryfall.com/card/wwk/133),
    /// [Mutavault](https://scryfall.com/card/mor/148), or
    /// [Inkmoth Nexus](https://scryfall.com/card/mbs/145).
    Creature,
    /// A land that enters tapped and produces three colors, such as
    /// [Mystic Monastery](https://scryfall.com/card/ktk/236).
    Tri,
    /// A land that enters tapped unless you control two basics in its
    /// colors, such as [Canopy Vista](https://scryfall.com/card/bfz/234).
    Battle,
}

impl fmt::Display for LandFamily {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LandFamily::Bicycle => "bicycle_land",
                LandFamily::Tricycle => "tricycle_land",
                LandFamily::Bounce => "bounce_land",
                LandFamily::Canopy => "canopy_land",
                LandFamily::Check => "check_land",
                LandFamily::Dual => "dual",
                LandFamily::Fast => "fast_land",
                LandFamily::Fetch => "fetch_land",
                LandFamily::Filter => "filter_land",
                LandFamily::Gain => "gain_land",
                LandFamily::Pain => "pain_land",
                LandFamily::Scry => "scry_land",
                LandFamily::Shadow => "shadow_land",
                LandFamily::Shock => "shock_land",
                LandFamily::Storage => "storage_land",
                LandFamily::Creature => "creature_land",
                LandFamily::Tri => "tri_land",
                LandFamily::Battle => "battle_land",
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) enum Cmc {
    /// The converted mana cost of this card is an even number.
    Even,
    /// The converted mana cost of this card is an odd number.
    Odd,
}

impl fmt::Display for Cmc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cmc::Even => "even",
                Cmc::Odd => "odd",
            }
        )
    }
}

/// A `Criterion` is a boolean flag associated with a card or printing.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(super) enum Criterion {
    Has(Has),
    New(New),
    Is(Is),
    SoldIn(SoldIn),
    LandFamily(LandFamily),
    Cmc(Cmc),
}

impl fmt::Display for Criterion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Criterion::Has(has) => write!(f, "has:{}", has),
            Criterion::New(new) => write!(f, "new:{}", new),
            Criterion::Is(is) => write!(f, "is:{}", is),
            Criterion::LandFamily(land_family) => write!(f, "is:{}", land_family),
            Criterion::SoldIn(sold_in) => write!(f, "is:{}", sold_in),
            Criterion::Cmc(cmc) => write!(f, "cmc:{}", cmc),
        }
    }
}
