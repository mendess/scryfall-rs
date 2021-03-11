//! This module defines the [`Property`] type, which contains all the boolean
//! properties Scryfall supports for searching cards.
use std::fmt;

use crate::search::param::Param;
use crate::search::query::Query;

/// A `Property` is a boolean flag associated with a card or printing.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum Property {
    /// Cards that have a color indicator.
    HasColorIndicator,
    /// Cards that have a watermark.
    HasWatermark,
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
    /// You can filter cards that contain Phyrexian mana symbols.
    IsPhyrexian,
    /// You can filter cards that contain hybrid mana symbols.
    IsHybrid,
    /// Find split cards.
    IsSplit,
    /// Find flip cards.
    IsFlip,
    /// Find transforming cards.
    IsTransform,
    /// Find cards with meld.
    IsMeld,
    /// Find leveler cards.
    IsLeveler,
    /// Find cards that are cast as spells
    IsSpell,
    /// Find permanent cards.
    IsPermanent,
    /// Find historic cards.
    IsHistoric,
    /// Find party cards.
    IsParty,
    /// Find cards with modal effects.
    IsModal,
    /// Find vanilla creatures.
    IsVanilla,
    /// Find french vanilla creatures (evergreen keywords only).
    IsFrenchVanilla,
    /// Find Un-cards, holiday cards, and other funny cards.
    IsFunny,
    /// Find cards that can be your commander.
    IsCommander,
    /// Find cards that can be your Brawl commander.
    IsBrawler,
    /// Find cards that can be your companion.
    IsCompanion,
    /// Find cards on the reserved list.
    IsReserved,
    /// Find cards with full art.
    IsFull,
    /// Find non-foil printings of cards.
    IsNonFoil,
    /// Find foil printings of cards.
    IsFoil,
    /// Find cards in `scryfall`'s database with high-resolution images.
    IsHiRes,
    /// Find prints that are only available digitally (MTGO and Arena)
    IsDigital,
    /// Find promotional cards.
    IsPromo,
    /// Find cards that are Story Spotlights.
    IsSpotlight,
    /// Find cards that are in the Masterpiece Series.
    IsMasterpiece,
    /// Find cards that have only been in a single set.
    IsUnique,
    /// Find first printings (digital or paper).
    IsFirstPrint,
    /// Find reprints.
    IsReprint,
    /// Find cards that were sold in boosters.
    SoldInBoosters,
    /// Find cards that were sold in planeswalker decks.
    SoldInPwDecks,
    /// Find cards that were given away in leagues.
    SoldInLeague,
    /// Find cards that were given away as buy a box promos.
    SoldInBuyABox,
    /// Find cards that were given away in gift boxes.
    SoldInGiftBox,
    /// Find cards that were given away in intro packs.
    SoldInIntroPack,
    /// Find cards that were given away in game days.
    SoldInGameDay,
    /// Find cards that were given away in pre-releases.
    SoldInPreRelease,
    /// Find cards that were given away in releases.
    SoldInRelease,

    /// A cycling dual land, such as [Fetid Pools](https://scryfall.com/card/akh/243).
    IsBicycleLand,
    /// A cycling tri land, such as [Ketria Triome](https://scryfall.com/card/iko/250).
    IsTricycleLand,
    /// A land that returns other lands to your hand, such as
    /// [Boros Garrison](https://scryfall.com/card/rav/275).
    IsBounceLand,
    /// A pain land that can be sacrificed to draw a card, such as
    /// [Horizon Canopy](https://scryfall.com/card/fut/177).
    IsCanopyLand,
    /// A land that enters tapped unless you control a basic of its color, such
    /// as [Glacial Fortress](https://scryfall.com/card/m10/226).
    IsCheckLand,
    /// An original dual land, such as [Tropical Island](https://scryfall.com/card/lea/283).
    IsDualLand,
    /// A land that enters tapped unless you control two or fewer other lands,
    /// such as [Blackcleave Cliffs](https://scryfall.com/card/som/224).
    IsFastLand,
    /// A fetch land, such as [Scalding Tarn](https://scryfall.com/card/zen/223).
    IsFetchLand,
    /// A land that filters mana into other colors, such as
    /// [Mystic Gate](https://scryfall.com/card/shm/277) or
    /// [Cascading Cataracts](https://scryfall.com/card/akh/240/cascading-cataracts).
    IsFilterLand,
    /// A land that enters tapped and gains 1 life, such as
    /// [Jungle Hollow](https://scryfall.com/card/ktk/235).
    IsGainLand,
    /// A land that costs life for colored mana, such as
    /// [Caves of Koilos](https://scryfall.com/card/apc/140).
    IsPainLand,
    /// A land that enters tapped and has "Scry 1", such as
    /// [Temple of Mystery](https://scryfall.com/card/ths/226).
    IsScryLand,
    /// A land that enters tapped unless you reveal a basic from your hand, such
    /// as [Choked Estuary](https://scryfall.com/card/soi/270).
    IsShadowLand,
    /// A land that enters tapped unless you pay 2 life, such as
    /// [Breeding Pool](https://scryfall.com/card/dis/172).
    IsShockLand,
    /// A land that allows you to store up mana for later use, such as
    /// [Fungal Reaches](https://scryfall.com/card/tsp/273) or
    /// [Crucible of the Spirit Dragon](https://scryfall.com/card/frf/167).
    IsStorageLand,
    /// A land that turns into a creature, such as
    /// [Celestial Colonnade](https://scryfall.com/card/wwk/133),
    /// [Mutavault](https://scryfall.com/card/mor/148), or
    /// [Inkmoth Nexus](https://scryfall.com/card/mbs/145).
    IsCreatureLand,
    /// A land that enters tapped and produces three colors, such as
    /// [Mystic Monastery](https://scryfall.com/card/ktk/236).
    IsTriLand,
    /// A land that enters tapped unless you control two basics in its
    /// colors, such as [Canopy Vista](https://scryfall.com/card/bfz/234).
    IsBattleLand,

    /// The converted mana cost of this card is an even number.
    EvenCmc,
    /// The converted mana cost of this card is an odd number.
    OddCmc,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            match self {
                Property::HasColorIndicator | Property::HasWatermark => "has",
                Property::NewCard
                | Property::NewArt
                | Property::NewArtist
                | Property::NewFlavor
                | Property::NewFrame
                | Property::NewLanguage
                | Property::NewRarity => "new",
                Property::EvenCmc | Property::OddCmc => "cmc",
                _ => "is",
            },
            match self {
                Property::HasColorIndicator => "indicator",
                Property::HasWatermark => "watermark",
                Property::NewCard => "card",
                Property::NewRarity => "rarity",
                Property::NewArt => "art",
                Property::NewFlavor => "flavor",
                Property::NewArtist => "artist",
                Property::NewFrame => "frame",
                Property::NewLanguage => "language",
                Property::IsPhyrexian => "phyrexian",
                Property::IsHybrid => "hybrid",
                Property::IsSplit => "split",
                Property::IsFlip => "flip",
                Property::IsTransform => "transform",
                Property::IsMeld => "meld",
                Property::IsLeveler => "leveler",
                Property::IsSpell => "spell",
                Property::IsPermanent => "permanent",
                Property::IsHistoric => "historic",
                Property::IsParty => "party",
                Property::IsModal => "modal",
                Property::IsVanilla => "vanilla",
                Property::IsFrenchVanilla => "french_vanilla",
                Property::IsFunny => "funny",
                Property::IsFull => "full",
                Property::IsFoil => "foil",
                Property::IsNonFoil => "nonfoil",
                Property::IsCommander => "commander",
                Property::IsBrawler => "brawler",
                Property::IsCompanion => "companion",
                Property::IsReserved => "reserved",
                Property::IsHiRes => "hires",
                Property::IsDigital => "digital",
                Property::IsPromo => "promo",
                Property::IsSpotlight => "spotlight",
                Property::IsFirstPrint => "first_print",
                Property::IsReprint => "reprint",
                Property::IsMasterpiece => "masterpiece",
                Property::IsUnique => "unique",
                Property::SoldInBoosters => "booster",
                Property::SoldInPwDecks => "planeswalker_deck",
                Property::SoldInLeague => "league",
                Property::SoldInBuyABox => "buyabox",
                Property::SoldInGiftBox => "giftbox",
                Property::SoldInIntroPack => "intro_pack",
                Property::SoldInGameDay => "gameday",
                Property::SoldInPreRelease => "prerelease",
                Property::SoldInRelease => "release",
                Property::IsBicycleLand => "bicycle_land",
                Property::IsTricycleLand => "tricycle_land",
                Property::IsBounceLand => "bounce_land",
                Property::IsCanopyLand => "canopy_land",
                Property::IsCheckLand => "check_land",
                Property::IsDualLand => "dual",
                Property::IsFastLand => "fast_land",
                Property::IsFetchLand => "fetch_land",
                Property::IsFilterLand => "filter_land",
                Property::IsGainLand => "gain_land",
                Property::IsPainLand => "pain_land",
                Property::IsScryLand => "scry_land",
                Property::IsShadowLand => "shadow_land",
                Property::IsShockLand => "shock_land",
                Property::IsStorageLand => "storage_land",
                Property::IsCreatureLand => "creature_land",
                Property::IsTriLand => "tri_land",
                Property::IsBattleLand => "battle_land",
                Property::EvenCmc => "even",
                Property::OddCmc => "odd",
            }
        )
    }
}

/// Match a card with a specified [`Property`].
pub fn prop(prop: Property) -> Query {
    Query::Param(Param::property(prop))
}
