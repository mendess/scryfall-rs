//! Enum defining the 5 colours of magic
use serde::{Deserialize, Serialize};

/// Enum defining the 5 colours of magic
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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

/// Definition of a cards colours. This can be used to in conjunction with
/// the search builder as a [`ColourParam`].
///
/// [`ColourParam`]: ../../card_searcher/enum.ColourParam.html
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Colours(u8);

impl Colours {
    /// Creates an instance representing a multicoloured card without specifying it's colours.
    pub fn multicoloured() -> Self {
        Colours(1 << 7)
    }

    /// Creates an instance representing a colourless card.
    pub fn colourless() -> Self {
        Colours(0)
    }

    /// Checks to see if a card is a certain colour.
    ///
    /// Note: Multicoloured cards are may not be any particular colour.
    pub fn is(self, colour: Colour) -> bool {
        self.0 & (1 << (colour as u8)) != 0
    }

    /// Checks if a card is multicoloured. This only works for instances
    /// created by [`Colours::multicoloured`].
    ///
    /// [`Colours::multicoloured`]: #method.multicoloured
    pub fn is_multicoloured(self) -> bool {
        self.0 & (1 << 7) != 0
    }

    /// Checks if a card is colourless.
    pub fn is_colourless(self) -> bool {
        self.0 == 0
    }
}

impl From<&[Colour]> for Colours {
    fn from(colours: &[Colour]) -> Self {
        let mut s: u8 = 0;
        for c in colours {
            s ^= 1 << *c as u8;
        }
        Colours(s)
    }
}

impl std::fmt::Display for Colours {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Colour::*;
        if self.is_multicoloured() {
            write!(f, "m")
        } else if self.is_colourless() {
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
