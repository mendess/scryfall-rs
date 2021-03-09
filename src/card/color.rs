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
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Colors(u8);

impl Colors {
    /// Azorius (White, Blue)
    pub const AZORIUS: Self = Colors::colorless().white().blue();
    /// Dimir (Blue, Black)
    pub const DIMIR: Self = Colors::colorless().blue().black();
    /// Rakdos (Black, Red)
    pub const RAKDOS: Self = Colors::colorless().black().red();
    /// Grull (Red, Green)
    pub const GRUUL: Self = Colors::colorless().red().green();
    /// Selesnya (White, Green)
    pub const SELESNYA: Self = Colors::colorless().green().white();
    /// Orzhov (White, Black)
    pub const ORZHOV: Self = Colors::colorless().white().black();
    /// Izzet (Blue, Red)
    pub const IZZET: Self = Colors::colorless().blue().red();
    /// Golgari (Black, Green)
    pub const GOLGARI: Self = Colors::colorless().black().green();
    /// Boros (White, Red)
    pub const BOROS: Self = Colors::colorless().red().white();
    /// Simic (Blue, Green)
    pub const SIMIC: Self = Colors::colorless().green().blue();
    /// Bant (White, Blue, Green)
    pub const BANT: Self = Colors::colorless().white().blue().green();
    /// Esper (White, Blue, Black)
    pub const ESPER: Self = Colors::colorless().white().blue().black();
    /// Grixi (Blue, Black, Red)
    pub const GRIXIS: Self = Colors::colorless().blue().black().red();
    /// Jund (Black, Red, Green)
    pub const JUND: Self = Colors::colorless().black().red().green();
    /// Naya (White, Red, Green)
    pub const NAYA: Self = Colors::colorless().white().red().green();
    /// Abzan (White, Black, Green)
    pub const ABZAN: Self = Colors::colorless().white().black().green();
    /// Jeskai (White, Blue, Red)
    pub const JESKAI: Self = Colors::colorless().white().blue().red();
    /// Sultai (Blue, Black, Green)
    pub const SULTAI: Self = Colors::colorless().blue().black().green();
    /// Mardu (White, Black, Red)
    pub const MARDU: Self = Colors::colorless().white().black().red();
    /// Temur (Blue, Red, Green)
    pub const TEMUR: Self = Colors::colorless().blue().red().green();
    /// Chaos (White, Blue, Black, Red)
    pub const CHAOS: Self = Colors::colorless().white().blue().black().red();
    /// Artifice (Blue, Black, Red, Green)
    pub const ARTIFICE: Self = Colors::colorless().blue().black().red().green();
    /// Aggression (Black, Red, Green, White)
    pub const AGGRESSION: Self = Colors::colorless().black().red().green().white();
    /// Altruism (Red, Green, White, Blue)
    pub const ALTRUISM: Self = Colors::colorless().red().green().white().blue();
    /// Growth (Green, White, Blue, Black)
    pub const GROWTH: Self = Colors::colorless().green().white().blue().black();
    /// WUBRG (White, Blue, Black, Red, Green)
    pub const WUBRG: Self = Colors::colorless().white().blue().black().red().green();

    /// Creates an instance representing a colorless card.
    pub const fn colorless() -> Self {
        Colors(Color::Colorless as u8)
    }

    /// Checks to see if a card is a certain color.
    pub const fn is(self, color: Color) -> bool {
        self.0 & color as u8 != 0
    }

    /// Checks if a card is multicolored.
    pub const fn is_multicolored(self) -> bool {
        self.0.count_ones() > 1
    }

    /// Checks if a card is colorless.
    pub const fn is_colorless(self) -> bool {
        self.0 == 0
    }

    /// Adds white
    pub const fn white(self) -> Self {
        self.add(Color::White)
    }

    /// Adds blue
    pub const fn blue(self) -> Self {
        self.add(Color::Blue)
    }

    /// Adds black
    pub const fn black(self) -> Self {
        self.add(Color::Black)
    }

    /// Adds red
    pub const fn red(self) -> Self {
        self.add(Color::Red)
    }

    /// Adds green
    pub const fn green(self) -> Self {
        self.add(Color::Green)
    }

    /// Add a color
    pub const fn add(mut self, color: Color) -> Self {
        self.0 |= color as u8;
        self
    }

    /// Remove a color
    pub const fn remove(mut self, color: Color) -> Self {
        self.0 &= !(color as u8);
        self
    }
}

impl From<&[Color]> for Colors {
    fn from(colors: &[Color]) -> Self {
        colors.iter().fold(Self::colorless(), |acc, c| acc.add(*c))
    }
}

impl From<Color> for Colors {
    fn from(color: Color) -> Self {
        Self::colorless().add(color)
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

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    macro_rules! single_color {
        ($($lower:ident => $upper:ident,)+) => {
            $(
                #[test]
                fn $lower() {
                    assert!(Colors::default().$lower().is(Color::$upper))
                }

                paste! {
                    #[test]
                    fn [<repeat_ $lower _ok>]() {
                        let double = Colors::default().$lower().$lower();
                        let single = Colors::default().$lower();
                        assert_eq!(double, single);
                    }
                }
            )*
        }
    }

    single_color! {
        white => White,
        blue => Blue,
        black => Black,
        red => Red,
        green => Green,
    }
}
