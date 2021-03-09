use std::{fmt, ops};

use serde::{Deserialize, Serialize};

use self::Color::*;

/// Enum defining the 5 colors of magic, plus colorless.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[allow(missing_docs)]
#[repr(u8)]
pub enum Color {
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
/// the `search` module as a [`ColorParam`][crate::card_searcher::ColorParam].
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Colors(u8);

macro_rules! color_consts {
    ($($(#[$($attr:meta)*])* $($name:ident),+ => [$($color:tt)*];)*) => {
        $(
            color_consts!(@inner ($(($($attr)*))*) ($($name),+) [$($color)*]);
        )*
    };

    (@inner $attrs:tt ($($name:ident),+) $colors:tt) => {
        $(
            color_consts!(@inner $attrs $name $colors);
        )+
    };

    (@inner ($(($($attr:meta)*))*) $name:ident [$($color:ident),*]) => {
        $(#[$($attr)*])*
        pub const $name: Self = Colors::from_slice(&[$($color),*]);
    };
}

#[warn(missing_docs)]
impl Colors {
    color_consts! {
        #[doc = "Colorless."]
        C, COLORLESS => [];

        #[doc = "White."]
        W, WHITE => [White];
        #[doc = "Blue."]
        U, BLUE => [Blue];
        #[doc = "Black."]
        B, BLACK => [Black];
        #[doc = "Red."]
        R, RED => [Red];
        #[doc = "Green."]
        G, GREEN => [Green];

        #[doc = "White and blue. The colors of the Azorius Senate from Ravnica."]
        WU, AZORIUS => [White, Blue];
        #[doc = "Blue and black. The colors of House Dimir from Ravnica."]
        UB, DIMIR => [Blue, Black];
        #[doc = "Black and red. The colors of the Cult of Rakdos from Ravnica."]
        BR, RAKDOS => [Black, Red];
        #[doc = "Red and green. The colors of the Gruul Clans from Ravnica."]
        RG, GRUUL => [Red, Green];
        #[doc = "Green and white. The colors of the Selesnya Conclave from Ravnica."]
        WG, SELESNYA => [Green, White];
        #[doc = "White and black. The colors of the Orzhov Syndicate from Ravnica."]
        WB, ORZHOV => [White, Black];
        #[doc = "Blue and red. The colors of the Izzet League from Ravnica."]
        UR, IZZET => [Blue, Red];
        #[doc = "Black and green. The colors of the Golgari Swarm from Ravnica."]
        BG, GOLGARI => [Black, Green];
        #[doc = "Red and white. The colors of the Boros Legion from Ravnica."]
        WR, BOROS => [Red, White];
        #[doc = "Green and blue. The colors of the Simic Combine from Ravnica."]
        UG, SIMIC => [Green, Blue];

        #[doc = "White, blue, and black. The colors of the Esper shard of Alara."]
        WUB, ESPER => [White, Blue, Black];
        #[doc = "Blue, black, and red. The colors of the Grixis shard of Alara."]
        UBR, GRIXIS => [Blue, Black, Red];
        #[doc = "Black, red, and green. The colors of the Jund shard of Alara."]
        BRG, JUND => [Black, Red, Green];
        #[doc = "Red, green, and white. The colors of the Naya shard of Alara."]
        WRG, NAYA => [Red, Green, White];
        #[doc = "Green, white, and blue. The colors of the Bant shard of Alara."]
        WUG, BANT => [Green, White, Blue];
        #[doc = "White, black, and green. The colors of the Abzan Houses from Tarkir."]
        WBG, ABZAN => [White, Black, Green];
        #[doc = "Blue, red, and white. The colors of the Jeskai Way from Tarkir."]
        WUR, JESKAI => [Blue, Red, White];
        #[doc = "Black, green, and blue. The colors of the Sultai Brood from Tarkir."]
        UBG, SULTAI => [Black, Green, Blue];
        #[doc = "Red, white, and black. The colors of the Mardu Horde from Tarkir."]
        WBR, MARDU => [Red, White, Black];
        #[doc = "Green, blue, and red. The colors of the Temur Frontier from Tarkir."]
        URG, TEMUR => [Green, Blue, Red];

        #[doc = "White, blue, black, and red. The colors of artifice and \
                 [Yore-Tiller Nephilim](https://scryfall.com/card/gpt/140)."]
        WUBR, ARTIFICE => [White, Blue, Black, Red];
        #[doc = "Blue, black, red, and green. The colors of chaos and \
                 [Glint-Eye Nephilim](https://scryfall.com/card/gpt/115)."]
        UBRG, CHAOS => [Blue, Black, Red, Green];
        #[doc = "Black, red, green, and white. The colors of aggression and \
                 [Dune-Brood Nephilim](https://scryfall.com/card/gpt/110)."]
        WBRG, AGGRESSION => [Black, Red, Green, White];
        #[doc = "Red, green, white, and blue. The colors of altruism and \
                 [Ink-treader Nephilim](https://scryfall.com/card/gpt/117)."]
        WURG, ALTRUISM => [Red, Green, White, Blue];
        #[doc = "Green, white, blue, and black. The colors of growth and \
                 [Witch-Maw Nephilim](https://scryfall.com/card/gpt/138)."]
        WUBG, GROWTH => [Green, White, Blue, Black];

        #[doc = "White, blue, black, red, and green. All five colors."]
        WUBRG, ALL => [White, Blue, Black, Red, Green];
    }

    /// Constructs an instance from a list of `colors`.
    pub const fn from_slice(colors: &[Color]) -> Self {
        let mut result = Colors::colorless();
        let mut i = 0;
        while i < colors.len() {
            result.0 |= colors[i] as u8;
            i += 1;
        }
        result
    }

    /// Constructs an instance representing a single `color`.
    pub const fn monocolor(color: Color) -> Self {
        let mut result = Colors::colorless();
        result.0 |= color as u8;
        result
    }

    /// Creates an instance representing a colorless card.
    pub const fn colorless() -> Self {
        Colors(Color::Colorless as u8)
    }

    /// Checks if this instance is a certain color.
    pub const fn is(self, color: Color) -> bool {
        self.0 & color as u8 != 0
    }

    /// Checks if this instance is multicolored, which is true if it contains
    /// more than one color flag.
    pub const fn is_multicolored(self) -> bool {
        self.0.count_ones() >= 2
    }

    /// Checks if this instance is colorless.
    pub const fn is_colorless(self) -> bool {
        self.0 == 0
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
            write!(f, "{}", s)
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

impl<Rhs: Into<Colors>> ops::BitOr<Rhs> for Color {
    type Output = Colors;

    fn bitor(self, other: Rhs) -> Self::Output {
        Colors(self as u8 | other.into().0)
    }
}

impl<Rhs: Into<Colors>> ops::BitOr<Rhs> for Colors {
    type Output = Colors;

    fn bitor(self, other: Rhs) -> Self::Output {
        Colors(self.0 | other.into().0)
    }
}

/// Multicolored card. This can be used as a
/// [`ColorParam`][crate::search::ColorParam] for searching Scryfall.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Multicolored;

impl fmt::Display for Multicolored {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "m")
    }
}
