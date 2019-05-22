//! Enum defining the 5 colors of magic
use serde::Deserialize;

/// Enum defining the 5 colors of magic
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Colors(u8);

impl Colors {
    pub fn from_slice(colors: &[Color]) -> Self {
        let mut s: u8 = 0;
        for c in colors {
            s ^= 1 << *c as u8;
        }
        Colors(s)
    }

    pub fn multicolored() -> Self {
        Colors(1 << 7)
    }

    pub fn is(self, color: Color) -> bool {
        self.0 & (1 << (color as u8)) != 0
    }

    pub fn is_colorless(self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Color::*;
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
