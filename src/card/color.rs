use std::fmt;

use serde::{Deserialize, Serialize};

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

/// Definition of a cards colors. This can be used to in conjunction with
/// the search builder as a [`ColorParam`][crate::card_searcher::ColorParam].
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Colors(u8);

impl Colors {
    /// Constructs a `Colors` object from a list of individual colors.
    pub const fn from_slice(colors: &[Color]) -> Self {
        let mut result = Colors::colorless();
        let mut i = 0;
        while i < colors.len() {
            result.0 |= colors[i] as u8;
            i += 1;
        }
        result
    }

    /// Creates an instance representing a multicolored card without specifying
    /// its colors.
    pub const fn multicolored() -> Self {
        Colors(1 << 7)
    }

    /// Creates an instance representing a colorless card.
    pub const fn colorless() -> Self {
        Colors(Color::Colorless as u8)
    }

    /// Checks to see if a card is a certain color.
    ///
    /// Note: Multicolored cards are may not be any particular color.
    pub const fn is(self, color: Color) -> bool {
        self.0 & color as u8 != 0
    }

    /// Checks if a card is multicolored. This only works for instances
    /// created by [`Colors::multicolored`].
    ///
    /// [`Colors::multicolored`]: #method.multicolored
    pub const fn is_multicolored(self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// Checks if a card is colorless.
    pub const fn is_colorless(self) -> bool {
        self.0 == 0
    }
}

impl From<&[Color]> for Colors {
    fn from(colors: &[Color]) -> Self {
        Colors::from_slice(colors)
    }
}

impl std::fmt::Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Color::*;
        if self.is_multicolored() {
            write!(f, "m")
        } else if self.is_colorless() {
            write!(f, "c")
        } else {
            let mut s = String::new();
            if self.is(White) {
                s += "w";
            }
            if self.is(Blue) {
                s += "u";
            }
            if self.is(Black) {
                s += "b";
            }
            if self.is(Red) {
                s += "r";
            }
            if self.is(Green) {
                s += "g";
            }
            write!(f, "{}", s)
        }
    }
}
