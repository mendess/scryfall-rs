//! Enum defining the 5 colors of magic
use serde::{Deserialize, Serialize};

/// Enum defining the 5 colors of magic
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[allow(missing_docs)]
pub enum Colour {
    #[serde(rename = "W")]
    White = 0,
    #[serde(rename = "U")]
    Blue = 1,
    #[serde(rename = "B")]
    Black = 2,
    #[serde(rename = "R")]
    Red = 3,
    #[serde(rename = "G")]
    Green = 4,
}

/// Definition of a cards colors. This can be used to in conjunction with
/// the search builder as a [`ColourParam`].
///
/// [`ColourParam`]: ../../card_searcher/enum.ColourParam.html
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Colours(u8);

impl Colours {
    /// Creates an instance representing a multicolored card without specifying it's colors.
    pub fn multicolored() -> Self {
        Colours(1 << 7)
    }

    /// Creates an instance representing a colorless card.
    pub fn colorless() -> Self {
        Colours(0)
    }

    /// Checks to see if a card is a certain color.
    ///
    /// Note: Multicolored cards are may not be any particular color.
    pub fn is(self, color: Colour) -> bool {
        self.0 & (1 << (color as u8)) != 0
    }

    /// Checks if a card is multicolored. This only works for instances
    /// created by [`Colours::multicolored`].
    ///
    /// [`Colours::multicolored`]: #method.multicolored
    pub fn is_multicolored(self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// Checks if a card is colorless.
    pub fn is_colorless(self) -> bool {
        self.0 == 0
    }
}

impl From<&[Colour]> for Colours {
    fn from(colors: &[Colour]) -> Self {
        let mut s: u8 = 0;
        for c in colors {
            s ^= 1 << *c as u8;
        }
        Colours(s)
    }
}

impl std::fmt::Display for Colours {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Colour::*;
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
