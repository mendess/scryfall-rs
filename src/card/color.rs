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

impl Colors {
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
