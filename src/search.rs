#![allow(missing_docs)]

use std::collections::VecDeque;
use std::fmt::Write;
use std::hash::Hash;
use std::rc::Rc;
use std::{fmt, ops};

use serde::{Serialize, Serializer};
use url::Url;

pub use self::compare_fns::*;
pub use self::param_fns::*;
pub use crate::card_searcher::{SortDirection, SortMethod, UniqueStrategy};
use crate::list::ListIter;
use crate::Card;

pub trait Search {
    fn write_query(&self, url: &mut Url) -> crate::Result<()>;

    fn search(&self) -> crate::Result<ListIter<Card>>
    where
        Self: Sized,
    {
        Card::search_new(self)
    }

    fn random(&self) -> crate::Result<Card>
    where
        Self: Sized,
    {
        Card::search_random_new(self)
    }
}

impl Search for SearchOptions {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        self.serialize(serde_urlencoded::Serializer::new(
            &mut url.query_pairs_mut(),
        ))?;
        Ok(())
    }
}

impl Search for str {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        url.set_query(Some(self));
        Ok(())
    }
}

impl Search for Query {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        url.query_pairs_mut()
            .append_pair("q", self.to_string().as_str());
        Ok(())
    }
}

#[derive(Serialize, Default, Debug)]
pub struct SearchOptions {
    #[serde(skip_serializing_if = "is_default")]
    unique: UniqueStrategy,
    #[serde(skip_serializing_if = "is_default")]
    sort_by: SortMethod,
    #[serde(skip_serializing_if = "is_default")]
    dir: SortDirection,
    #[serde(skip_serializing_if = "is_default")]
    page: usize,
    #[serde(skip_serializing_if = "is_default")]
    include_extras: bool,
    #[serde(skip_serializing_if = "is_default")]
    include_multilingual: bool,
    #[serde(skip_serializing_if = "is_default")]
    include_variations: bool,
    #[serde(rename = "q", serialize_with = "serialize_query")]
    query: Query,
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    value == &Default::default()
}

fn serialize_query<S: Serializer>(query: &Query, serializer: S) -> Result<S::Ok, S::Error> {
    query.to_string().serialize(serializer)
}

impl SearchOptions {
    pub fn new() -> Self {
        SearchOptions {
            page: 1,
            ..Default::default()
        }
    }

    pub fn query(&mut self, query: Query) -> &mut Self {
        self.query = query;
        self
    }

    pub fn unique(&mut self, unique: UniqueStrategy) -> &mut Self {
        self.unique = unique;
        self
    }

    pub fn sorted(&mut self, sort_by: SortMethod, dir: SortDirection) -> &mut Self {
        self.sort_by = sort_by;
        self.dir = dir;
        self
    }

    pub fn extras(&mut self, include_extras: bool) -> &mut Self {
        self.include_extras = include_extras;
        self
    }

    pub fn multilingual(&mut self, include_multilingual: bool) -> &mut Self {
        self.include_multilingual = include_multilingual;
        self
    }

    pub fn variations(&mut self, include_variations: bool) -> &mut Self {
        self.include_variations = include_variations;
        self
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Query {
    And(VecDeque<Query>),
    Or(VecDeque<Query>),
    Not(Box<Query>),
    Param(Param),
    Empty,
}

impl Default for Query {
    fn default() -> Self {
        Query::Empty
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (exprs, sep) = match self {
            Query::And(exprs) => (exprs, " AND "),
            Query::Or(exprs) => (exprs, " OR "),
            Query::Not(expr) => return write!(f, "-{}", expr),
            Query::Param(param) => return write!(f, "{}", param),
            Query::Empty => return write!(f, ""),
        };

        use itertools::Itertools;
        write!(f, "({})", exprs.iter().format(sep))
    }
}

impl From<Param> for Query {
    fn from(param: Param) -> Self {
        Query::Param(param)
    }
}

impl ops::BitAnd for Query {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        self.and(other)
    }
}

impl ops::BitOr for Query {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        self.or(other)
    }
}

impl ops::Not for Query {
    type Output = Query;

    fn not(self) -> Self::Output {
        match self {
            Query::Not(q) => *q,
            q => Query::Not(Box::new(q)),
        }
    }
}

impl Query {
    pub fn and(self, other: Self) -> Query {
        match (self, other) {
            (Query::Empty, x) | (x, Query::Empty) => x,
            (Query::And(mut a_list), Query::And(mut b_list)) => {
                a_list.append(&mut b_list);
                Query::And(a_list)
            },
            (Query::And(mut a_list), b) => {
                a_list.push_back(b);
                Query::And(a_list)
            },
            (a, Query::And(mut b_list)) => {
                b_list.push_front(a);
                Query::And(b_list)
            },
            (a, b) => {
                let mut list = VecDeque::with_capacity(2);
                list.push_back(a);
                list.push_back(b);
                Query::And(list)
            },
        }
    }

    pub fn or(self, other: Self) -> Query {
        match (self, other) {
            (Query::Empty, x) | (x, Query::Empty) => x,
            (Query::Or(mut a_list), Query::Or(mut b_list)) => {
                a_list.append(&mut b_list);
                Query::Or(a_list)
            },
            (Query::Or(mut a_list), b) => {
                a_list.push_back(b);
                Query::Or(a_list)
            },
            (a, Query::Or(mut b_list)) => {
                b_list.push_front(a);
                Query::Or(b_list)
            },
            (a, b) => {
                let mut list = VecDeque::with_capacity(2);
                list.push_back(a);
                list.push_back(b);
                Query::Or(list)
            },
        }
    }
}

mod query_fns {
    use super::*;

    pub fn and(queries: impl IntoIterator<Item = Query>) -> Query {
        let mut result = Query::Empty;
        for query in queries {
            result = result.and(query);
        }
        result
    }

    pub fn or(queries: impl IntoIterator<Item = Query>) -> Query {
        let mut result = Query::Empty;
        for query in queries {
            result = result.and(query);
        }
        result
    }

    pub fn not(q: Query) -> Query {
        ops::Not::not(q)
    }
}

#[derive(Clone, Debug)]
pub struct Param(ParamImpl);

#[derive(Clone, Debug)]
enum ParamImpl {
    Property(Property),
    Value(ValueKind, Rc<Compare<Box<dyn ParamValue>>>),
}

impl PartialEq for Param {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            ParamImpl::Property(kind) => fmt::Display::fmt(kind, f),
            // TODO(msmorgan): Quote the value. How?
            ParamImpl::Value(ValueKind::Exact, value) => write!(f, "!{}", value.value),
            ParamImpl::Value(kind, value) => {
                write!(f, "{}{}", kind, value)
            },
        }
    }
}

impl From<Property> for Param {
    fn from(prop: Property) -> Self {
        Param(ParamImpl::Property(prop))
    }
}

mod param_fns {
    use super::*;

    macro_rules! param_fns {
        ($($meth:ident => $Kind:ident : $Constraint:ident,)*) => {
            $(
                pub fn $meth(value: impl $Constraint) -> Query {
                    Query::Param(Param(ParamImpl::Value(
                        ValueKind::$Kind,
                        Rc::new(value.into_compare()),
                    )))
                }
            )*
        };
    }

    pub fn prop(prop: Property) -> Query {
        Query::Param(Param(ParamImpl::Property(prop)))
    }

    param_fns! {
        color => Color: ColorValue,
        artist => Artist: TextValue,
        cmc => Cmc: NumericValue,
        named => Name: TextOrRegexValue,
        keyword => Keyword: TextValue,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum Property {
    /// Cards that have a color indicator.
    HasColorIndicator,
    /// Cards that have a watermark.
    HasWatermark,
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

    IsCycleLand,
    IsBounceLand,
    IsCanopyLand,
    IsCheckLand,
    IsDualLand,
    IsFastLand,
    IsFetchLand,
    IsFilterLand,
    IsGainLand,
    IsPainLand,
    IsScryLand,
    IsShadowLand,
    IsShockLand,
    IsStorageLand,
    IsCreatureLand,
    IsTriLand,
    IsBattleLand,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            match self {
                Property::HasColorIndicator | Property::HasWatermark => "has",
                Property::NewArt
                | Property::NewArtist
                | Property::NewFlavor
                | Property::NewFrame
                | Property::NewLanguage
                | Property::NewRarity => "new",
                _ => "is",
            },
            match self {
                Property::HasColorIndicator => "indicator",
                Property::HasWatermark => "watermark",
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
                Property::IsFrenchVanilla => "frenchvanilla",
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
                Property::IsCycleLand => "cycle_land",
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
            }
        )
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum ValueKind {
    Color,
    ColorIdentity,
    Type,
    Oracle,
    FullOracle,
    Keyword,
    Mana,
    Cmc,
    Devotion,
    Produces,
    Power,
    Toughness,
    PowTou,
    Loyalty,
    Rarity,
    InRarity,
    Set,
    InSet,
    Number,
    Block,
    SetType,
    InSetType,
    Cube,
    Format,
    Banned,
    Restricted,
    Usd,
    UsdFoil,
    Eur,
    Tix,
    Cheapest,
    Artist,
    ArtistCount,
    Flavor,
    Watermark,
    IllustrationCount,
    BorderColor,
    Frame,
    Year,
    Date,
    PrintCount,
    SetCount,
    PaperPrintCount,
    PaperSetCount,
    Game,
    InGame,
    Language,
    InLanguage,
    Name,
    Exact,
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ValueKind::Color => "color",
                ValueKind::ColorIdentity => "identity",
                ValueKind::Type => "type",
                ValueKind::Oracle => "oracle",
                ValueKind::FullOracle => "fulloracle",
                ValueKind::Keyword => "keyword",
                ValueKind::Mana => "mana",
                ValueKind::Cmc => "cmc",
                ValueKind::Devotion => "devotion",
                ValueKind::Produces => "produces",
                ValueKind::Power => "power",
                ValueKind::Toughness => "toughness",
                ValueKind::PowTou => "powtou",
                ValueKind::Loyalty => "loyalty",
                ValueKind::Rarity => "rarity",
                ValueKind::Set => "set",
                ValueKind::Number => "number",
                ValueKind::Block => "block",
                ValueKind::SetType => "settype",
                ValueKind::Cube => "cube",
                ValueKind::Format => "format",
                ValueKind::Banned => "banned",
                ValueKind::Restricted => "restricted",
                ValueKind::Usd => "usd",
                ValueKind::UsdFoil => "usdfoil",
                ValueKind::Eur => "eur",
                ValueKind::Tix => "tix",
                ValueKind::Cheapest => "cheapest",
                ValueKind::Artist => "artist",
                ValueKind::ArtistCount => "artists",
                ValueKind::Flavor => "flavor",
                ValueKind::Watermark => "watermark",
                ValueKind::IllustrationCount => "illustrations",
                ValueKind::BorderColor => "border",
                ValueKind::Frame => "frame",
                ValueKind::Year => "year",
                ValueKind::Date => "date",
                ValueKind::PrintCount => "prints",
                ValueKind::SetCount => "sets",
                ValueKind::PaperPrintCount => "paperprints",
                ValueKind::PaperSetCount => "papersets",
                ValueKind::Game => "game",
                ValueKind::Language => "language",
                ValueKind::InRarity
                | ValueKind::InSet
                | ValueKind::InSetType
                | ValueKind::InGame
                | ValueKind::InLanguage => "in",
                ValueKind::Name => "name",
                // TODO(msmorgan): Should "Exact" be a ValueKind?
                ValueKind::Exact => "",
            }
        )
    }
}

pub enum Guild {
    Azorius,
    Boros,
    Dimir,
    Golgari,
    Gruul,
    Izzet,
    Orzhov,
    Rakdos,
    Selesnya,
    Simic,
}

pub enum Shard {
    Bant,
    Esper,
    Grixis,
    Jund,
    Naya,
}

pub enum Wedge {
    Abzan,
    Jeskai,
    Mardu,
    Sultai,
    Temur,
}

pub enum FourColor {
    Aggression,
    Altruism,
    Artifice,
    Chaos,
    Growth,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum CompareOp {
    Lte,
    Lt,
    Gte,
    Gt,
    Eq,
    Neq,
}

impl fmt::Display for CompareOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            CompareOp::Lte => "<=",
            CompareOp::Lt => "<",
            CompareOp::Gte => ">=",
            CompareOp::Gt => ">",
            CompareOp::Eq => "=",
            CompareOp::Neq => "!=",
        })
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Compare<T> {
    op: Option<CompareOp>,
    value: T,
}

impl<T: fmt::Display> fmt::Display for Compare<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(op) = &self.op {
            write!(f, "{}{}", op, &self.value)
        } else {
            write!(f, ":{}", &self.value)
        }
    }
}

mod compare_fns {
    use super::*;

    macro_rules! compare_fns {
        ($($meth:ident => $Variant:ident,)*) => {
            $(
                pub fn $meth<T>(value: T) -> Compare<T> {
                    Compare {
                        op: Some(CompareOp::$Variant),
                        value,
                    }
                }
            )*
        };
    }

    compare_fns! {
        lt => Lt,
        lte => Lte,
        gte => Gte,
        gt => Gt,
        eq => Eq,
        neq => Neq,
    }
}

pub trait ParamValue: fmt::Debug + fmt::Display {
    fn into_compare(self) -> Compare<Box<dyn ParamValue>>;
}

impl<T: 'static + ParamValue> ParamValue for Compare<T> {
    fn into_compare(self) -> Compare<Box<dyn ParamValue>> {
        Compare {
            op: self.op,
            value: Box::new(self.value),
        }
    }
}

pub trait ColorValue: ParamValue {}

impl<T: 'static + ColorValue> ColorValue for Compare<T> {}

pub trait NumericValue: ParamValue {}

impl<T: 'static + NumericValue> NumericValue for Compare<T> {}

pub trait TextValue: ParamValue {}

impl<T: 'static + TextValue> TextValue for Compare<T> {}

pub trait TextOrRegexValue: ParamValue {}

impl<T: 'static + TextOrRegexValue> TextOrRegexValue for Compare<T> {}

// TODO(msmorgan): This is inelegant.
macro_rules! impl_into_compare {
    () => {
        fn into_compare(self) -> Compare<Box<dyn ParamValue>> {
            Compare {
                op: None,
                value: Box::new(self),
            }
        }
    };
}

impl ParamValue for String {
    impl_into_compare!();
}

impl TextValue for String {}

impl TextOrRegexValue for String {}

impl ParamValue for &'_ str {
    fn into_compare(self) -> Compare<Box<dyn ParamValue>> {
        Compare {
            op: None,
            value: Box::new(self.to_string()),
        }
    }
}

impl TextValue for &'_ str {}

impl TextOrRegexValue for &'_ str {}

impl ParamValue for u32 {
    impl_into_compare!();
}

impl NumericValue for u32 {}

pub mod prelude {
    pub use super::compare_fns::*;
    pub use super::param_fns::*;
    pub use super::query_fns::*;
    pub use super::{
        Compare,
        ParamValue,
        Search,
        SearchOptions,
        TextOrRegexValue,
        TextValue,
        UniqueStrategy,
    };
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::prelude::*;
    use crate::search::{Property, SortDirection, SortMethod};

    #[test]
    fn basic_search() {
        let cards = SearchOptions::new()
            .query(named("lightning").and(named("helix")).and(cmc(eq(2))))
            .unique(UniqueStrategy::Prints)
            .search()
            .unwrap()
            .map(|c| c.unwrap())
            .collect::<Vec<_>>();

        assert!(cards.len() > 1);

        for card in cards {
            assert_eq!(card.name, "Lightning Helix")
        }
    }

    #[test]
    fn random_works_with_search_options() {
        // `SearchOptions` can set more query params than the "cards/random" API method
        // accepts. Scryfall should ignore these and return a random card.
        assert!(
            SearchOptions::new()
                .query(keyword("storm"))
                .unique(UniqueStrategy::Art)
                .sorted(SortMethod::Usd, SortDirection::Ascending)
                .extras(true)
                .multilingual(true)
                .variations(true)
                .random()
                .unwrap()
                .oracle_text
                .unwrap()
                .to_lowercase()
                .contains("storm")
        );
    }

    #[test]
    #[ignore]
    fn all_properties_work() {
        for p in Property::iter() {
            let query = prop(p);
            query
                .random()
                .unwrap_or_else(|_| panic!("Could not get a random card with {}", p));
        }
    }
}
