use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
/// A type of mana a card can produce
pub enum ProducedMana {
    /// A normal color of magic
    Color(super::color::Color),
    /// An unfinity kind of mana
    UnfinityMana(UnfinityMana),
}

impl ProducedMana {
    #[allow(non_upper_case_globals)]
    /// Alias to unfinity "tap" mana symbol
    pub const Tap: Self = Self::UnfinityMana(UnfinityMana::Two);
    #[allow(non_upper_case_globals)]
    /// Alias to unfinity 2 mana symbol
    pub const Two: Self = Self::UnfinityMana(UnfinityMana::Two);
}

/// Kinds of mana only produced in unfinity
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum UnfinityMana {
    /// Some sticker sheets have stickers that give creatures the ability to
    /// generate 2 colorless mana, for some reason wizards used the old
    /// templating
    ///
    /// ## Examples
    /// - [Happy Dead Squirrel](https://scryfall.com/card/sunf/8/happy-dead-squirrel)
    /// - [Unglued Pea-Brained Dinosaur](https://scryfall.com/card/sunf/45/unglued-pea-brained-dinosaur)
    #[serde(rename = "2")]
    Two,
    /// The is one unfinity card that produces this:
    /// [Sole Performer](https://scryfall.com/card/unf/440/sole-performer)
    #[serde(rename = "T")]
    Tap,
}

#[cfg(test)]
mod test {
    use serde_json::from_str;

    use super::super::color::Color;
    use super::*;

    #[test]
    fn color() {
        let c = from_str::<ProducedMana>("\"W\"").unwrap();
        assert_eq!(c, ProducedMana::Color(Color::White))
    }

    #[test]
    fn two() {
        let c = from_str::<ProducedMana>("\"2\"").unwrap();
        assert_eq!(c, ProducedMana::UnfinityMana(UnfinityMana::Two))
    }

    #[test]
    fn tap() {
        let c = from_str::<ProducedMana>("\"T\"").unwrap();
        assert_eq!(c, ProducedMana::UnfinityMana(UnfinityMana::Tap))
    }

    #[test]
    fn in_json() {
        let s = r#"{ "produced_mana": [ "B" ] }"#;
        #[derive(Deserialize)]
        struct T {
            produced_mana: Vec<ProducedMana>,
        }

        let t = from_str::<T>(s).unwrap();
        assert_eq!(t.produced_mana, [ProducedMana::Color(Color::Black)])
    }
}
