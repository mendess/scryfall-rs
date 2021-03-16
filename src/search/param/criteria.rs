//! This module defines the [`Criterion`] type, which contains all the boolean
//! properties Scryfall supports for searching cards.
//!
//! TODO(msmorgan): More.
use std::fmt;

use crate::search::param::Param;
use crate::search::query::Query;

/// A `Criterion` is a boolean flag associated with a card or printing.
///
/// TODO(msmorgan): More.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum Criterion {
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

impl fmt::Display for Criterion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            match self {
                Criterion::HasColorIndicator | Criterion::HasWatermark => "has",
                Criterion::NewCard
                | Criterion::NewArt
                | Criterion::NewArtist
                | Criterion::NewFlavor
                | Criterion::NewFrame
                | Criterion::NewLanguage
                | Criterion::NewRarity => "new",
                Criterion::EvenCmc | Criterion::OddCmc => "cmc",
                _ => "is",
            },
            match self {
                Criterion::HasColorIndicator => "indicator",
                Criterion::HasWatermark => "watermark",
                Criterion::NewCard => "card",
                Criterion::NewRarity => "rarity",
                Criterion::NewArt => "art",
                Criterion::NewFlavor => "flavor",
                Criterion::NewArtist => "artist",
                Criterion::NewFrame => "frame",
                Criterion::NewLanguage => "language",
                Criterion::IsPhyrexian => "phyrexian",
                Criterion::IsHybrid => "hybrid",
                Criterion::IsSplit => "split",
                Criterion::IsFlip => "flip",
                Criterion::IsTransform => "transform",
                Criterion::IsMeld => "meld",
                Criterion::IsLeveler => "leveler",
                Criterion::IsSpell => "spell",
                Criterion::IsPermanent => "permanent",
                Criterion::IsHistoric => "historic",
                Criterion::IsParty => "party",
                Criterion::IsModal => "modal",
                Criterion::IsVanilla => "vanilla",
                Criterion::IsFrenchVanilla => "french_vanilla",
                Criterion::IsFunny => "funny",
                Criterion::IsFull => "full",
                Criterion::IsFoil => "foil",
                Criterion::IsNonFoil => "nonfoil",
                Criterion::IsCommander => "commander",
                Criterion::IsBrawler => "brawler",
                Criterion::IsCompanion => "companion",
                Criterion::IsReserved => "reserved",
                Criterion::IsHiRes => "hires",
                Criterion::IsDigital => "digital",
                Criterion::IsPromo => "promo",
                Criterion::IsSpotlight => "spotlight",
                Criterion::IsFirstPrint => "first_print",
                Criterion::IsReprint => "reprint",
                Criterion::IsMasterpiece => "masterpiece",
                Criterion::IsUnique => "unique",
                Criterion::SoldInBoosters => "booster",
                Criterion::SoldInPwDecks => "planeswalker_deck",
                Criterion::SoldInLeague => "league",
                Criterion::SoldInBuyABox => "buyabox",
                Criterion::SoldInGiftBox => "giftbox",
                Criterion::SoldInIntroPack => "intro_pack",
                Criterion::SoldInGameDay => "gameday",
                Criterion::SoldInPreRelease => "prerelease",
                Criterion::SoldInRelease => "release",
                Criterion::IsBicycleLand => "bicycle_land",
                Criterion::IsTricycleLand => "tricycle_land",
                Criterion::IsBounceLand => "bounce_land",
                Criterion::IsCanopyLand => "canopy_land",
                Criterion::IsCheckLand => "check_land",
                Criterion::IsDualLand => "dual",
                Criterion::IsFastLand => "fast_land",
                Criterion::IsFetchLand => "fetch_land",
                Criterion::IsFilterLand => "filter_land",
                Criterion::IsGainLand => "gain_land",
                Criterion::IsPainLand => "pain_land",
                Criterion::IsScryLand => "scry_land",
                Criterion::IsShadowLand => "shadow_land",
                Criterion::IsShockLand => "shock_land",
                Criterion::IsStorageLand => "storage_land",
                Criterion::IsCreatureLand => "creature_land",
                Criterion::IsTriLand => "tri_land",
                Criterion::IsBattleLand => "battle_land",
                Criterion::EvenCmc => "even",
                Criterion::OddCmc => "odd",
            }
        )
    }
}

/// Match a card that meets the specified `criterion`.
pub fn criterion(criterion: Criterion) -> Query {
    Query::Param(Param::criterion(criterion))
}
