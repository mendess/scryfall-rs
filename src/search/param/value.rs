//! TODO(msmorgan): Module docs.

use std::fmt;

pub use self::functions::*;
use crate::search::param::compare::{compare_op_str, Compare, CompareOp};
use crate::search::param::Param;

/// The type of parameter that this is. Corresponds to the name before the ':'
/// or other operator.
///
/// Refer to [the syntax documentation](https://scryfall.com/docs/syntax) for details on the
/// available parameter types.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ValueKind(ValueKindImpl);

impl ValueKind {
    pub(super) fn fmt_value(&self, value: &str, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self, value)
    }

    pub(super) fn fmt_comparison(
        &self,
        op: CompareOp,
        value: &str,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "{}{}{}", self, compare_op_str(Some(op)), value)
    }
}

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
            }
        )
    }
}

/// The base trait for a parameter value. The `into_param` method handles
/// converting the type into a [`Param`].
pub trait ParamValue: fmt::Debug + fmt::Display {
    /// Convert this value into a [`Param`] with the specified `kind`.
    fn into_param(self, kind: ValueKind) -> Param
    where
        Self: Sized,
    {
        Param::value(kind, self)
    }
}

/// A numeric value for a parameter.
///
/// Searchable parameters which directly use a `NumericValue` argument include
/// [`color_count()`] and [`collector_number()`]. Other parameters, such as
/// [`power()`] and [`toughness()`], can be directly compared against one
/// another. See [`NumericComparableValue`] for more information.
///
/// This trait is implemented for all numeric primitive types.
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

/// A numeric value for a parameter, supporting [comparison
/// operators][super::compare].
///
/// Parameters with a `NumericComparableValue` include [`power()`],
/// [`toughness()`], and [`cmc()`].
///
/// Parameters that use this trait can be compared to one another through
/// the [`NumProperty`] enum. For example, to search for a card with power
/// greater than its toughness:
///
/// ```rust,no_run
/// use scryfall::search::prelude::*;
/// let query = power(gt(NumProperty::Toughness));
/// ```
///
/// This trait is implemented by all `NumericValue` types and the `NumProperty`
/// enum.
pub trait NumericComparableValue: ParamValue {}

impl<T: NumericComparableValue> NumericComparableValue for Compare<T> {}

impl ParamValue for NumProperty {
    fn into_param(self, kind: ValueKind) -> Param {
        numeric_property_str(self).into_param(kind)
    }
}
impl NumericComparableValue for NumProperty {}

/// A string value for a parameter. Does not support comparison
/// operations.
///
/// Searchable parameters that directly use a `TextValue` argument include
/// [`watermark()`] and [`keyword()`]. Additionally, many types can
pub trait TextValue: ParamValue {}

/// Helper struct for a quoted value. The `Display` impl for this struct
/// surrounds the value in quotes. Representations that already contain
/// quotes are not supported.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Quoted<T>(T);

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

/// A string or regular expression value for a parameter. Does not support
/// comparison operators.
///
/// Some searchable properties can be searched with regular expressions.
/// These properties are [`name()`], [`type_line()`], [`oracle_text()`],
/// and [`flavor_text()`]. To specify a regular expression, use the
/// [`Regex`] type from this module.
///
/// This trait is implemented for all `TextValue` types and `Regex`.
///
/// For more information on supported regular expressions, see the
/// [official help page](https://scryfall.com/docs/regular-expressions).
pub trait TextOrRegexValue: ParamValue {}

impl<T: TextValue> TextOrRegexValue for T {}

/// `Regex` is a newtype for String, indicating that the string represents a
/// regular expression and should be surrounded by slashes in the search
/// query.
///
/// For more information on supported regular expressions, see the
/// [official help page](https://scryfall.com/docs/regular-expressions).
///
/// # Example
///
/// ```rust
/// # use scryfall::search::prelude::*;
/// # fn main() -> scryfall::Result<()> {
/// let cards_named_fog = name(r#"^fog$"#).search_all()?;
/// assert_eq!(cards_named_fog.len(), 1);
/// assert_eq!(cards_named_fog[0].name, "Fog");
/// # Ok(())
/// # }
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Regex(pub String);

impl fmt::Display for Regex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}/", self.0.replace('/', "\\/"))
    }
}

impl ParamValue for Regex {}
impl TextOrRegexValue for Regex {}

/// A color value represents one or more colors, or colorless/multicolored.
/// Supports [comparison operators][super::compare].
///
/// `ColorValue` is the argument type for the functions [`color()`] and
/// [`color_identity()`].
///
/// This type is implemented for [`Color`][crate::card::Color],
/// [`Colors`][crate::card::Colors],
/// [`Multicolored`][crate::card:: Multicolored],
/// and all [`TextValue`] types.
pub trait ColorValue: ParamValue {}

impl<T: ColorValue> ColorValue for Compare<T> {}

impl ParamValue for crate::card::Color {}
impl ColorValue for crate::card::Color {}

impl ParamValue for crate::card::Colors {}
impl ColorValue for crate::card::Colors {}

impl ParamValue for crate::card::Multicolored {}
impl ColorValue for crate::card::Multicolored {}

impl<T: TextValue> ColorValue for T {}

/// A value representing an amount of devotion to one or two colors. Supports
/// [comparison operations][super::compare].
///
/// The only parameter that takes a `DevotionValue` is [`devotion()`].
///
/// This trait is implemented by the [`Devotion`] type from this module.
///
/// # Example
/// ```rust
/// # use scryfall::search::prelude::*;
/// # fn main() -> scryfall::Result<()> {
/// use scryfall::card::Color;
/// let five_red_devotion = devotion(Devotion::monocolor(Color::Red, 5)).random()?;
/// assert!(five_red_devotion.cmc >= 5.0);
/// # Ok(())
/// # }
/// ```
pub trait DevotionValue: ParamValue {}

/// A representation of a permanent's devotion to one or two colors. Use the
/// constructors [`monocolor`][Devotion::monocolor()] and
/// [`hybrid`][Devotion::hybrid()] to create values of this type.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Devotion(crate::card::Color, Option<crate::card::Color>, usize);

impl fmt::Display for Devotion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let count = self.2;
        if count == 0 {
            // This is invalid syntax, but prevents false positives. The query "devotion:"
            // returns cards with a name containing "devotion".
            write!(f, "0")
        } else {
            let color_a = self.0;
            for _ in 0..=count {
                match self.1 {
                    Some(color_b) if color_b != color_a => {
                        write!(f, "{{{}/{}}}", color_a, color_b)
                    },
                    _ => write!(f, "{{{}}}", color_a),
                }?;
            }
            Ok(())
        }
    }
}

impl ParamValue for Devotion {}
impl DevotionValue for Devotion {}

impl DevotionValue for Compare<Devotion> {}

impl Devotion {
    /// Constructs a `Devotion` object with the given color and devotion count.
    pub fn monocolor(color: crate::card::Color, count: usize) -> Self {
        Devotion(color, None, count)
    }

    /// Constructs a `Devotion` object representing devotion to two colors with
    /// the given count.
    pub fn hybrid(color_a: crate::card::Color, color_b: crate::card::Color, count: usize) -> Self {
        Devotion(color_a, Some(color_b), count)
    }
}

/// A value representing the rarity of a printing. Supports [comparison
/// operators][super::compare].
///
/// Parameter functions with a `RarityValue` argument include [`rarity()`]
/// and [`in_rarity()`].
///
/// This trait is implemented for `String`, `&str`, and the
/// [`Rarity`][crate::card::Rarity] enum.
///
/// # Example
///
/// ```rust
/// # use scryfall::search::prelude::*;
/// use scryfall::card::Rarity;
/// # fn main() -> scryfall::Result<()> {
/// // Get the most expensive Common card, in USD.
/// let card = SearchOptions::new()
///     .query(rarity(Rarity::Common).and(cheapest("usd")))
///     .sort(SortOrder::Usd, SortDirection::Descending)
///     .unique(UniqueStrategy::Cards)
///     .search()?
///     .next()
///     .unwrap()?;
///
/// assert!(card.prices.usd.is_some());
/// # Ok(())
/// # }
/// ```
pub trait RarityValue: ParamValue {}

impl<T: TextValue> RarityValue for T {}

impl ParamValue for crate::card::Rarity {}
impl RarityValue for crate::card::Rarity {}

impl RarityValue for Compare<crate::card::Rarity> {}
impl<T: TextValue> RarityValue for Compare<T> {}

/// A value representing the name or code of the set a printing appears in.
///
/// Parameters with a `SetValue` argument include [`set()`] and [`in_set()`].
///
/// This trait is implemented for `String`, `&str`, and
/// [`SetCode`][crate::set::SetCode].
///
/// # Example
///
/// ```rust
/// # use scryfall::search::prelude::*;
/// # fn main() -> scryfall::Result<()> {
/// // Get a random Abzan card from Khans of Tarkir.
/// let card = set("ktk").and(name("abzan")).random()?;
/// assert!(card.name.to_lowercase().contains("abzan"));
/// # Ok(())
/// # }
/// ```
pub trait SetValue: ParamValue {}

impl<T: TextValue> SetValue for T {}

impl ParamValue for crate::set::SetCode {}
impl SetValue for crate::set::SetCode {}

/// A value representing a draft cube from MTGO, such as the
/// [Vintage Cube](https://scryfall.com/cubes/vintage).
///
/// `CubeValue` is used as the value type for [`cube()`].
///
/// This trait is implemented for `String` and `&str`.
pub trait CubeValue: ParamValue {}

impl<T: TextValue> CubeValue for T {}

/// A value representing a constructed format, such as Standard or Commander.
///
/// Parameters with a `FormatValue` argument include [`format()`], [`banned()`],
/// and [`restricted()`].
///
/// This trait is implemented for `String` and `&str`, as well as the
/// [`Format`][crate::format::Format] enum.
///
/// ```rust
/// # use scryfall::search::prelude::*;
/// # fn main() -> scryfall::Result<()> {
/// use scryfall::format::Format;
/// // Find a card that's restricted in Vintage whose name contains 'recall'.
/// let card = restricted(Format::Vintage)
///     .and(name("recall"))
///     .search_all()?
///     .into_iter()
///     .next()
///     .unwrap();
/// assert_eq!(card.name, "Ancestral Recall");
/// # Ok(())
/// # }
/// ```
pub trait FormatValue: ParamValue {}

impl<T: TextValue> FormatValue for T {}

impl ParamValue for crate::format::Format {}
impl FormatValue for crate::format::Format {}

/// A value representing a currency which has prices available on Scryfall.
///
/// `CurrencyValue` is used as an argument for the [`cheapest`] parameter.
///
/// This trait is implemented for `String` and `&str`.
pub trait CurrencyValue: ParamValue {}

impl<T: TextValue> CurrencyValue for T {}

/// A value representing a type of Magic set, such as a core set or a duel deck.
///
/// `SetTypeValue` is used as the argument type for [`set_type()`] and
/// [`in_set_type()`].
///
/// This trait is implemented for the [`SetType`][crate::set::SetType] enum
/// and all [`TextValue`] types.
pub trait SetTypeValue: ParamValue {}

impl ParamValue for crate::set::SetType {}
impl SetTypeValue for crate::set::SetType {}

impl<T: TextValue> SetTypeValue for T {}

/// A value representing a border color, such as black, white, or silver.
///
/// `BorderColorValue` is used as the argument type for [`border_color()`].
///
/// This trait is implemented for the [`BorderColor`][crate::card::BorderColor]
/// and all [`TextValue`] types.
pub trait BorderColorValue: ParamValue {}

impl<T: TextValue> BorderColorValue for T {}

impl ParamValue for crate::card::BorderColor {}
impl BorderColorValue for crate::card::BorderColor {}

/// A value representing card frames and frame effects.
///
/// `FrameValue` is the argument type for [`frame()`] and [`frame_effect()`].
///
/// This trait is implemented for the enums [`Frame`][crate::card::Frame]
/// and [`FrameEffect`][crate::card::FrameEffect], as well as all [`TextValue`]
/// types.
pub trait FrameValue: ParamValue {}

impl<T: TextValue> FrameValue for T {}

impl ParamValue for crate::card::FrameEffect {}
impl FrameValue for crate::card::FrameEffect {}

impl ParamValue for crate::card::Frame {}
impl FrameValue for crate::card::Frame {}

/// A parameter that represents a date. A set code can also be used used to
/// stand for the date that set was released. Supports
/// [comparison operators][super::compare].
///
/// `DateValue` is the argument type for [`date()`].
///
/// This trait is implemented for [`chrono::NaiveDate`],
/// [`SetCode`][crate::set::SetCode], and any [`TextValue`] such as `String` or
/// `&str`. When searching with a string, it must either be a valid set code or
/// a date in the format `yyyy[-mm[-dd]]`.
pub trait DateValue: ParamValue {}

impl<T: DateValue> DateValue for Compare<T> {}

impl<T: SetValue> DateValue for T {}

impl ParamValue for chrono::NaiveDate {
    fn into_param(self, kind: ValueKind) -> Param
    where
        Self: Sized,
    {
        Param::value(kind, self.format("%Y-%m-%d").to_string())
    }
}
impl DateValue for chrono::NaiveDate {}

/// A parameter that specifies a game that the card appears in.
///
/// `GameValue` is the argument type for [`game()`] and [`in_game()`].
///
/// This trait is implemented for the [`Game`][crate::card::Game] enum, and for
/// all [`TextValue`] types, such as `String` and `&str`.
pub trait GameValue: ParamValue {}

impl<T: TextValue> GameValue for T {}

impl ParamValue for crate::card::Game {}
impl GameValue for crate::card::Game {}

/// A parameter that represents a written language that a card
/// is printed in. For a full list of supported languages,
/// refer to the [official docs](https://scryfall.com/docs/api/languages).
///
/// `LanguageValue` is used as an argument to [`language()`] and
/// [`in_language()`].
///
/// This trait is implemented for all `TextValue` types.
pub trait LanguageValue: ParamValue {}

impl<T: TextValue> LanguageValue for T {}

mod functions {
    use super::*;
    use crate::search::query::Query;

    macro_rules! value_fns {
        ($(
            $(#[$($attr:meta)*])*
            $func:ident => $Kind:ident : $Constraint:ident,
        )*) => {
            $(
                $(#[$($attr)*])*
                pub fn $func(value: impl $Constraint) -> Query {
                    Query::Param(value.into_param(ValueKind(ValueKindImpl::$Kind)))
                }
            )*
        };
    }

    value_fns! {
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
        #[doc = "The devotion granted by this permanent. See [`Devotion`]."]
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
        collector_number => Number: NumericValue,
        #[doc = "The block of this card. Works with any set grouped in the same block."]
        block => Block: SetValue,
        #[doc = "The type of set this printing is in."]
        set_type => SetType: SetTypeValue,
        #[doc = "Has the card appeared in a set of this type?"]
        in_set_type => InSetType: SetTypeValue,
        #[doc = "Does the card appear in this cube on MTGO?"]
        cube => Cube: CubeValue,
        #[doc(hidden)]
        format => Format: FormatValue,
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
    }

    macro_rules! numeric_value_fns {
        ($(
            $(#[$($attr:meta)*])*
            $func:ident => $NumProp:ident,
        )*) => {
            $(
                $(#[$($attr)*])*
                pub fn $func(value: impl NumericComparableValue) -> Query {
                    Query::Param(value.into_param(ValueKind(
                        ValueKindImpl::NumericComparable(NumProperty::$NumProp),
                    )))
                }
            )*
        };
    }

    numeric_value_fns! {
        #[doc = "The card's power, if it is a creature or vehicle. '*' and 'X' count as 0."]
        power => Power,
        #[doc = "The card's toughness, if it is a creature or vehicle. '*' and 'X' count as 0."]
        toughness => Toughness,
        #[doc = "The card's power plus its toughness."]
        pow_tou => PowTou,
        #[doc = "The card's loyalty, if it is a planeswalker. 'X' counts as 0."]
        loyalty => Loyalty,
        #[doc = "The converted mana cost of this card."]
        cmc => Cmc,
        #[doc = "The number of artists credited for this printing."]
        artist_count => ArtistCount,
        #[doc = "The current market price of this card in US Dollars."]
        usd => Usd,
        #[doc = "The current foil market price of this card in US Dollars."]
        usd_foil => UsdFoil,
        #[doc = "The current market price of this card in Euros."]
        eur => Eur,
        #[doc = "The current market price of this card in MTGO tickets."]
        tix => Tix,
        #[doc = "The number of unique art this card has had."]
        illustration_count => IllustrationCount,
        #[doc = "The number of unique prints of this card."]
        print_count => PrintCount,
        #[doc = "The number of sets this card has appeared in."]
        set_count => SetCount,
        #[doc = "The number of unique prints of this card, counting paper only."]
        paper_print_count => PaperPrintCount,
        #[doc = "The number of sets this card has appeared in, counting paper only."]
        paper_set_count => PaperSetCount,
        #[doc = "The year this card was released."]
        year => Year,
    }
}
