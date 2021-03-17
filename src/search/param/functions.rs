use crate::search::param::{criteria, value, Param};
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

macro_rules! criterion_fns {
    ($(
        $(#[$($attr:meta)*])*
        $func:ident => $Variant:ident($SubVariant:ident),
    )*) => {
        $(
            $(#[$($attr)*])*
            pub fn $func(positive: bool) -> Query {
                let criterion = criteria::Criterion::$Variant(criteria::$Variant::$SubVariant);
                let query = Query::Param(Param::criterion(criterion));
                if positive {
                    query
                } else {
                    Query::Not(query.into())
                }
            }
        )*
    };
}

criterion_fns! {
    #[doc = "Matches cards that have a color indicator."]
    has_color_indicator => Has(ColorIndicator),
    #[doc = "Matches printings that have a watermark."]
    has_watermark => Has(Watermark),

    #[doc = "Find cards that are printed for the first time in paper."]
    new_card => New(Card),
    #[doc = "Find cards printed at a new rarity for the first time (including first prints)."]
    new_rarity => New(Rarity),
    #[doc = "Find cards being printed with new illustrations (including first prints)."]
    new_art => New(Art),
    #[doc = "Find cards being illustrated by a particular artist for the first time \
             (including first prints)."]
    new_artist => New(Artist),
    #[doc = "Find cards being printed with brand-new flavor text using for the first time."]
    new_flavor => New(Flavor),
    #[doc = "Find cards printed in a specific frame for the first time."]
    new_frame => New(Frame),
    #[doc = "Find the first printing of a card in each language."]
    new_language => New(Language),

    #[doc = "You can filter cards that contain Phyrexian mana symbols."]
    is_phyrexian => Is(Phyrexian),
    #[doc = "You can filter cards that contain hybrid mana symbols."]
    is_hybrid => Is(Hybrid),
    #[doc = "Find split cards."]
    is_split => Is(Split),
    #[doc = "Find flip cards."]
    is_flip => Is(Flip),
    #[doc = "Find transforming cards."]
    is_transform => Is(Transform),
    #[doc = "Find cards with meld."]
    is_meld => Is(Meld),
    #[doc = "Find leveler cards."]
    is_leveler => Is(Leveler),
    #[doc = "Find cards that are cast as spells"]
    is_spell => Is(Spell),
    #[doc = "Find permanent cards."]
    is_permanent => Is(Permanent),
    #[doc = "Find historic cards."]
    is_historic => Is(Historic),
    #[doc = "Find party cards."]
    is_party => Is(Party),
    #[doc = "Find cards with modal effects."]
    is_modal => Is(Modal),
    #[doc = "Find vanilla creatures."]
    is_vanilla => Is(Vanilla),
    #[doc = "Find french vanilla creatures (evergreen keywords only)."]
    is_french_vanilla => Is(FrenchVanilla),
    #[doc = "Find Un-cards, holiday cards, and other funny cards."]
    is_funny => Is(Funny),
    #[doc = "Find cards that can be your commander."]
    is_commander => Is(Commander),
    #[doc = "Find cards that can be your Brawl commander."]
    is_brawler => Is(Brawler),
    #[doc = "Find cards that can be your companion."]
    is_companion => Is(Companion),
    #[doc = "Find cards on the reserved list."]
    is_reserved => Is(Reserved),
    #[doc = "Find cards with full art."]
    is_full_art => Is(Full),
    #[doc = "Find non-foil printings of cards."]
    is_nonfoil => Is(NonFoil),
    #[doc = "Find foil printings of cards."]
    is_foil => Is(Foil),
    #[doc = "Find cards in `scryfall`'s database with high-resolution images."]
    is_high_resolution => Is(HiRes),
    #[doc = "Find prints that are only available digitally (MTGO and Arena)"]
    is_digital => Is(Digital),
    #[doc = "Find promotional cards."]
    is_promo => Is(Promo),
    #[doc = "Find cards that are Story Spotlights."]
    is_story_spotlight => Is(Spotlight),
    #[doc = "Find cards that are in the Masterpiece Series."]
    is_masterpiece => Is(Masterpiece),
    #[doc = "Find cards that have only been in a single set."]
    is_unique => Is(Unique),
    #[doc = "Find first printings (digital or paper)."]
    is_first_print => Is(FirstPrint),
    #[doc = "Find reprints."]
    is_reprint => Is(Reprint),

    // TODO(msmorgan): These names are bad.
    #[doc = "Find cards that were sold in boosters."]
    sold_in_boosters => SoldIn(Booster),
    #[doc = "Find cards that were sold in planeswalker decks."]
    sold_in_planeswalker_decks => SoldIn(PlaneswalkerDeck),
    #[doc = "Find cards that were given away in leagues."]
    sold_in_league => SoldIn(League),
    #[doc = "Find cards that were included as buy a box promos."]
    sold_in_buy_a_box => SoldIn(BuyABox),
    #[doc = "Find cards that were included in gift boxes."]
    sold_in_gift_box => SoldIn(GiftBox),
    #[doc = "Find cards that were given away in intro packs."]
    sold_in_intro_pack => SoldIn(IntroPack),
    #[doc = "Find cards that were given away in game days."]
    sold_in_game_day => SoldIn(GameDay),
    #[doc = "Find cards that were given away in prereleases."]
    sold_in_prerelease => SoldIn(Prerelease),
    #[doc = "Find cards that were given away in releases."]
    sold_in_release => SoldIn(Release),

    #[doc = "A cycling dual land, such as [Fetid Pools](https://scryfall.com/card/akh/243)."]
    is_bicycle_land => LandFamily(Bicycle),
    #[doc = "A cycling tri land, such as [Ketria Triome](https://scryfall.com/card/iko/250)."]
    is_tricycle_land => LandFamily(Tricycle),
    #[doc = "A land that returns other lands to your hand, such as \
             [Boros Garrison](https://scryfall.com/card/rav/275)."]
    is_bounce_land => LandFamily(Bounce),
    #[doc = "A pain land that can be sacrificed to draw a card, such as \
             [Horizon Canopy](https://scryfall.com/card/fut/177)."]
    is_canopy_land => LandFamily(Canopy),
    #[doc = "A land that enters tapped unless you control a basic of its color, such \
             as [Glacial Fortress](https://scryfall.com/card/m10/226)."]
    is_check_land => LandFamily(Check),
    #[doc = "An original dual land, such as [Tropical Island](https://scryfall.com/card/lea/283)."]
    is_dual_land => LandFamily(Dual),
    #[doc = "A land that enters tapped unless you control two or fewer other lands, \
             such as [Blackcleave Cliffs](https://scryfall.com/card/som/224)."]
    is_fast_land => LandFamily(Fast),
    #[doc = "A fetch land, such as [Scalding Tarn](https://scryfall.com/card/zen/223)."]
    is_fetch_land => LandFamily(Fetch),
    #[doc = "A land that filters mana into other colors, such as \
             [Mystic Gate](https://scryfall.com/card/shm/277) or \
             [Cascading Cataracts](https://scryfall.com/card/akh/240/cascading-cataracts)."]
    is_filter_land => LandFamily(Filter),
    #[doc = "A land that enters tapped and gains 1 life, such as \
             [Jungle Hollow](https://scryfall.com/card/ktk/235)."]
    is_gain_land => LandFamily(Gain),
    #[doc = "A land that costs life for colored mana, such as \
             [Caves of Koilos](https://scryfall.com/card/apc/140)."]
    is_pain_land => LandFamily(Pain),
    #[doc = "A land that enters tapped and has \"Scry 1\", such as \
             [Temple of Mystery](https://scryfall.com/card/ths/226)."]
    is_scry_land => LandFamily(Scry),
    #[doc = "A land that enters tapped unless you reveal a basic from your hand, such \
             as [Choked Estuary](https://scryfall.com/card/soi/270)."]
    is_shadow_land => LandFamily(Shadow),
    #[doc = "A land that enters tapped unless you pay 2 life, such as\
             [Breeding Pool](https://scryfall.com/card/dis/172)."]
    is_shock_land => LandFamily(Shock),
    #[doc = "A land that allows you to store up mana for later use, such as \
             [Fungal Reaches](https://scryfall.com/card/tsp/273) or \
             [Crucible of the Spirit Dragon](https://scryfall.com/card/frf/167)."]
    is_storage_land => LandFamily(Storage),
    #[doc = "A land that turns into a creature, such as \
             [Celestial Colonnade](https://scryfall.com/card/wwk/133), \
             [Mutavault](https://scryfall.com/card/mor/148), or \
             [Inkmoth Nexus](https://scryfall.com/card/mbs/145)."]
    is_creature_land => LandFamily(Creature),
    #[doc = "A land that enters tapped and produces three colors, such as \
             [Mystic Monastery](https://scryfall.com/card/ktk/236)."]
    is_tri_land => LandFamily(Tri),
    #[doc = "A land that enters tapped unless you control two basics in its \
             colors, such as [Canopy Vista](https://scryfall.com/card/bfz/234)."]
    is_battle_land => LandFamily(Battle),

    #[doc = "The converted mana cost of this card is an even number."]
    even_cmc => Cmc(Even),
    #[doc = "The converted mana cost of this card is an odd number."]
    odd_cmc => Cmc(Odd),
}
