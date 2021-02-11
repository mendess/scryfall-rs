//! This module provides an abstraction over the search parameters provided by
//! `scryfall`. For a complete documentation, refer to the
//! [official site](https://scryfall.com/docs/syntax).
//!
//! The main struct of this module is the [`SearchBuilder`], this represents
//! a search compatible with `scryfall` search route.
//!
//! All types whose name ends in `Param` are parameters that can be passed to
//! the builder.
//!
//! [`SortMethod`], [`SortDirection`] and [`UniqueStrategy`] are options that
//! can be set on the builder.
//!
//!
//! Finally the [`Search`] trait, defines what is a valid search for `scryfall`
//! it's implemented for `String` in case it's easier for the user to directly
//! use a text representation.
//!
//! [`SortMethod`]: enum.SortMethod.html
//! [`SortDirection`]: enum.SortDirection.html
//! [`UniqueStrategy`]: enum.UniqueStrategy.html
//! [`SearchBuilder`]: struct.SearchBuilder.html
//! [`Search`]: trait.Search.html
use crate::card::{
    BorderColor, Colors, Frame, FrameEffect,
    Game, Rarity, Card,
};
use crate::format::Format;
use crate::set::SetCode;
use crate::util::uri::PaginatedURI;

use std::fmt::Write;
use std::str;

use percent_encoding::{percent_encode, CONTROLS};
use serde::{Deserialize, Serialize};

/// Search expresses that the implementing type can be turned into a query to
/// `scryfall`. This means that is should be
/// [properly encoded](https://en.wikipedia.org/wiki/Percent-encoding).
pub trait Search {
    /// Turns a searchable into it's string representation.
    fn to_query(&self) -> String;
}

impl<T> Search for &T
where
    T: Search,
{
    fn to_query(&self) -> String {
        Search::to_query(*self)
    }
}

impl<T> Search for &mut T
where
    T: Search,
{
    fn to_query(&self) -> String {
        Search::to_query(*self)
    }
}

impl Search for &str {
    /// This guarantees that the query is properly encoded. Be wary that you need
    /// to follow `scryfall` syntax.
    ///
    /// The use case of this implementation is usually this. (See [`Card::search`]
    /// for details)
    ///
    /// ```rust,no_run
    /// use scryfall::card::Card;
    /// assert!(Card::search("lightning")
    ///     .filter_map(|x| x.ok())
    ///     .flatten()
    ///     .all(|x| x.name.to_lowercase().contains("lightning")))
    /// ```
    ///
    /// [`Card::search`]: ../card/struct.Card.html#method.search
    fn to_query(&self) -> String {
        format!("q={}", percent_encode(self.as_bytes(), CONTROLS))
    }
}

/// Param expresses that the implementing type can be turned into a parameter
/// in a scryfall search parameters. The valid parameters can be seen
/// [here](https://scryfall.com/docs/syntax).
pub trait Param {
    /// Turns a parameter into it's string version.
    fn to_param(&self) -> String;
}

impl Param for String {
    fn to_param(&self) -> String {
        self.clone()
    }
}

impl<P> From<P> for Box<dyn Param>
where
    P: Param + 'static,
{
    fn from(param: P) -> Self {
        Box::new(param)
    }
}

/// A search builder for constructing a Search for `scryfall`. The various
/// parameters that can be passed to this struct are defined in this module.
///
/// A search is composed of settings and params.
/// ## Settings
/// The in depth documentation for the settings can be found
/// [here](https://scryfall.com/docs/api/cards/search)
/// - [`with_unique_strategy`]: The strategy used to reduce duplicates. (default: See [`UniqueStrategy`])
/// - [`sorting_by`]: The order in which results appear. (default: See [`SortMethod`])
/// - [`with_sort_direction`]: The sorting direction. (default: See [`SortDirection`])
/// - [`on_page`]: The page to start at. (default: 1)
/// - [`including_extras`]: Whether to include extras. (default: false)
/// - [`including_multilingual`]: Whether to include multilingual cards. (default: false)
/// - [`including_variations`]: Whether to include variations. (default: false)
///
/// ## Parameters
/// Parameters are filters to provide to the search to reduce the cards returned.
///
/// The official documentation for the parameters can be found
/// [here](https://scryfall.com/docs/syntax)
///
/// [`with_unique_strategy`]: #method.with_unique_strategy
/// [`sorting_by`]: #method.sorting_by
/// [`with_sort_direction`]: #method.with_sort_direction
/// [`on_page`]: #method.on_page
/// [`including_extras`]: #method.including_extras
/// [`including_multilingual`]: #method.including_multilingual
/// [`including_variations`]: #method.including_variations
/// [`UniqueStrategy`]: enum.UniqueStrategy.html
/// [`SortMethod`]: enum.SortMethod.html
/// [`SortDirection`]: enum.SortDirection.html
pub struct SearchBuilder {
    unique: UniqueStrategy,
    sort_by: SortMethod,
    dir: SortDirection,
    page: usize,
    include_extras: bool,
    include_multilingual: bool,
    include_variations: bool,
    params: Vec<Box<dyn Param>>,
}

impl Default for SearchBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchBuilder {
    /// Create a new search builder with the default values.
    pub fn new() -> Self {
        SearchBuilder {
            page: 1,
            unique: Default::default(),
            sort_by: Default::default(),
            dir: Default::default(),
            include_extras: false,
            include_multilingual: false,
            include_variations: false,
            params: Default::default(),
        }
    }

    /// Change the unique strategy to be used to remove repeated cards.
    pub fn with_unique_strategy(&mut self, strat: UniqueStrategy) -> &mut Self {
        self.unique = strat;
        self
    }

    /// Change the sorting method used for the results.
    pub fn sorting_by(&mut self, method: SortMethod) -> &mut Self {
        self.sort_by = method;
        self
    }

    /// Change the direction in which things are sorted.
    pub fn with_sort_direction(&mut self, dir: SortDirection) -> &mut Self {
        self.dir = dir;
        self
    }

    /// Enable the inclusion of extras.
    pub fn including_extras(&mut self) -> &mut Self {
        self.include_extras = true;
        self
    }

    /// Enable the inclusion of multilingual cards.
    pub fn including_multilingual(&mut self) -> &mut Self {
        self.include_multilingual = true;
        self
    }

    /// Enable the inclusion of variations on cards.
    pub fn including_variations(&mut self) -> &mut Self {
        self.include_variations = true;
        self
    }

    /// Change the starting page of the search.
    pub fn on_page(&mut self, page: usize) -> &mut Self {
        self.page = page;
        self
    }

    /// Add a param to the search.
    pub fn param<P>(&mut self, param: P) -> &mut Self
    where
        P: Into<Box<dyn Param>>,
    {
        self.params.push(param.into());
        self
    }

    /// A covenience method for passing this to a search.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::card::Card;
    /// use scryfall::card_searcher::{
    ///     NumericParam::CollectorNumber, Search, SearchBuilder, StringParam::Set,
    /// };
    /// use scryfall::set::SetCode;
    /// use std::convert::TryFrom;
    ///
    /// assert_eq!(
    ///     Card::search(
    ///         SearchBuilder::new()
    ///             .param(CollectorNumber(123))
    ///             .param(Set(SetCode::try_from("war").unwrap()))
    ///     )
    ///     .flatten()
    ///     .collect::<Vec<_>>(),
    ///     SearchBuilder::new()
    ///         .param(CollectorNumber(123))
    ///         .param(Set(SetCode::try_from("war").unwrap()))
    ///         .search()
    ///         .flatten()
    ///         .collect::<Vec<_>>()
    /// );
    /// ```
    pub fn search(&mut self) -> PaginatedURI<Card> {
        Card::search(self)
    }
}

impl Search for SearchBuilder {
    fn to_query(&self) -> String {
        use itertools::Itertools;
        let mut query = format!(
            "{}&{}&{}&",
            self.unique.to_param(),
            self.sort_by.to_param(),
            self.dir.to_param()
        );
        if self.include_extras {
            query += "include_extras=true";
        }
        if self.include_multilingual {
            query += "include_variations=true";
        }
        if self.include_variations {
            query += "include_multilingual=true";
        }
        if self.page > 1 {
            query += &format!("page={}", self.page + 1);
        }
        query += "q=";
        let _ = write!(
            query,
            "{}",
            percent_encode(
                self.params
                    .iter()
                    .map(|x| {
                        #[allow(clippy::redundant_closure)]
                        x.to_param()
                    })
                    .join("+")
                    .as_bytes(),
                CONTROLS,
            )
        );
        query
    }
}

/// The unique parameter specifies if Scryfall should remove “duplicate” results in your query.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum UniqueStrategy {
    /// Removes duplicate gameplay objects (cards that share a name and have the same
    /// functionality). For example, if your search matches more than one print of Pacifism, only one
    /// copy of Pacifism will be returned.
    Cards,
    /// Returns only one copy of each unique artwork for matching cards. For example, if
    /// your search matches more than one print of Pacifism, one card with each different illustration
    /// for Pacifism will be returned, but any cards that duplicate artwork already in the results will
    /// be omitted.
    Arts,
    /// Returns all prints for all cards matched (disables rollup). For example, if your
    /// search matches more than one print of Pacifism, all matching prints will be returned.
    Prints,
}

impl Default for UniqueStrategy {
    fn default() -> Self {
        UniqueStrategy::Cards
    }
}

impl Param for UniqueStrategy {
    fn to_param(&self) -> String {
        use UniqueStrategy::*;
        String::from("unique=")
            + match self {
                Cards => "cards",
                Arts => "art",
                Prints => "prints",
            }
    }
}

/// The order parameter determines how Scryfall should sort the returned cards.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SortMethod {
    /// Sort cards by name, A → Z
    Name,
    /// Sort cards by their set and collector number: AAA/#1 → ZZZ/#999
    Set,
    /// Sort cards by their release date: Newest → Oldest
    Released,
    /// Sort cards by their rarity: Common → Mythic
    Rarity,
    /// Sort cards by their color and color identity: WUBRG → multicolor → colorless
    Color,
    /// Sort cards by their lowest known U.S. Dollar price: 0.01 → highest, null last
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

impl Param for SortMethod {
    fn to_param(&self) -> String {
        use SortMethod::*;
        String::from("sort_by=")
            + match self {
                Name => "name",
                Set => "set",
                Released => "released",
                Rarity => "rarity",
                Color => "color",
                Usd => "usd",
                Tix => "tix",
                Eur => "eur",
                Cmc => "cmc",
                Power => "power",
                Toughness => "toughness",
                Edhrec => "edhrec",
                Artist => "artist",
            }
    }
}

/// Which direction the sorting should occur:
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SortDirection {
    /// Scryfall will automatically choose the most inuitive direction to sort
    Auto,
    /// Sort ascending (flip the direction of the arrows in [`SortMethod`])
    ///
    /// [`SortMethod`]: enum.SortMethod.html
    Ascending,
    /// Sort descending (flip the direction of the arrows in [`SortMethod`])
    ///
    /// [`SortMethod`]: enum.SortMethod.html
    Descending,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Auto
    }
}

impl Param for SortDirection {
    fn to_param(&self) -> String {
        use SortDirection::*;
        format!(
            "dir={}",
            match self {
                Auto => "auto",
                Ascending => "asc",
                Descending => "desc",
            }
        )
    }
}

/// Parameters that are either added or are `false`.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BooleanParam {
    /// Cards that have a color indicator.
    ColorIndicator,
    /// Cards that have a watermark.
    WaterMark,
    /// Find reprint cards printed at a new rarity for the first time.
    NewRarity,
    /// Find cards being printed with new illustrations.
    NewArt,
    /// Find cards being illustrated by a particular artist for the first time.
    NewArtist,
    /// Find cards being printed with brand-new flavor text using for the first time.
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
    /// Find cards with modal effects.
    IsModal,
    /// Find vanilla creatures.
    IsVanilla,
    /// Find Un-cards, holiday cards, and other funny cards.
    IsFunny,
    /// Find cards that can be your commander.
    IsCommander,
    /// Find cards on the reserved list.
    IsReserved,
    /// Find cards with full art.
    IsFull,
    /// Find non-foil printings of cards.
    IsNonFoil,
    /// Find foil printings of cards.
    IsFoil,
    /// Find cards in `scryfall`'s database with high-resolution images.
    IsHires,
    /// Find prints that are only available digitally (MTGO and Arena)
    IsDigital,
    /// Find promotional cards.
    IsPromo,
    /// Find cards that are Story Spotlights.
    IsSpotlight,
    /// Find cards that have only been in a single set.
    IsUnique,
    /// Find reprints.
    IsReprint,
    /// Find cards that were sold in boosters.
    SoldInBoosters,
    /// Find cards that were sold in planeswalker decks.
    SoldInPWDecks,
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
}

impl Param for BooleanParam {
    fn to_param(&self) -> String {
        use BooleanParam::*;
        format!(
            "{}:{}",
            match self {
                ColorIndicator | WaterMark => "has",
                NewRarity | NewArt | NewFlavor | NewArtist | NewFrame | NewLanguage => "new",
                _ => "is",
            },
            match self {
                ColorIndicator => "indicator",
                WaterMark => "watermark",
                NewRarity => "rarity",
                NewArt => "art",
                NewFlavor => "flavor",
                NewArtist => "artist",
                NewFrame => "frame",
                NewLanguage => "language",
                IsPhyrexian => "phyrexian",
                IsHybrid => "hybrid",
                IsSplit => "split",
                IsFlip => "flip",
                IsTransform => "transform",
                IsMeld => "meld",
                IsLeveler => "leveler",
                IsSpell => "spell",
                IsPermanent => "permanent",
                IsHistoric => "historic",
                IsModal => "modal",
                IsVanilla => "vanilla",
                IsFunny => "funny",
                IsFull => "full",
                IsFoil => "foil",
                IsNonFoil => "nonfoil",
                IsCommander => "commander",
                IsReserved => "reserved",
                IsHires => "hires",
                IsDigital => "digital",
                IsPromo => "promo",
                IsSpotlight => "spotlight",
                IsReprint => "reprint",
                IsUnique => "unique",
                SoldInBoosters => "boosters",
                SoldInPWDecks => "planeswalker_deck",
                SoldInLeague => "league",
                SoldInBuyABox => "buyabox",
                SoldInGiftBox => "giftbox",
                SoldInIntroPack => "intro_pack",
                SoldInGameDay => "gameday",
                SoldInPreRelease => "prerelease",
                SoldInRelease => "release",
            }
        )
    }
}

/// Some filters require a comparison expression.
///
/// # Examples
/// The cmc of a spell can be filtered like this.
/// ```rust
/// use scryfall::card_searcher::{ComparisonExpr, NumericParam, Param};
///
/// assert_eq!(NumericParam::CMC(ComparisonExpr::AtLeast, 3).to_param(), "cmc>3")
/// ```
///
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ComparisonExpr {
    /// `>`
    AtLeast,
    /// `>=`
    AtLeastInclusive,
    /// `<`
    AtMost,
    /// `<=`
    AtMostInclusive,
    /// `=`
    Is,
    /// `!=`
    IsNot,
}

/// [`ComparisonExpr::Is`] (aka `=`)
///
/// [`ComparisonExpr::Is`]: enum.ComparisonExpr.html#variant.Is
impl Default for ComparisonExpr {
    fn default() -> Self {
        ComparisonExpr::Is
    }
}

impl std::fmt::Display for ComparisonExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ComparisonExpr::*;
        write!(
            f,
            "{}",
            match self {
                AtLeast => ">",
                AtLeastInclusive => ">=",
                AtMost => "<",
                AtMostInclusive => "<=",
                Is => "=",
                IsNot => "!=",
            }
        )
    }
}

/// A parameter that takes a string as its value.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub enum StringParam {
    /// The mana cost of the cards. This uses the official text version of mana costs set forth in the
    /// [Comprehensive Rules](http://magic.wizards.com/en/game-info/gameplay/rules-and-formats/rules)
    ManaCost(String),
    /// Search for any supertype, card type, or subtype. Using only partial words is allowed.
    Type(String),
    /// Keywords to find cards that have specific phrases in their text box
    /// `~` Can be used as a placeholder for the card's name.
    ///
    /// **Note:** This keyword usually checks the current Oracle text for cards, so it uses the
    /// most up-to-date phrasing available. For example, “dies” instead of “is put into a
    /// graveyard”.
    Oracle(String),
    /// Search full Oracle text only, which includes reminder text
    OracleFull(String),
    /// The power of the cards. The parameter can be a number, a `*`, an `X`, etc.
    ///
    /// It can also be `tou`/`toughness` to search, for example, for creatures with more
    /// power then toughness: `StringParam::Power("tow", ComparisonExpr::AtLeast)`
    Power(ComparisonExpr, String),
    /// The toughness of the cards. The parameter can be a number, a `*`, an `X`, etc.
    ///
    /// It can also be `pow`/`power` to search, for example, for creatures with more
    /// toughness then power: `StringParam::Toughness("pow", ComparisonExpr::AtLeast)`
    Toughness(ComparisonExpr, String),
    /// The starting loyalty of the card. The parameter can be a number, a `*`, an `X`, etc.
    Loyalty(ComparisonExpr, String),
    /// Which set the cards are from using their three or four-letter Magic set code.
    Set(SetCode),
    /// Which block the cards are from using any of the codes of the sets that make up the
    /// block.
    Block(SetCode),
    /// Find cards that once “passed through” the given set code.
    WasInSet(SetCode),
    /// Find cards that are part of cube lists. For the supported values see
    /// the scryfall [docs](https://scryfall.com/docs/syntax#cubes).
    InCube(String),
    /// Find cards illustrated by a certain artist.
    Artist(String),
    /// Search for words in a card's flavor text.
    Flavor(String),
    /// Search for a card's affiliation watermark.
    WaterMark(String),
    /// Find cards in certain languages.
    Lang(String),
    /// Find cards in any language.
    LangAny,
    /// Find cards that were printed in a certain language.
    PrintedInLang(String),
}

impl Param for StringParam {
    fn to_param(&self) -> String {
        use StringParam::*;
        match self {
            ManaCost(s) => format!("m:{}", s),
            Type(s) => format!("t:{}", s),
            Oracle(s) => format!("o:\"{}\"", s),
            OracleFull(s) => format!("fo:\"{}\"", s),
            Power(c, s) => format!("pow{}{}", c, s),
            Toughness(c, s) => format!("tou{}{}", c, s),
            Loyalty(c, s) => format!("loy{}{}", c, s),
            Set(s) => format!("s:{}", s),
            Block(s) => format!("b:{}", s),
            WasInSet(s) => format!("in:{}", s),
            InCube(s) => format!("cube:{}", s),
            Artist(s) => format!("a:{}", s),
            Flavor(s) => format!("ft:{}", s),
            WaterMark(s) => format!("wt:{}", s),
            Lang(s) => format!("lang:{}", s),
            LangAny => "lang:any".to_string(),
            PrintedInLang(s) => format!("in:{}", s),
        }
    }
}

/// A parameter that takes a number as its value.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum NumericParam {
    /// Find cards of a specific converted mana cost
    CMC(ComparisonExpr, usize),
    /// Find cards by collector number within a set. Combine this with [`StringParam::Set`] to find
    /// specific card editions.
    ///
    /// [`StringParam::Set`]: enum.StringParam.html#variant.Set
    CollectorNumber(usize),
    /// Find cards by price in tix.
    TixPrice(ComparisonExpr, usize),
    /// Find cards by price in euros.
    EurPrice(ComparisonExpr, usize),
    /// Find cards by price in usd.
    UsdPrice(ComparisonExpr, usize),
    /// Find cards by the number of times a card has been printed.
    Prints(ComparisonExpr, usize),
    /// Find by number of sets a card has been in.
    Sets(ComparisonExpr, usize),
    /// Find cards by the number of times a card has been printed in paper.
    PaperPrints(ComparisonExpr, usize),
    /// Find by number of paper sets a card has been in.
    PaperSets(ComparisonExpr, usize),
}

impl Param for NumericParam {
    fn to_param(&self) -> String {
        use NumericParam::*;
        match self {
            CMC(c, p) => format!("cmc{}{}", c, p),
            CollectorNumber(n) => format!("cn:{}", n),
            TixPrice(c, n) => format!("tix{}{}", c, n),
            EurPrice(c, n) => format!("eur{}{}", c, n),
            UsdPrice(c, n) => format!("usd{}{}", c, n),
            Prints(c, n) => format!("prints{}{}", c, n),
            Sets(c, n) => format!("sets{}{}", c, n),
            PaperPrints(c, n) => format!("paperprints{}{}", c, n),
            PaperSets(c, n) => format!("papersets{}{}", c, n),
        }
    }
}

/// Find cards by their print rarity.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct RarityParam(
    /// How to filter for the rarity
    pub ComparisonExpr,
    /// The rarity to search for
    pub Rarity,
);

impl Param for RarityParam {
    fn to_param(&self) -> String {
        format!(
            "r{}{}",
            self.0,
            match self.1 {
                Rarity::Common => "c",
                Rarity::Uncommon => "u",
                Rarity::Rare => "r",
                Rarity::Mythic => "m",
            }
        )
    }
}

/// A parameter that takes a colour as its value.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ColorParam {
    /// Find cards that are a certain color.
    Color(ComparisonExpr, Colors),
    /// Find cards by their color identity.
    ColorIdentity(ComparisonExpr, Colors),
}

impl Param for ColorParam {
    fn to_param(&self) -> String {
        use ColorParam::*;
        match self {
            Color(ce, cl) => format!("c{}{}", cl, ce),
            ColorIdentity(ce, cl) => format!("id{}{}", cl, ce),
        }
    }
}

/// A parameter that takes a format as its value.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum FormatParam {
    /// Find cards legal in a format.
    Legal(Format),
    /// Find cards banned in a format.
    Banned(Format),
    /// Find cards restricted in a format.
    Restricted(Format),
}

impl Param for FormatParam {
    fn to_param(&self) -> String {
        use FormatParam::*;
        match self {
            Legal(f) => format!("legal:{}", f),
            Banned(f) => format!("banned:{}", f),
            Restricted(f) => format!("restricted:{}", f),
        }
    }
}

impl Param for BorderColor {
    fn to_param(&self) -> String {
        format!("border:{}", self)
    }
}

impl Param for Frame {
    fn to_param(&self) -> String {
        format!("frame:{}", self)
    }
}

impl Param for FrameEffect {
    fn to_param(&self) -> String {
        format!("frame:{}", self)
    }
}

/// A parameter that takes a game mode as its value.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameParam {
    /// Find specific prints available in different Magic game environments
    Game(Game),
    /// Filter by a card’s availability in a game
    InGame(Game),
}

impl Param for GameParam {
    fn to_param(&self) -> String {
        use GameParam::*;
        match self {
            Game(s) => format!("game:{}", s),
            InGame(s) => format!("in:{}", s),
        }
    }
}

/// A parameter that takes a time string as its value.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TimeParam {
    /// Find cards that were released relative to a certain year.
    Year(ComparisonExpr, usize),
    /// Find cards that were released relative to a certain date.
    Date(ComparisonExpr, chrono::NaiveDate),
    /// Find cards that were released relative to a certain set.
    Set(ComparisonExpr, SetCode),
}

impl Param for TimeParam {
    fn to_param(&self) -> String {
        use TimeParam::*;
        match self {
            Year(c, y) => format!("year{}{}", c, y),
            Date(c, d) => format!("date{}{}", c, d),
            Set(c, s) => format!("date{}{}", c, s),
        }
    }
}

/// The negative version of a param, for example, "is:spell" becomes "-is:spell"
///
/// ```rust
/// use scryfall::card_searcher::{BooleanParam, not, Param};
///
/// assert_eq!(not(BooleanParam::IsSpell).to_param(), "-is:spell")
/// ```
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NotParam<T: Param>(T);

/// Negates a parameter. See [`NotParam`] for the full documentation.
///
/// [`NotParam`]: struct.NotParam.html
pub fn not<T: Param>(t: T) -> NotParam<T> {
    NotParam(t)
}

impl<T: Param> Param for NotParam<T> {
    fn to_param(&self) -> String {
        format!("-{}", self.0.to_param())
    }
}
