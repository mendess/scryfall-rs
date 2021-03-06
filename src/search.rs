#![warn(missing_docs)]

use std::fmt;
use std::hash::Hash;
use std::rc::Rc as Lrc;

use serde::{Serialize, Serializer};
use url::Url;

pub use self::compare_fns::*;
pub use self::param_fns::*;
use crate::list::ListIter;
use crate::Card;

/// A type implementing `Search` can be turned into a Scryfall query. This is
/// the argument type for [`Card::search`] and
/// [`search_random`][Card::search_random].
///
/// The `scryfall` crate provides the type [`Query`] for specifying search
/// expressions. For advanced search, use [`SearchOptions`] to specify sorting,
/// unique rollup, and other options.
///
/// The `Search` trait is implemented for `&str` and `String` as well,
/// supporting custom searches using [scryfall.com syntax][https://scryfall.com/docs/syntax].
pub trait Search {
    /// Write this search as the query for the given `Url`.
    fn write_query(&self, url: &mut Url) -> crate::Result<()>;

    #[cfg(test)]
    fn query_string(&self) -> crate::Result<String> {
        let mut url = Url::parse("http://localhost")?;
        self.write_query(&mut url)?;
        Ok(url.query().unwrap_or_default().to_string())
    }

    /// Convenience method for passing this object to [`Card::search`].
    fn search(&self) -> crate::Result<ListIter<Card>>
    where
        Self: Sized,
    {
        Card::search_new(self)
    }

    /// Convenience method for passing this object to [`Card::search_random`].
    fn random(&self) -> crate::Result<Card>
    where
        Self: Sized,
    {
        Card::search_random_new(self)
    }
}

impl<T: Search> Search for &T {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        <T as Search>::write_query(*self, url)
    }
}

impl<T: Search> Search for &mut T {
    fn write_query(&self, url: &mut Url) -> crate::Result<()> {
        <T as Search>::write_query(*self, url)
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

/// Advanced searching options for Scryfall, including unique de-duplication
/// strategy, sort order, page number, and any extras to include. For
/// documentation on each option, refer to this struct's methods.
///
/// For more information, refer to the [official docs](https://scryfall.com/docs/api/cards/search).
#[derive(Serialize, Default, Debug)]
pub struct SearchOptions {
    #[serde(skip_serializing_if = "is_default")]
    unique: UniqueStrategy,
    #[serde(skip_serializing_if = "is_default")]
    order: SortMethod,
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
    /// Constructs a new `SearchOptions` with default values and an empty query.
    pub fn new() -> Self {
        SearchOptions {
            page: 1,
            ..Default::default()
        }
    }

    /// Constructs a new `SearchOptions` with default values and the specified
    /// `query`.
    pub fn with_query(query: Query) -> Self {
        SearchOptions {
            query,
            ..Self::new()
        }
    }

    /// Sets the [`Query`] to use for this search.
    pub fn query(&mut self, query: Query) -> &mut Self {
        self.query = query;
        self
    }

    /// Sets the strategy for omitting similar cards.
    pub fn unique(&mut self, unique: UniqueStrategy) -> &mut Self {
        self.unique = unique;
        self
    }

    /// Sets the method and direction to sort returned cards.
    pub fn sorted(&mut self, sort_by: SortMethod, dir: SortDirection) -> &mut Self {
        self.order = sort_by;
        self.dir = dir;
        self
    }

    /// If true, extra cards (tokens, planes, etc) will be included.
    pub fn extras(&mut self, include_extras: bool) -> &mut Self {
        self.include_extras = include_extras;
        self
    }

    /// If true, cards in every language supported by Scryfall will be included.
    pub fn multilingual(&mut self, include_multilingual: bool) -> &mut Self {
        self.include_multilingual = include_multilingual;
        self
    }

    /// If true, rare care variants will be included, like the
    /// [Hairy Runesword](https://scryfall.com/card/drk/107%E2%80%A0/runesword).
    pub fn variations(&mut self, include_variations: bool) -> &mut Self {
        self.include_variations = include_variations;
        self
    }
}

/// The unique parameter specifies if Scryfall should remove “duplicate” results
/// in your query.
#[derive(Serialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum UniqueStrategy {
    /// Removes duplicate gameplay objects (cards that share a name and have the
    /// same functionality). For example, if your search matches more than
    /// one print of Pacifism, only one copy of Pacifism will be returned.
    Cards,
    /// Returns only one copy of each unique artwork for matching cards. For
    /// example, if your search matches more than one print of Pacifism, one
    /// card with each different illustration for Pacifism will be returned,
    /// but any cards that duplicate artwork already in the results will
    /// be omitted.
    Art,
    /// Returns all prints for all cards matched (disables rollup). For example,
    /// if your search matches more than one print of Pacifism, all matching
    /// prints will be returned.
    Prints,
}

impl Default for UniqueStrategy {
    fn default() -> Self {
        UniqueStrategy::Cards
    }
}

/// The order parameter determines how Scryfall should sort the returned cards.
#[derive(Serialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SortMethod {
    /// Sort cards by name, A → Z
    Name,
    /// Sort cards by their set and collector number: AAA/#1 → ZZZ/#999
    Set,
    /// Sort cards by their release date: Newest → Oldest
    Released,
    /// Sort cards by their rarity: Common → Mythic
    Rarity,
    /// Sort cards by their color and color identity: WUBRG → multicolor →
    /// colorless
    Color,
    /// Sort cards by their lowest known U.S. Dollar price: 0.01 → highest, null
    /// last
    Usd,
    /// Sort cards by their lowest known TIX price: 0.01 → highest, null last
    Tix,
    /// Sort cards by their lowest known Euro price: 0.01 → highest, null last
    Eur,
    /// Sort cards by their converted mana cost: 0 → highest
    Cmc,
    /// Sort cards by their power: null → highest
    Power,
    /// Sort cards by their toughness: null → highest
    Toughness,
    /// Sort cards by their EDHREC ranking: lowest → highest
    Edhrec,
    /// Sort cards by their front-side artist name: A → Z
    Artist,
}

impl Default for SortMethod {
    fn default() -> Self {
        SortMethod::Name
    }
}

/// Which direction the sorting should occur:
#[derive(Serialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    /// Scryfall will automatically choose the most intuitive direction to sort
    Auto,
    /// Sort ascending (flip the direction of the arrows in [`SortMethod`])
    ///
    /// [`SortMethod`]: enum.SortMethod.html
    #[serde(rename = "asc")]
    Ascending,
    /// Sort descending (flip the direction of the arrows in [`SortMethod`])
    ///
    /// [`SortMethod`]: enum.SortMethod.html
    #[serde(rename = "desc")]
    Descending,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Auto
    }
}

/// A search query, composed of search parameters and boolean operations.
///
/// For information on search parameters, see [`Param`].
#[derive(PartialEq, Debug)]
pub struct Query(QueryImpl);

// TODO(msmorgan): Move the docs from here to somewhere else?
#[derive(PartialEq, Debug)]
enum QueryImpl {
    /// The returned cards must match all of the sub-queries.
    And(Vec<Query>),
    /// The returned cards must match at least one of the sub-queries.
    Or(Vec<Query>),
    /// The returned cards must not match the sub-query.
    Not(Box<Query>),
    /// The returned cards must match the specified search param.
    Param(Param),
    /// Empty query, used as a default value. Attempting to search with an empty
    /// query will result in a failure response.
    Empty,
}

impl Default for Query {
    fn default() -> Self {
        Query(QueryImpl::Empty)
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (exprs, sep) = match &self.0 {
            QueryImpl::And(exprs) => (exprs, " AND "),
            QueryImpl::Or(exprs) => (exprs, " OR "),
            QueryImpl::Not(expr) => return write!(f, "-{}", expr),
            QueryImpl::Param(param) => return write!(f, "{}", param),
            QueryImpl::Empty => return write!(f, ""),
        };

        use itertools::Itertools;
        write!(f, "({})", exprs.iter().format(sep))
    }
}

impl From<Param> for Query {
    fn from(param: Param) -> Self {
        Query(QueryImpl::Param(param))
    }
}

impl Query {
    /// Combines this query with `other` using the boolean AND operation.
    pub fn and(self, other: Self) -> Query {
        Query(match (self.0, other.0) {
            (QueryImpl::Empty, q) | (q, QueryImpl::Empty) => q,
            (QueryImpl::And(mut a_list), QueryImpl::And(mut b_list)) => {
                a_list.append(&mut b_list);
                QueryImpl::And(a_list)
            },
            (QueryImpl::And(mut a_list), b) => {
                a_list.push(Query(b));
                QueryImpl::And(a_list)
            },
            (a, QueryImpl::And(mut b_list)) => {
                b_list.insert(0, Query(a));
                QueryImpl::And(b_list)
            },
            (a, b) => QueryImpl::And(vec![Query(a), Query(b)]),
        })
    }

    /// Combines this query with `other` using the boolean OR operation.
    pub fn or(self, other: Self) -> Query {
        Query(match (self.0, other.0) {
            (QueryImpl::Empty, q) | (q, QueryImpl::Empty) => q,
            (QueryImpl::Or(mut a_list), QueryImpl::Or(mut b_list)) => {
                a_list.append(&mut b_list);
                QueryImpl::Or(a_list)
            },
            (QueryImpl::Or(mut a_list), b) => {
                a_list.push(Query(b));
                QueryImpl::Or(a_list)
            },
            (a, QueryImpl::Or(mut b_list)) => {
                b_list.insert(0, Query(a));
                QueryImpl::Or(b_list)
            },
            (a, b) => QueryImpl::Or(vec![Query(a), Query(b)]),
        })
    }
}

mod query_fns {
    use super::*;

    /// Combines the specified `queries` using the boolean AND operation.
    pub fn and(queries: impl IntoIterator<Item = Query>) -> Query {
        let mut result = Query(QueryImpl::Empty);
        for query in queries {
            result = result.and(query);
        }
        result
    }

    /// Combines the specified `queries` using the boolean OR operation.
    pub fn or(queries: impl IntoIterator<Item = Query>) -> Query {
        let mut result = Query(QueryImpl::Empty);
        for query in queries {
            result = result.and(query);
        }
        result
    }

    /// Negates the specified `query`.
    pub fn not(query: Query) -> Query {
        Query(match query.0 {
            QueryImpl::Not(q) => (*q).0,
            QueryImpl::Empty => QueryImpl::Empty,
            q => QueryImpl::Not(Box::new(Query(q))),
        })
    }
}

/// A filter to provide to the search to reduce the cards returned.
///
/// For more information on available parameters, refer to the
/// [official docs](https://scryfall.com/docs/syntax).
#[derive(Clone, Debug)]
pub struct Param(ParamImpl);

impl Param {
    fn property(prop: Property) -> Self {
        Param(ParamImpl::Property(prop))
    }

    fn value(kind: ValueKind, value: impl 'static + ParamValue) -> Self {
        Param(ParamImpl::Value(kind, None, Lrc::new(value)))
    }

    fn cmp_value(kind: ValueKind, op: CompareOp, value: impl 'static + ParamValue) -> Self {
        Param(ParamImpl::Value(kind, Some(op), Lrc::new(value)))
    }
}

#[derive(Clone, Debug)]
enum ParamImpl {
    Property(Property),
    Value(ValueKind, Option<CompareOp>, Lrc<dyn ParamValue>),
}

impl PartialEq for Param {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            ParamImpl::Property(prop) => write!(f, "{}", prop),
            ParamImpl::Value(ValueKind(ValueKindImpl::Exact), None, value) => {
                write!(f, "!{}", value)
            },
            ParamImpl::Value(kind, op, value) => {
                write!(f, "{}{}{}", kind, compare_op_str(*op), value)
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

    // TODO(msmorgan): Docs for these functions.
    macro_rules! param_fns {
        ($func:ident => $Kind:ident : $Constraint:ident, $($rest:tt)*) => {
            #[allow(missing_docs)]
            pub fn $func(value: impl 'static + $Constraint) -> Query {
                Query(QueryImpl::Param(value.into_param(ValueKind(ValueKindImpl::$Kind))))
            }

            param_fns!($($rest)*);
        };

        ($func:ident => NumericComparable($Kind:ident), $($rest:tt)*) => {
            #[allow(missing_docs)]
            pub fn $func(value: impl 'static + NumericComparableValue) -> Query {
                Query(QueryImpl::Param(value.into_param(ValueKind(ValueKindImpl::NumericComparable(
                    NumericProperty::$Kind,
                )))))
            }

            param_fns!($($rest)*);
        };

        () => {};
    }

    /// Match a card with a specified [`Property`].
    pub fn prop(prop: Property) -> Query {
        Query(QueryImpl::Param(Param::property(prop)))
    }

    param_fns! {
        color => Color: ColorValue,
        color_identity => ColorIdentity: ColorValue,
        type_line => Type: TextOrRegexValue,
        oracle_text => Oracle: TextOrRegexValue,
        full_oracle_text => FullOracle: TextOrRegexValue,
        keyword => Keyword: TextValue,
        mana => Mana: ColorValue,
        devotion => Devotion: DevotionValue,
        produces => Produces: ColorValue,
        rarity => Rarity: RarityValue,
        in_rarity => InRarity: RarityValue,
        set => Set: SetValue,
        in_set => InSet: SetValue,
        number => Number: NumericValue,
        block => Block: SetValue,
        set_type => SetType: SetTypeValue,
        in_set_type => InSetType: SetTypeValue,
        cube => Cube: CubeValue,
        legal => Format: FormatValue,
        banned => Banned: FormatValue,
        restricted => Restricted: FormatValue,
        cheapest => Cheapest: CurrencyValue,
        artist => Artist: TextValue,
        flavor => Flavor: TextOrRegexValue,
        watermark => Watermark: TextValue,
        border_color => BorderColor: BorderColorValue,
        frame => Frame: FrameValue,
        date => Date: DateValue,
        game => Game: GameValue,
        in_game => InGame: GameValue,
        language => Language: LanguageValue,
        in_language => InLanguage: LanguageValue,
        name => Name: TextOrRegexValue,
        exact => Exact: TextValue,

        power => NumericComparable(Power),
        toughness => NumericComparable(Toughness),
        pow_tou => NumericComparable(PowTou),
        loyalty => NumericComparable(Loyalty),
        cmc => NumericComparable(Cmc),
        artist_count => NumericComparable(ArtistCount),
        usd => NumericComparable(Usd),
        usd_foil => NumericComparable(UsdFoil),
        eur => NumericComparable(Eur),
        tix => NumericComparable(Tix),
        illustration_count => NumericComparable(IllustrationCount),
        print_count => NumericComparable(PrintCount),
        set_count => NumericComparable(SetCount),
        paper_print_count => NumericComparable(PaperPrintCount),
        paper_set_count => NumericComparable(PaperSetCount),
        year => NumericComparable(Year),
    }
}

// TODO(msmorgan): Expand on these docs to explain different types of props.
/// A property is a boolean flag associated with a card or printing.
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

    // TODO(msmorgan): Docs and examples for all land families.
    // TODO(msmorgan): Rename to Bicycle/BiCycle?
    #[allow(missing_docs)]
    IsCyclingDualLand,
    // TODO(msmorgan): Rename to Tricycle/TriCycle?
    #[allow(missing_docs)]
    IsCyclingTriLand,
    #[allow(missing_docs)]
    IsBounceLand,
    #[allow(missing_docs)]
    IsCanopyLand,
    #[allow(missing_docs)]
    IsCheckLand,
    #[allow(missing_docs)]
    IsDualLand,
    #[allow(missing_docs)]
    IsFastLand,
    #[allow(missing_docs)]
    IsFetchLand,
    #[allow(missing_docs)]
    IsFilterLand,
    #[allow(missing_docs)]
    IsGainLand,
    #[allow(missing_docs)]
    IsPainLand,
    #[allow(missing_docs)]
    IsScryLand,
    #[allow(missing_docs)]
    IsShadowLand,
    #[allow(missing_docs)]
    IsShockLand,
    #[allow(missing_docs)]
    IsStorageLand,
    #[allow(missing_docs)]
    IsCreatureLand,
    #[allow(missing_docs)]
    IsTriLand,
    #[allow(missing_docs)]
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
                Property::NewArt
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
                Property::IsCyclingDualLand => "bicycle_land",
                Property::IsCyclingTriLand => "tricycle_land",
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ValueKind(ValueKindImpl);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum ValueKindImpl {
    Color,
    ColorIdentity,
    Type,
    Oracle,
    FullOracle,
    Keyword,
    Mana,
    Devotion,
    Produces,
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
    Cheapest,
    Artist,
    Flavor,
    Watermark,
    BorderColor,
    Frame,
    Date,
    Game,
    InGame,
    Language,
    InLanguage,
    Name,
    Exact,
    NumericComparable(NumericProperty),
}

/// These properties can be compared against one another.
///
/// For example `power(gt(NumericProperty::Toughness)`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum NumericProperty {
    Power,
    Toughness,
    PowTou,
    Loyalty,
    Cmc,
    ArtistCount,
    Usd,
    UsdFoil,
    Eur,
    Tix,
    IllustrationCount,
    PrintCount,
    SetCount,
    PaperPrintCount,
    PaperSetCount,
    Year,
}

const fn numeric_property_str(prop: NumericProperty) -> &'static str {
    match prop {
        NumericProperty::Power => "power",
        NumericProperty::Toughness => "toughness",
        NumericProperty::PowTou => "powtou",
        NumericProperty::Loyalty => "loyalty",
        NumericProperty::Cmc => "cmc",
        NumericProperty::ArtistCount => "artists",
        NumericProperty::Usd => "usd",
        NumericProperty::UsdFoil => "usdfoil",
        NumericProperty::Eur => "eur",
        NumericProperty::Tix => "tix",
        NumericProperty::IllustrationCount => "illustrations",
        NumericProperty::PrintCount => "prints",
        NumericProperty::SetCount => "sets",
        NumericProperty::PaperPrintCount => "paperprints",
        NumericProperty::PaperSetCount => "papersets",
        NumericProperty::Year => "year",
    }
}

impl fmt::Display for NumericProperty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(numeric_property_str(*self))
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self.0 {
                ValueKindImpl::Color => "color",
                ValueKindImpl::ColorIdentity => "identity",
                ValueKindImpl::Type => "type",
                ValueKindImpl::Oracle => "oracle",
                ValueKindImpl::FullOracle => "fulloracle",
                ValueKindImpl::Keyword => "keyword",
                ValueKindImpl::Mana => "mana",
                ValueKindImpl::Devotion => "devotion",
                ValueKindImpl::Produces => "produces",
                ValueKindImpl::Rarity => "rarity",
                ValueKindImpl::Set => "set",
                ValueKindImpl::Number => "number",
                ValueKindImpl::Block => "block",
                ValueKindImpl::SetType => "settype",
                ValueKindImpl::Cube => "cube",
                ValueKindImpl::Format => "format",
                ValueKindImpl::Banned => "banned",
                ValueKindImpl::Restricted => "restricted",
                ValueKindImpl::Cheapest => "cheapest",
                ValueKindImpl::Artist => "artist",
                ValueKindImpl::Flavor => "flavor",
                ValueKindImpl::Watermark => "watermark",
                ValueKindImpl::BorderColor => "border",
                ValueKindImpl::Frame => "frame",
                ValueKindImpl::Date => "date",
                ValueKindImpl::Game => "game",
                ValueKindImpl::Language => "language",
                ValueKindImpl::InRarity
                | ValueKindImpl::InSet
                | ValueKindImpl::InSetType
                | ValueKindImpl::InGame
                | ValueKindImpl::InLanguage => "in",
                ValueKindImpl::Name => "name",
                ValueKindImpl::NumericComparable(np) => numeric_property_str(*np),
                // TODO(msmorgan): Should this just be unreachable?
                ValueKindImpl::Exact => return Err(fmt::Error::default()),
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum CompareOp {
    Lte,
    Lt,
    Gte,
    Gt,
    Eq,
    Neq,
}

const fn compare_op_str(op: Option<CompareOp>) -> &'static str {
    match op {
        None => ":",
        Some(CompareOp::Lte) => "<=",
        Some(CompareOp::Lt) => "<",
        Some(CompareOp::Gte) => ">=",
        Some(CompareOp::Gt) => ">",
        Some(CompareOp::Eq) => "=",
        Some(CompareOp::Neq) => "!=",
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Compare<T> {
    op: CompareOp,
    value: T,
}

impl<T: fmt::Display> fmt::Display for Compare<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", compare_op_str(Some(self.op)), &self.value)
    }
}

mod compare_fns {
    use super::*;

    macro_rules! compare_fns {
        ($($meth:ident => $Variant:ident,)*) => {
            $(
                pub fn $meth<T>(x: T) -> Compare<T> {
                    Compare {
                        op: CompareOp::$Variant,
                        value: x,
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
    fn into_param(self, kind: ValueKind) -> Param
    where
        Self: 'static + Sized,
    {
        Param::value(kind, self)
    }
}

impl<T: 'static + ParamValue> ParamValue for Compare<T> {
    fn into_param(self, kind: ValueKind) -> Param {
        Param::cmp_value(kind, self.op, self.value)
    }
}

// TODO(msmorgan): Impls for `Guild` etc.
pub trait ColorValue: ParamValue {}

// TODO(msmorgan): Maybe add `color_count` and `produces_count` etc instead.
impl<T: NumericValue> ColorValue for T {}

impl<T: 'static + ColorValue> ColorValue for Compare<T> {}

impl ParamValue for crate::card::Color {}

/// Color parameters allow to query by specific colors
impl ColorValue for crate::card::Color {}

impl ParamValue for crate::card::Colors {}

impl ColorValue for crate::card::Colors {}

// impl<T: TextValue> ColorValue for T {}

/// Devotion works differently than other color parameters. All the color
/// symbols must match and the symbols can be hybrid mana.
pub trait DevotionValue: ParamValue {}

pub struct Devotion(crate::card::Color, usize);

pub trait NumericValue: ParamValue {}

macro_rules! impl_numeric_values {
    ($($Ty:ty,)*) => {
        $(
            impl ParamValue for $Ty {}
            impl NumericValue for $Ty {}
            impl NumericComparableValue for $Ty {}
        )*
    };
}

#[rustfmt::skip]
impl_numeric_values!(
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f32, f64,
);

pub trait NumericComparableValue: ParamValue {}

impl<T: 'static + NumericComparableValue> NumericComparableValue for Compare<T> {}

impl ParamValue for NumericProperty {
    fn into_param(self, kind: ValueKind) -> Param {
        numeric_property_str(self).into_param(kind)
    }
}

impl NumericComparableValue for NumericProperty {}

pub trait TextValue: ParamValue {}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Quoted<T>(T);

impl<T: fmt::Display> fmt::Display for Quoted<T> {
    // TODO(msmorgan): This breaks if the value has quotes in it.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

impl ParamValue for Quoted<String> {
    fn into_param(self, kind: ValueKind) -> Param {
        Param::value(kind, self)
    }
}

impl TextValue for Quoted<String> {}

impl ParamValue for String {
    fn into_param(self, kind: ValueKind) -> Param {
        Quoted(self).into_param(kind)
    }
}

impl TextValue for String {}

impl ParamValue for &str {
    fn into_param(self, kind: ValueKind) -> Param {
        self.to_string().into_param(kind)
    }
}

impl TextValue for &str {}

pub trait TextOrRegexValue: ParamValue {}

impl<T: TextValue> TextOrRegexValue for T {}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Regex(String);

impl fmt::Display for Regex {
    // TODO(msmorgan): Escapes.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}/", self.0)
    }
}

impl ParamValue for Regex {}

impl TextOrRegexValue for Regex {}

/// A parameter that
pub trait RarityValue: ParamValue {}

impl<T: TextValue> RarityValue for T {}

impl ParamValue for crate::card::Rarity {}

impl RarityValue for crate::card::Rarity {}

impl RarityValue for Compare<crate::card::Rarity> {}

pub trait SetValue: ParamValue {}

pub trait CubeValue: ParamValue {}

impl<T: TextValue> CubeValue for T {}

pub trait FormatValue: ParamValue {}

impl<T: TextValue> FormatValue for T {}

impl ParamValue for crate::format::Format {}

impl FormatValue for crate::format::Format {}

pub trait CurrencyValue: ParamValue {}

impl<T: TextValue> CurrencyValue for T {}

// TODO(msmorgan): Make a currency enum.

pub trait SetTypeValue: ParamValue {}

impl<T: TextValue> SetTypeValue for T {}

pub trait BorderColorValue: ParamValue {}

impl<T: TextValue> BorderColorValue for T {}

impl ParamValue for crate::card::BorderColor {}

impl BorderColorValue for crate::card::BorderColor {}

/// A parameter with card frames and frame effects.
pub trait FrameValue: ParamValue {}

impl<T: TextValue> FrameValue for T {}

impl ParamValue for crate::card::FrameEffect {}

impl FrameValue for crate::card::FrameEffect {}

impl ParamValue for crate::card::Frame {}

impl FrameValue for crate::card::Frame {}

/// A parameter that specifies a date.
/// TODO(msmorgan): What is the date format?
/// TODO(msmorgan): Implement for chrono types.
pub trait DateValue: ParamValue {}

/// A parameter that specifies a game that the card appears in.
/// Valid for any `TextValue` and for [`Game`][crate::card::Game].
pub trait GameValue: ParamValue {}

impl<T: TextValue> GameValue for T {}

impl ParamValue for crate::card::Game {}

impl GameValue for crate::card::Game {}

pub trait LanguageValue: ParamValue {}

impl<T: TextValue> LanguageValue for T {}

pub mod prelude {
    pub use super::compare_fns::*;
    pub use super::param_fns::*;
    pub use super::query_fns::*;
    pub use super::{
        BorderColorValue as _,
        Compare,
        CubeValue as _,
        CurrencyValue as _,
        DateValue as _,
        FormatValue as _,
        FrameValue as _,
        GameValue as _,
        LanguageValue as _,
        NumericComparableValue as _,
        NumericProperty,
        NumericValue as _,
        ParamValue,
        Property,
        Query,
        RarityValue as _,
        Search,
        SearchOptions,
        SetTypeValue as _,
        SetValue as _,
        SortDirection,
        SortMethod,
        TextOrRegexValue,
        TextValue,
        UniqueStrategy,
    };
    pub use crate::card::{BorderColor, Frame, FrameEffect, Game, Rarity};
    pub use crate::set::{SetCode, SetType};
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use crate::Card;

    #[test]
    fn basic_search() {
        let cards = SearchOptions::new()
            .query(and(vec![name("lightning"), name("helix"), cmc(eq(2))]))
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
        use strum::IntoEnumIterator;

        for p in Property::iter() {
            let query = prop(p);
            query
                .random()
                .unwrap_or_else(|_| panic!("Could not get a random card with {}", p));
        }
    }

    #[test]
    fn finds_alpha_lotus() {
        let mut search = SearchOptions::new();

        search
            .query(exact("Black Lotus"))
            .unique(UniqueStrategy::Prints)
            .sorted(SortMethod::Released, SortDirection::Ascending);

        eprintln!("{}", search.query_string().unwrap());

        assert_eq!(
            Card::search_new(&search)
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
                .set
                .to_string(),
            "lea",
        );
    }

    #[test]
    fn rarity_comparison() {
        // The cards with "Bonus" rarity (power nine in vma).
        let cards = SearchOptions::new()
            .query(rarity(gt(Rarity::Mythic)))
            .search()
            .unwrap()
            .collect::<Vec<_>>();

        assert!(cards.len() >= 9, "Couldn't find the Power Nine from VMA.");

        assert!(
            cards
                .into_iter()
                .map(|c| c.unwrap())
                .all(|c| c.rarity > Rarity::Mythic)
        );
    }

    #[test]
    fn numeric_property_comparison() {
        let card = Card::search_random_new(and(vec![
            power(eq(NumericProperty::Toughness)),
            pow_tou(eq(NumericProperty::Cmc)),
            not(prop(Property::IsFunny)),
        ]))
        .unwrap();

        let power = card.power.unwrap().parse::<u32>().unwrap();
        let toughness = card.toughness.unwrap().parse::<u32>().unwrap();

        assert_eq!(power, toughness);
        assert_eq!(power + toughness, card.cmc as u32);
    }

    #[test]
    fn query_string_sanity_check() {
        let query = cmc(4).and(name("Yargle"));
        assert_eq!(
            query.query_string().unwrap(),
            "q=%28cmc%3A4+AND+name%3A%22Yargle%22%29"
        );
    }
}
