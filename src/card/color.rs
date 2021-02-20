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

/// Definition of a cards colors. This can be used to in conjunction with
/// the search builder as a [`ColorParam`][crate::card_searcher::ColorParam].
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Colors(u8);

impl Colors {
    /// Creates an instance representing a multicolored card without specifying
    /// its colors.
    pub fn multicolored() -> Self {
        Colors(1 << 7)
    }

    /// Creates an instance representing a colorless card.
    pub fn colorless() -> Self {
        Colors(Color::Colorless as u8)
    }

    /// Checks to see if a card is a certain color.
    ///
    /// Note: Multicolored cards are may not be any particular color.
    pub fn is(self, color: Color) -> bool {
        self.0 & color as u8 != 0
    }

    /// Checks if a card is multicolored. This only works for instances
    /// created by [`Colors::multicolored`].
    ///
    /// [`Colors::multicolored`]: #method.multicolored
    pub fn is_multicolored(self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// Checks if a card is colorless.
    pub fn is_colorless(self) -> bool {
        self.0 == 0
    }
}

impl From<&[Color]> for Colors {
    fn from(colors: &[Color]) -> Self {
        Colors(colors.iter().fold(0, |acc, c| acc | *c as u8))
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
