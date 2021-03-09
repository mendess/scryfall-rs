use std::fmt;

use crate::card::Color::*;
use crate::card::Colors;

/// The guilds of Ravnica, commonly used as an alias for color pairs.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum Guild {
    /// The Azorius Senate, aligned with white and blue (WU).
    Azorius,
    /// The Boros Legion, aligned with red and white (RW).
    Boros,
    /// House Dimir, aligned with blue and black (UB).
    Dimir,
    /// The Golgari Swarm, aligned with black and green (BG).
    Golgari,
    /// The Gruul Clans, aligned with red and green (RG).
    Gruul,
    /// The Izzet League, aligned with blue and red (UR).
    Izzet,
    /// The Orzhov Syndicate, aligned with white and black (WB).
    Orzhov,
    /// The Cult of Rakdos, aligned with black and red (BR).
    Rakdos,
    /// The Selesnya Conclave, aligned with green and white (GW).
    Selesnya,
    /// The Simic Combine, aligned with green and blue (GU).
    Simic,
}

impl fmt::Display for Guild {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Guild::Azorius => "azorius",
                Guild::Boros => "boros",
                Guild::Dimir => "dimir",
                Guild::Golgari => "golgari",
                Guild::Gruul => "gruul",
                Guild::Izzet => "izzet",
                Guild::Orzhov => "orzhov",
                Guild::Rakdos => "rakdos",
                Guild::Selesnya => "selesnya",
                Guild::Simic => "simic",
            }
        )
    }
}

const fn colors_from_guild(guild: Guild) -> Colors {
    let colors = match guild {
        Guild::Azorius => [White, Blue],
        Guild::Boros => [Red, White],
        Guild::Dimir => [Blue, Black],
        Guild::Golgari => [Black, Green],
        Guild::Gruul => [Red, Green],
        Guild::Izzet => [Blue, Red],
        Guild::Orzhov => [White, Black],
        Guild::Rakdos => [Black, Red],
        Guild::Selesnya => [Green, White],
        Guild::Simic => [Green, Blue],
    };
    Colors::from_slice(&colors)
}

impl From<Guild> for Colors {
    fn from(guild: Guild) -> Self {
        colors_from_guild(guild)
    }
}

/// The shards of Alara, commonly used as aliases for color shards (three colors
/// in an unbroken chain in the color pie).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum Shard {
    /// The shard of Bant, aligned with green, white, and blue (GWU).
    Bant,
    /// The shard of Esper, aligned with white, blue, and black (WUB).
    Esper,
    /// The shard of Grixis, aligned with blue, black, and red (UBR).
    Grixis,
    /// The shard of Jund, aligned with black, red, and green (BRG).
    Jund,
    /// The shard of Naya, aligned with red, green, and white (RGW).
    Naya,
}

impl fmt::Display for Shard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Shard::Bant => "bant",
                Shard::Esper => "esper",
                Shard::Grixis => "grixis",
                Shard::Jund => "jund",
                Shard::Naya => "naya",
            }
        )
    }
}

const fn colors_from_shard(shard: Shard) -> Colors {
    let colors = match shard {
        Shard::Bant => [Green, White, Blue],
        Shard::Esper => [White, Blue, Black],
        Shard::Grixis => [Blue, Black, Red],
        Shard::Jund => [Black, Red, Green],
        Shard::Naya => [Red, Green, White],
    };
    Colors::from_slice(&colors)
}

impl From<Shard> for Colors {
    fn from(shard: Shard) -> Self {
        colors_from_shard(shard)
    }
}

/// The clans of Tarkir, commonly used as aliases for color wedges (one color
/// and its two enemy colors).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum Wedge {
    /// The Abzan Houses, aligned with white, black, and green (WBG).
    Abzan,
    /// The Jeskai Way, aligned with blue, red, and white (URW).
    Jeskai,
    /// The Mardu Horde, aligned with red, white, and black (RWB).
    Mardu,
    /// The Sultai Brood, aligned with black, green, and blue (BGU).
    Sultai,
    /// The Temur Frontier, aligned with green, blue, and red (GUR).
    Temur,
}

impl fmt::Display for Wedge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Wedge::Abzan => "abzan",
                Wedge::Jeskai => "jeskai",
                Wedge::Mardu => "mardu",
                Wedge::Sultai => "sultai",
                Wedge::Temur => "temur",
            }
        )
    }
}

const fn colors_from_wedge(wedge: Wedge) -> Colors {
    let colors = match wedge {
        Wedge::Abzan => [White, Black, Green],
        Wedge::Jeskai => [Blue, Red, White],
        Wedge::Mardu => [Red, White, Black],
        Wedge::Sultai => [Black, Green, Blue],
        Wedge::Temur => [Green, Blue, Red],
    };
    Colors::from_slice(&colors)
}

impl From<Wedge> for Colors {
    fn from(wedge: Wedge) -> Self {
        colors_from_wedge(wedge)
    }
}

/// The four-color aliases supported by Scryfall.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum FourColor {
    /// Black, green, red, and white (BGRW).
    Aggression,
    /// Green, red, blue, and white (GRUW).
    Altruism,
    /// Black, red, blue, and white (BRUW).
    Artifice,
    /// Black, green, red, and blue (BGRU).
    Chaos,
    /// Black, green, blue, and white (BGUW).
    Growth,
}

impl fmt::Display for FourColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FourColor::Aggression => "aggression",
                FourColor::Altruism => "altruism",
                FourColor::Artifice => "artifice",
                FourColor::Chaos => "chaos",
                FourColor::Growth => "growth",
            }
        )
    }
}

const fn colors_from_four_color(alias: FourColor) -> Colors {
    let colors = match alias {
        FourColor::Aggression => [Black, Red, Green, White],
        FourColor::Altruism => [Red, Green, White, Blue],
        FourColor::Artifice => [White, Blue, Black, Red],
        FourColor::Chaos => [Blue, Black, Red, Green],
        FourColor::Growth => [Green, White, Blue, Black],
    };
    Colors::from_slice(&colors)
}

impl From<FourColor> for Colors {
    fn from(alias: FourColor) -> Self {
        colors_from_four_color(alias)
    }
}
