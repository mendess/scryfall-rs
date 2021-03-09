#![warn(missing_docs)]

use std::fmt;
use std::hash::Hash;
use std::rc::Rc as Lrc;

use serde::{Serialize, Serializer};
use url::Url;

pub use self::color_aliases::*;
pub use self::compare_fns::*;
pub use self::param_fns::*;
use crate::list::ListIter;
use crate::Card;

mod color_aliases;

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
// TODO(msmorgan): Move the docs from here to somewhere else?
#[derive(PartialEq, Debug)]
pub enum Query {
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
    #[doc(hidden)]
    Empty,
}

impl Default for Query {
    fn default() -> Self {
        Query::Empty
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (exprs, sep) = match &self {
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

impl Query {
    /// Combines this query with `other` using the boolean AND operation.
    pub fn and(self, other: Self) -> Query {
        match (self, other) {
            (Query::Empty, q) | (q, Query::Empty) => q,
            (Query::And(mut a_list), Query::And(mut b_list)) => {
                a_list.append(&mut b_list);
                Query::And(a_list)
            },
            (Query::And(mut a_list), b) => {
                a_list.push(b);
                Query::And(a_list)
            },
            (a, Query::And(mut b_list)) => {
                b_list.insert(0, a);
                Query::And(b_list)
            },
            (a, b) => Query::And(vec![a, b]),
        }
    }

    /// Combines this query with `other` using the boolean OR operation.
    pub fn or(self, other: Self) -> Query {
        match (self, other) {
            (Query::Empty, q) | (q, Query::Empty) => q,
            (Query::Or(mut a_list), Query::Or(mut b_list)) => {
                a_list.append(&mut b_list);
                Query::Or(a_list)
            },
            (Query::Or(mut a_list), b) => {
                a_list.push(b);
                Query::Or(a_list)
            },
            (a, Query::Or(mut b_list)) => {
                b_list.insert(0, a);
                Query::Or(b_list)
            },
            (a, b) => Query::Or(vec![a, b]),
        }
    }
}

mod query_fns {
    use super::*;

    /// Negates the specified `query`.
    pub fn not(query: Query) -> Query {
        match query {
            Query::Not(q) => *q,
            Query::Empty => Query::Empty,
            q => Query::Not(Box::new(q)),
        }
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
        (
            $(#[$($attr:meta)*])*
            $func:ident => $Kind:ident : $Constraint:ident,
            $($rest:tt)*
        ) => {
            $(#[$($attr)*])*
            pub fn $func(value: impl 'static + $Constraint) -> Query {
                Query::Param(value.into_param(ValueKind(ValueKindImpl::$Kind)))
            }

            param_fns!($($rest)*);
        };

        (
            $(#[$($attr:meta)*])*
            $func:ident => NumericComparable($Kind:ident),
            $($rest:tt)*
        ) => {
            $(#[$($attr)*])*
            pub fn $func(value: impl 'static + NumericComparableValue) -> Query {
                Query::Param(value.into_param(ValueKind(
                    ValueKindImpl::NumericComparable(NumProperty::$Kind),
                )))
            }

            param_fns!($($rest)*);
        };

        () => {};
    }

    /// Match a card with a specified [`Property`].
    pub fn prop(prop: Property) -> Query {
        Query::Param(Param::property(prop))
    }

    param_fns! {
        #[doc = "The color of this card, based on indicator or cost."]
        color => Color: ColorValue,
        #[doc = "The number of colors of this card, based on indicator or cost."]
        color_count => Color: NumericValue,
        #[doc = "The color identity of this card, for Commander-like formats."]
        color_identity => ColorIdentity: ColorValue,
        #[doc = "The number of colors in this card's identity, for Commander-like formats."]
        color_identity_count => ColorIdentity: NumericValue,
        #[doc = "The type line of this card."]
        type_line => Type: TextOrRegexValue,
        #[doc = "The updated oracle text of this card."]
        oracle_text => Oracle: TextOrRegexValue,
        #[doc = "The updated oracle text of this card, including reminder text."]
        full_oracle_text => FullOracle: TextOrRegexValue,
        #[doc = "Keyword ability that this card has."]
        keyword => Keyword: TextValue,
        #[doc = "The mana cost of this card."]
        mana => Mana: ColorValue,
        #[doc = "The devotion granted by this permanent."]
        devotion => Devotion: DevotionValue,
        #[doc = "The colors of mana produced by this card."]
        produces => Produces: ColorValue,
        #[doc = "The rarity of this printing."]
        rarity => Rarity: RarityValue,
        #[doc = "Has the card ever been printed in this rarity?"]
        in_rarity => InRarity: RarityValue,
        #[doc = "The set code of this printing."]
        set => Set: SetValue,
        #[doc = "Was the card printed in this set?"]
        in_set => InSet: SetValue,
        #[doc = "The card's collector number."]
        number => Number: NumericValue,
        #[doc = "The block of this card. Works with any set grouped in the same block."]
        block => Block: SetValue,
        #[doc(hidden)]
        #[doc = "The type of set this printing is in."]
        set_type => SetType: SetTypeValue,
        #[doc = "Has the card appeared in a set of this type?"]
        in_set_type => InSetType: SetTypeValue,
        #[doc = "Does the card appear in this cube on MTGO?"]
        cube => Cube: CubeValue,
        #[doc(hidden)]
        format => Format: FormatValue,
        #[doc = "The card is legal in this format."]
        legal => Format: FormatValue,
        #[doc = "The card is banned in this format."]
        banned => Banned: FormatValue,
        #[doc = "The card is restricted in this format."]
        restricted => Restricted: FormatValue,
        #[doc = "Return the printing that is the cheapest in the specified currency."]
        cheapest => Cheapest: CurrencyValue,
        #[doc = "The artist who illustrated this card."]
        artist => Artist: TextValue,
        #[doc = "The flavor text of this printing."]
        flavor => Flavor: TextOrRegexValue,
        #[doc = "The type of watermark on this printing."]
        watermark => Watermark: TextValue,
        #[doc = "The border color of this printing."]
        border_color => BorderColor: BorderColorValue,
        #[doc = "The card frame of this printing, related to the year of the print."]
        frame => Frame: FrameValue,
        #[doc = "The date this printing was released."]
        date => Date: DateValue,
        #[doc = "This printing is available in the specified game."]
        game => Game: GameValue,
        #[doc = "This card is available in the specified game."]
        in_game => InGame: GameValue,
        #[doc = "This printing is in the specified language."]
        language => Language: LanguageValue,
        #[doc = "Has this card ever been printed in the specified language?"]
        in_language => InLanguage: LanguageValue,
        #[doc = "The card's name, using fuzzy search."]
        name => Name: TextOrRegexValue,
        #[doc = "The card's full, exact name."]
        exact => Exact: TextValue,

        #[doc = "The card's power, if it is a creature or vehicle. '*' and 'X' count as 0."]
        power => NumericComparable(Power),
        #[doc = "The card's toughness, if it is a creature or vehicle. '*' and 'X' count as 0."]
        toughness => NumericComparable(Toughness),
        #[doc = "The card's power plus its toughness."]
        pow_tou => NumericComparable(PowTou),
        #[doc = "The card's loyalty, if it is a planeswalker. 'X' counts as 0."]
        loyalty => NumericComparable(Loyalty),
        #[doc = "The converted mana cost of this card."]
        cmc => NumericComparable(Cmc),
        #[doc = "The number of artists credited for this printing."]
        artist_count => NumericComparable(ArtistCount),
        #[doc = "The current market price of this card in US Dollars."]
        usd => NumericComparable(Usd),
        #[doc = "The current foil market price of this card in US Dollars."]
        usd_foil => NumericComparable(UsdFoil),
        #[doc = "The current market price of this card in Euros."]
        eur => NumericComparable(Eur),
        #[doc = "The current market price of this card in MTGO tickets."]
        tix => NumericComparable(Tix),
        #[doc = "The number of unique art this card has had."]
        illustration_count => NumericComparable(IllustrationCount),
        #[doc = "The number of unique prints of this card."]
        print_count => NumericComparable(PrintCount),
        #[doc = "The number of sets this card has appeared in."]
        set_count => NumericComparable(SetCount),
        #[doc = "The number of unique prints of this card, counting paper only."]
        paper_print_count => NumericComparable(PaperPrintCount),
        #[doc = "The number of sets this card has appeared in, counting paper only."]
        paper_set_count => NumericComparable(PaperSetCount),
        #[doc = "The year this card was released."]
        year => NumericComparable(Year),
    }

    /// The card is not eligible to be legal in this format.
    pub fn not_legal(format: impl 'static + FormatValue + Clone) -> Query {
        Query::And(vec![
            Query::Not(Box::new(Query::Param(
                format.clone().into_param(ValueKind(ValueKindImpl::Format)),
            ))),
            Query::Not(Box::new(Query::Param(
                format.clone().into_param(ValueKind(ValueKindImpl::Banned)),
            ))),
            Query::Not(Box::new(Query::Param(
                format.into_param(ValueKind(ValueKindImpl::Restricted)),
            ))),
        ])
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

/// The type of parameter that this is. Corresponds to the name before the ':'
/// or other operator.
///
/// Refer to [the syntax documentation](https://scryfall.com/docs/syntax) for details on the
/// available parameter types.
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
    NumericComparable(NumProperty),
}

/// These properties can be compared against one another.
///
/// For example `power(gt(NumericProperty::Toughness)`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum NumProperty {
    /// The card's power. Only creature cards have this.
    Power,
    /// The card's toughness. Only creature cards have this.
    Toughness,
    /// The card's power plus its toughness. Only creatures cards have this.
    PowTou,
    /// The card's starting loyalty. Only planeswalker cards have this.
    ///
    /// The value '0' will match non-numeric loyalties such as 'X'.
    Loyalty,
    /// The card's converted mana cost. Cards without a mana cost have a
    /// converted mana cost of '0'.
    Cmc,
    /// The number of artists who contributed to this printing of the card.
    ///
    /// *Note*: This is not the same as the number of unique artists for a
    /// particular card.
    ArtistCount,
    /// This card's current nonfoil market price in US Dollars.
    Usd,
    /// This card's current foil market price in US Dollars.
    UsdFoil,
    /// This card's current market price in Euros.
    Eur,
    /// This card's current market price in MTGO Tickets.
    Tix,
    /// The number of different illustrations among prints of this card.
    IllustrationCount,
    /// The number of different prints of this card, including both paper and
    /// digital-exclusive sets.
    PrintCount,
    /// The number of different sets this card has appeared in, including both
    /// paper and digital-exclusive sets.
    SetCount,
    /// The number of different prints of this card in paper.
    PaperPrintCount,
    /// The number of different sets this card has appeared in, paper only.
    PaperSetCount,
    /// The release year of this printing.
    Year,
}

const fn numeric_property_str(prop: NumProperty) -> &'static str {
    match prop {
        NumProperty::Power => "power",
        NumProperty::Toughness => "toughness",
        NumProperty::PowTou => "powtou",
        NumProperty::Loyalty => "loyalty",
        NumProperty::Cmc => "cmc",
        NumProperty::ArtistCount => "artists",
        NumProperty::Usd => "usd",
        NumProperty::UsdFoil => "usdfoil",
        NumProperty::Eur => "eur",
        NumProperty::Tix => "tix",
        NumProperty::IllustrationCount => "illustrations",
        NumProperty::PrintCount => "prints",
        NumProperty::SetCount => "sets",
        NumProperty::PaperPrintCount => "paperprints",
        NumProperty::PaperSetCount => "papersets",
        NumProperty::Year => "year",
    }
}

impl fmt::Display for NumProperty {
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum CompareOp {
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

/// An operator and RHS for a comparison expression of a parameter.
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
        ($(
            $(#[$($attr:meta)*])*
            $meth:ident => $Variant:ident,
        )*) => {
            $(
                $(#[$($attr)*])*
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
        #[doc = "Less than `x`."]
        lt => Lt,
        #[doc = "Less than or equal to `x`."]
        lte => Lte,
        #[doc = "Greater than or equal to `x`."]
        gte => Gte,
        #[doc = "Greater than `x`."]
        gt => Gt,
        #[doc = "Equal to `x`."]
        eq => Eq,
        #[doc = "Not equal to `x`."]
        neq => Neq,
    }
}

/// The base type for a parameter value. The `into_param` function handles
/// converting the type into a [`Param`].
pub trait ParamValue: fmt::Debug + fmt::Display {
    /// Convert this value into a [`Param`] with the specified `kind`.
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

/// Color parameters allow querying by specific colors.
pub trait ColorValue: ParamValue {}

impl<T: 'static + ColorValue> ColorValue for Compare<T> {}

impl ParamValue for crate::card::Color {}
impl ColorValue for crate::card::Color {}

impl ParamValue for crate::card::Colors {}
impl ColorValue for crate::card::Colors {}

impl ParamValue for crate::card::Multicolored {}
impl ColorValue for crate::card::Multicolored {}

impl ParamValue for Guild {}
impl ColorValue for Guild {}

impl ParamValue for Shard {}
impl ColorValue for Shard {}

impl ParamValue for Wedge {}
impl ColorValue for Wedge {}

impl ParamValue for FourColor {}
impl ColorValue for FourColor {}

// TODO(msmorgan): Should text be a valid ColorValue?

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

impl ParamValue for NumProperty {
    fn into_param(self, kind: ValueKind) -> Param {
        numeric_property_str(self).into_param(kind)
    }
}
impl NumericComparableValue for NumProperty {}

pub trait TextValue: ParamValue {}

/// Helper struct for a quoted value. The `Display` impl for this struct
/// surrounds the value in quotes. Representations that contain quotes are
/// not supported.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Quoted<T>(T);

impl<T: fmt::Display> fmt::Display for Quoted<T> {
    // TODO(msmorgan): This breaks if the value has quotes in it.
    //     Scryfall does not support quote escaping.
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
    pub use super::color_aliases::*;
    pub use super::compare_fns::*;
    pub use super::param_fns::*;
    pub use super::query_fns::*;
    // Value types.
    pub use super::{
        BorderColorValue,
        ColorValue,
        Compare,
        CubeValue,
        CurrencyValue,
        DateValue,
        DevotionValue,
        FormatValue,
        FrameValue,
        GameValue,
        LanguageValue,
        NumProperty,
        NumericComparableValue,
        NumericValue,
        ParamValue,
        Property,
        Query,
        RarityValue,
        Search,
        SearchOptions,
        SetTypeValue,
        SetValue,
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
            .query(Query::And(vec![
                name("lightning"),
                name("helix"),
                cmc(eq(2)),
            ]))
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
        let card = Card::search_random_new(Query::And(vec![
            power(eq(NumProperty::Toughness)),
            pow_tou(eq(NumProperty::Cmc)),
            not(prop(Property::IsFunny)),
        ]))
        .unwrap();

        let power = card.power.unwrap().parse::<u32>().unwrap();
        let toughness = card.toughness.unwrap().parse::<u32>().unwrap();

        assert_eq!(power, toughness);
        assert_eq!(power + toughness, card.cmc as u32);

        let card = Card::search_new(pow_tou(gt(NumProperty::Year)))
            .unwrap()
            .map(|c| c.unwrap())
            .collect::<Vec<_>>();

        assert!(card.into_iter().any(|c| &c.name == "Infinity Elemental"));
    }

    #[test]
    fn query_string_sanity_check() {
        let query = cmc(4).and(name("Yargle"));
        assert_eq!(
            query.query_string().unwrap(),
            "q=%28cmc%3A4+AND+name%3A%22Yargle%22%29"
        );
    }

    #[test]
    #[ignore]
    fn all_color_aliases_work() {
        use strum::IntoEnumIterator;

        use crate::card::Colors;

        fn do_test<T>()
        where
            T: 'static + IntoEnumIterator + Clone + ColorValue + Into<Colors>,
        {
            for alias in T::iter() {
                let card = color(eq(alias.clone())).random().unwrap();
                assert_eq!(Colors::from(card.colors.unwrap().as_slice()), alias.into());
            }
        }

        do_test::<Guild>();
        do_test::<Shard>();
        do_test::<Wedge>();
        do_test::<FourColor>();
    }
}
