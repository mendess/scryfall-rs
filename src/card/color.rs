use std::fmt;

use serde::{Deserialize, Serialize};

use self::Color::*;

/// Enum defining the 5 colors of magic, plus colorless.
#[derive(
    Default, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum Color {
    #[default]
    #[serde(rename = "C")]
    Colorless = 0,
    #[serde(rename = "W")]
    White = 1 << 0,
    #[serde(rename = "U")]
    Blue = 1 << 1,
    #[serde(rename = "B")]
    Black = 1 << 2,
    #[serde(rename = "R")]
    Red = 1 << 3,
    #[serde(rename = "G")]
    Green = 1 << 4,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Colorless => "C",
                Color::White => "W",
                Color::Blue => "U",
                Color::Black => "B",
                Color::Red => "R",
                Color::Green => "G",
            }
        )
    }
}

/// Definition of a cards colors. This can be used in conjunction with
/// the `search` module as a
/// [`ColorValue`][crate::search::param::value::ColorValue].
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct Colors(u8);

macro_rules! color_consts {
    ($(
        $(#[$($attr:meta)*])*
        $($name:ident),+ => [$($color:tt)*];
    )*) => {
        $(
            color_consts!(@inner ($(($($attr)*))*) ($($name),+) [$($color)*]);
        )*
    };

    (
        @inner
        $attrs:tt ($($name:ident),+) $colors:tt
    ) => {
        $(
            color_consts!(@inner $attrs $name $colors);
        )+
    };

    (
        @inner
        ($(($($attr:meta)*))*)
        $name:ident [$($color:ident),*]
    ) => {
        $(#[$($attr)*])*
        pub const $name: Self = Colors::from_slice(&[$($color),*]);
    };
}

impl Colors {
    color_consts! {
        #[doc = "Colorless."]
        #[doc(alias = "c")]
        COLORLESS => [];

        #[doc = "White."]
        #[doc(alias = "w")]
        WHITE => [White];
        #[doc = "Blue."]
        #[doc(alias = "u")]
        BLUE => [Blue];
        #[doc = "Black."]
        #[doc(alias = "b")]
        BLACK => [Black];
        #[doc = "Red."]
        #[doc(alias = "r")]
        RED => [Red];
        #[doc = "Green."]
        #[doc(alias = "g")]
        GREEN => [Green];

        #[doc = "White and blue. The colors of the Azorius Senate from Ravnica."]
        #[doc(alias = "uw")]
        #[doc(alias = "wu")]
        AZORIUS => [White, Blue];
        #[doc = "Blue and black. The colors of House Dimir from Ravnica."]
        #[doc(alias = "ub")]
        DIMIR => [Blue, Black];
        #[doc = "Black and red. The colors of the Cult of Rakdos from Ravnica."]
        #[doc(alias = "br")]
        RAKDOS => [Black, Red];
        #[doc = "Red and green. The colors of the Gruul Clans from Ravnica."]
        #[doc(alias = "rg")]
        GRUUL => [Red, Green];
        #[doc = "Green and white. The colors of the Selesnya Conclave from Ravnica."]
        #[doc(alias = "gw")]
        SELESNYA => [Green, White];
        #[doc = "White and black. The colors of the Orzhov Syndicate from Ravnica."]
        #[doc(alias = "wb")]
        #[doc(alias = "bw")]
        ORZHOV => [White, Black];
        #[doc = "Blue and red. The colors of the Izzet League from Ravnica."]
        #[doc(alias = "ur")]
        IZZET => [Blue, Red];
        #[doc = "Black and green. The colors of the Golgari Swarm from Ravnica."]
        #[doc(alias = "bg")]
        GOLGARI => [Black, Green];
        #[doc = "Red and white. The colors of the Boros Legion from Ravnica."]
        #[doc(alias = "rw")]
        BOROS => [Red, White];
        #[doc = "Green and blue. The colors of the Simic Combine from Ravnica."]
        #[doc(alias = "gu")]
        #[doc(alias = "ug")]
        SIMIC => [Green, Blue];

        #[doc = "White, blue, and black. The colors of the Esper shard of Alara."]
        #[doc(alias = "wub")]
        ESPER => [White, Blue, Black];
        #[doc = "Blue, black, and red. The colors of the Grixis shard of Alara."]
        #[doc(alias = "ubr")]
        GRIXIS => [Blue, Black, Red];
        #[doc = "Black, red, and green. The colors of the Jund shard of Alara."]
        #[doc(alias = "brg")]
        JUND => [Black, Red, Green];
        #[doc = "Red, green, and white. The colors of the Naya shard of Alara."]
        #[doc(alias = "rgw")]
        NAYA => [Red, Green, White];
        #[doc = "Green, white, and blue. The colors of the Bant shard of Alara."]
        #[doc(alias = "gwu")]
        BANT => [Green, White, Blue];
        #[doc = "White, black, and green. The colors of the Abzan Houses from Tarkir."]
        #[doc(alias = "junk")]
        #[doc(alias = "bgw")]
        ABZAN => [White, Black, Green];
        #[doc = "Blue, red, and white. The colors of the Jeskai Way from Tarkir."]
        #[doc(alias = "american")]
        #[doc(alias = "ruw")]
        JESKAI => [Blue, Red, White];
        #[doc = "Black, green, and blue. The colors of the Sultai Brood from Tarkir."]
        #[doc(alias = "bug")]
        SULTAI => [Black, Green, Blue];
        #[doc = "Red, white, and black. The colors of the Mardu Horde from Tarkir."]
        #[doc(alias = "rbw")]
        MARDU => [Red, White, Black];
        #[doc = "Green, blue, and red. The colors of the Temur Frontier from Tarkir."]
        #[doc(alias = "rug")]
        TEMUR => [Green, Blue, Red];

        #[doc = "White, blue, black, and red. The colors of artifice and \
                 [Yore-Tiller Nephilim](https://scryfall.com/card/gpt/140)."]
        #[doc(alias = "wubr")]
        ARTIFICE => [White, Blue, Black, Red];
        #[doc = "Blue, black, red, and green. The colors of chaos and \
                 [Glint-Eye Nephilim](https://scryfall.com/card/gpt/115)."]
        #[doc(alias = "ubrg")]
        CHAOS => [Blue, Black, Red, Green];
        #[doc = "Black, red, green, and white. The colors of aggression and \
                 [Dune-Brood Nephilim](https://scryfall.com/card/gpt/110)."]
        #[doc(alias = "brgw")]
        AGGRESSION => [Black, Red, Green, White];
        #[doc = "Red, green, white, and blue. The colors of altruism and \
                 [Ink-treader Nephilim](https://scryfall.com/card/gpt/117)."]
        #[doc(alias = "rgwu")]
        ALTRUISM => [Red, Green, White, Blue];
        #[doc = "Green, white, blue, and black. The colors of growth and \
                 [Witch-Maw Nephilim](https://scryfall.com/card/gpt/138)."]
        #[doc(alias = "gwub")]
        GROWTH => [Green, White, Blue, Black];

        #[doc = "White, blue, black, red, and green. All five colors."]
        #[doc(alias = "wubrg")]
        ALL => [White, Blue, Black, Red, Green];
    }

    /// Constructs an instance from a list of `colors`.
    pub const fn from_slice(colors: &[Color]) -> Self {
        let mut result = Colors::colorless();
        let mut i = 0;
        while i < colors.len() {
            result = result.with(colors[i]);
            i += 1;
        }
        result
    }

    /// Constructs an instance representing a single `color`.
    pub const fn monocolor(color: Color) -> Self {
        Colors(color as u8)
    }

    /// Creates an instance representing a colorless card.
    pub const fn colorless() -> Self {
        Colors(Colorless as u8)
    }

    /// Checks if this instance is a certain color.
    pub const fn is(self, color: Color) -> bool {
        self.0 & color as u8 != 0
    }

    /// Checks if this instance is multicolored, which is true if it contains
    /// more than one color flag.
    pub const fn is_multicolored(self) -> bool {
        self.0.count_ones() > 1
    }

    /// Checks if this instance is colorless.
    pub const fn is_colorless(self) -> bool {
        self.0 == Colorless as u8
    }

    /// Produces a new instance with all the colors from both `self` and
    /// `other`.
    pub const fn union(self, other: Colors) -> Self {
        Colors(self.0 | other.0)
    }

    /// Produces a new instance with the colors that `self` and `other` have in
    /// common.
    pub const fn intersection(self, other: Colors) -> Self {
        Colors(self.0 & other.0)
    }

    /// Produces a new instance with the colors from `other` removed.
    pub const fn difference(self, other: Colors) -> Self {
        Colors(self.0 & !other.0)
    }

    /// Produces a new instance with the colors that are in `self` or `other`,
    /// but not both.
    pub const fn symmetric_difference(self, other: Colors) -> Self {
        Colors((self.0 ^ other.0) & (self.0 | other.0))
    }

    /// Produces a new instance with the specified color added.
    pub const fn with(self, color: Color) -> Self {
        Colors(self.0 | color as u8)
    }

    /// Produces a new instance with the specified color removed.
    pub const fn without(self, color: Color) -> Self {
        Colors(self.0 & !(color as u8))
    }
}

impl std::fmt::Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_colorless() {
            write!(f, "c")
        } else {
            let mut s = String::with_capacity(5);
            if self.is(White) {
                s.push('w');
            }
            if self.is(Blue) {
                s.push('u');
            }
            if self.is(Black) {
                s.push('b');
            }
            if self.is(Red) {
                s.push('r');
            }
            if self.is(Green) {
                s.push('g');
            }
            write!(f, "{s}")
        }
    }
}

impl From<&[Color]> for Colors {
    fn from(colors: &[Color]) -> Self {
        Colors::from_slice(colors)
    }
}

impl From<Color> for Colors {
    fn from(color: Color) -> Self {
        Colors::monocolor(color)
    }
}

/// Multicolored card. This can be used as a
/// [`ColorValue`][crate::search::param::value::ColorValue] for searching
/// Scryfall.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Multicolored;

impl fmt::Display for Multicolored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "m")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union() {
        assert_eq!(Colors::RED.union(Colors::WHITE), Colors::BOROS);
        assert_eq!(Colors::BLACK.union(Colorless.into()), Colors::BLACK);
        assert_eq!(Colors::GOLGARI.union(Colors::WHITE), Colors::ABZAN);
        assert_eq!(Colors::ALL.union(Colors::GREEN), Colors::ALL);
    }

    #[test]
    fn intersection() {
        assert_eq!(
            Colors::ORZHOV.intersection(Colors::IZZET),
            Colors::COLORLESS
        );
        assert_eq!(Colors::NAYA.intersection(Colors::ESPER), Colors::WHITE);
        assert_eq!(Colors::ALL.intersection(Colors::GRUUL), Colors::GRUUL);
    }

    #[test]
    fn difference() {
        assert_eq!(Colors::SELESNYA.difference(Colors::ALL), Colors::COLORLESS);
        assert_eq!(Colors::BANT.difference(Colors::RAKDOS), Colors::BANT);
        assert_eq!(Colors::CHAOS.difference(Colors::JESKAI), Colors::GOLGARI);
    }

    #[test]
    fn symmetric_difference() {
        assert_eq!(
            Colors::WHITE.symmetric_difference(Colors::BLACK),
            Colors::ORZHOV
        );
        assert_eq!(
            Colors::SIMIC.symmetric_difference(Colors::BLUE),
            Colors::GREEN
        );
        assert_eq!(
            Colors::GRIXIS.symmetric_difference(Colors::TEMUR),
            Colors::GOLGARI
        );
    }
}
