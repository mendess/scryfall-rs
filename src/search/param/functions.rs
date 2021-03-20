use crate::search::param::{value, Param};
use crate::search::query::Query;

/// Matches a card whose name is exactly `name`.
pub fn exact(name: impl Into<String>) -> Query {
    Query::Param(Param::exact(name))
}

macro_rules! value_fns {
    ($(
        $(#[$($attr:meta)*])*
        $func:ident => $Kind:ident : $Constraint:ident,
    )*) => {
        $(
            $(#[$($attr)*])*
            pub fn $func(value: impl value::$Constraint) -> Query {
                Query::Param(value.into_param(value::ValueKind(value::ValueKindImpl::$Kind)))
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
            pub fn $func(value: impl value::NumericComparableValue) -> Query {
                Query::Param(value.into_param(value::ValueKind(
                    value::ValueKindImpl::NumericComparable(value::NumProperty::$NumProp),
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
