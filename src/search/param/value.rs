//! TODO(msmorgan): Module docs.

use std::fmt;

use crate::search::param::compare::{compare_op_str, Compare, CompareOp};
use crate::search::param::Param;
use crate::search::query::Query;

/// The type of parameter that this is. Corresponds to the name before the ':'
/// or other operator.
///
/// Refer to [the syntax documentation](https://scryfall.com/docs/syntax) for details on the
/// available parameter types.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ValueKind(ValueKindImpl);

impl ValueKind {
    pub(super) fn fmt_value(
        &self,
        op: Option<CompareOp>,
        value: &dyn fmt::Display,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        if let (ValueKindImpl::Exact, None) = (&self.0, op) {
            write!(f, "!{}", value)
        } else {
            write!(f, "{}{}{}", self, compare_op_str(op), value)
        }
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

/// The base trait for a parameter value. The `into_param` function handles
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

/// Color parameters allow querying by specific colors.
pub trait ColorValue: ParamValue {}

impl<T: 'static + ColorValue> ColorValue for Compare<T> {}

impl ParamValue for crate::card::Color {}
impl ColorValue for crate::card::Color {}

impl ParamValue for crate::card::Colors {}
impl ColorValue for crate::card::Colors {}

impl ParamValue for crate::card::Multicolored {}
impl ColorValue for crate::card::Multicolored {}

// TODO(msmorgan): Should text be a valid ColorValue?

/// Devotion works differently than other color parameters. All the color
/// symbols must match and the symbols can be hybrid mana.
pub trait DevotionValue: ParamValue {}

// TODO(msmorgan): Support hybrid mana devotion. `Colors` will not work, since
//   the syntax is different for hybrid mana. Maybe a new `ManaSymbol` type?
/// TODO(msmorgan): Docs.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Devotion(crate::card::Color, usize);

impl fmt::Display for Devotion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.1 == 0 {
            // This is invalid syntax, but prevents false positives. The query "devotion:"
            // returns cards with a name containing "devotion".
            write!(f, "0")
        } else {
            for _ in 0..=self.1 {
                write!(f, "{{{}}}", self.0)?;
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
    pub fn new(color: crate::card::Color, count: usize) -> Self {
        Devotion(color, count)
    }
}

/// A numeric value for a parameter.
///
/// TODO(msmorgan): More.
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

/// TODO(msmorgan): Docs.
pub trait NumericComparableValue: ParamValue {}

impl<T: 'static + NumericComparableValue> NumericComparableValue for Compare<T> {}

impl ParamValue for NumProperty {
    fn into_param(self, kind: ValueKind) -> Param {
        numeric_property_str(self).into_param(kind)
    }
}
impl NumericComparableValue for NumProperty {}

/// This is the
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

/// TODO(msmorgan): Docs.
pub trait TextOrRegexValue: ParamValue {}

impl<T: TextValue> TextOrRegexValue for T {}

/// `Regex` is a newtype for String, indicating that the string represents a
/// regular expression and should be surrounded by slashes instead of quotes.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Regex(pub String);

impl fmt::Display for Regex {
    // TODO(msmorgan): Escapes.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}/", self.0)
    }
}

impl ParamValue for Regex {}
impl TextOrRegexValue for Regex {}

/// A value representing the rarity of a printing. Supports [comparison
/// operators][super::compare].
///
/// Parameters with a `RarityValue` argument include [`rarity()`] and
/// [`in_rarity()`].
///
/// This trait is implemented for `String`, `&str`, and the
/// [`Rarity`][crate::card::Rarity] enum, and supports comparison operators.
pub trait RarityValue: ParamValue {}

impl<T: TextValue> RarityValue for T {}

impl ParamValue for crate::card::Rarity {}
impl RarityValue for crate::card::Rarity {}

impl RarityValue for Compare<crate::card::Rarity> {}
impl<T: 'static + TextValue> RarityValue for Compare<T> {}

/// A value representing the name or code of the set a printing appears in.
///
/// Parameters with a `SetValue` argument include [`set()`] and [`in_set()`].
///
/// This trait is implemented for `String`, `&str`, and
/// [`SetCode`][crate::set::SetCode].
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

/// A value representing a type of Magic set, such as
/// TODO(msmorgan): More.
pub trait SetTypeValue: ParamValue {}

impl<T: TextValue> SetTypeValue for T {}

/// TODO(msmorgan): Docs.
pub trait BorderColorValue: ParamValue {}

impl<T: TextValue> BorderColorValue for T {}

impl ParamValue for crate::card::BorderColor {}

impl BorderColorValue for crate::card::BorderColor {}

/// A value representing card frames and frame effects.
///
/// TODO(msmorgan): More.
pub trait FrameValue: ParamValue {}

impl<T: TextValue> FrameValue for T {}

impl ParamValue for crate::card::FrameEffect {}
impl FrameValue for crate::card::FrameEffect {}

impl ParamValue for crate::card::Frame {}
impl FrameValue for crate::card::Frame {}

/// A parameter that represents a date, in `yyyy[-mm[-dd]]` format. A set code
/// can also be used used to stand in for the date that set was released.
/// Supports [comparison operators][super::compare].
///
/// TODO(msmorgan): More.
pub trait DateValue: ParamValue {}

impl<T: 'static + DateValue> DateValue for Compare<T> {}

impl<T: SetValue> DateValue for T {}

impl ParamValue for chrono::NaiveDate {
    fn into_param(self, kind: ValueKind) -> Param
    where
        Self: 'static + Sized,
    {
        Param::value(kind, self.format("%Y-%m-%d").to_string())
    }
}
impl DateValue for chrono::NaiveDate {}

/// A parameter that specifies a game that the card appears in.
/// Valid for any `TextValue` and for [`Game`][crate::card::Game].
///
/// TODO(msmorgan): Docs.
pub trait GameValue: ParamValue {}

impl<T: TextValue> GameValue for T {}

impl ParamValue for crate::card::Game {}
impl GameValue for crate::card::Game {}

/// TODO(msmorgan): Docs.
pub trait LanguageValue: ParamValue {}

impl<T: TextValue> LanguageValue for T {}

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
