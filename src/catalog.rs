//! A Catalog object contains an array of Magic datapoints (words, card values, etc). Catalog
//! objects are provided by the API as aids for building other Magic software and understanding
//! possible values for a field on Card objects.
//!
//! Visit the oficial [docs](https://scryfall.com/docs/api/catalogs) for more documentation.

use serde::{Deserialize, Serialize};

use crate::util::uri::{url_fetch, URI};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
#[allow(missing_docs)]
pub struct Catalog {
    pub uri: URI<Catalog>,
    pub data: Vec<String>,
}

impl Catalog {
    /// Returns a list of all nontoken English card names in Scryfall’s database. Values are updated
    /// as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::card_names().unwrap().data.len() > 0)
    /// ```
    pub fn card_names() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/card-names")
    }

    /// Returns a list of all canonical artist names in Scryfall’s database. This catalog won’t
    /// include duplicate, misspelled, or funny names for artists. Values are updated as soon as a
    /// new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::artist_names().unwrap().data.len() > 0)
    /// ```
    pub fn artist_names() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/artist-names")
    }

    /// Returns a Catalog of all English words, of length 2 or more, that could appear in a card
    /// name. Values are drawn from cards currently in Scryfall’s database. Values are updated as
    /// soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::word_bank().unwrap().data.len() > 0)
    /// ```
    pub fn word_bank() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/word-bank")
    }

    /// Returns a Catalog of all creature types in Scryfall’s database. Values are updated as soon
    /// as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::creature_types().unwrap().data.len() > 0)
    /// ```
    pub fn creature_types() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/creature-types")
    }

    /// Returns a Catalog of all Planeswalker types in Scryfall’s database. Values are updated as
    /// soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::planeswalker_types().unwrap().data.len() > 0)
    /// ```
    pub fn planeswalker_types() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/planeswalker-types")
    }

    /// Returns a Catalog of all Land types in Scryfall’s database. Values are updated as soon as a
    /// new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::land_types().unwrap().data.len() > 0)
    /// ```
    pub fn land_types() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/land-types")
    }

    /// Returns a Catalog of all artifact types in Scryfall’s database. Values are updated as soon
    /// as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::artifact_types().unwrap().data.len() > 0)
    /// ```
    pub fn artifact_types() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/artifact-types")
    }

    /// Returns a Catalog of all enchantment types in Scryfall’s database. Values are updated as
    /// soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::enchantment_types().unwrap().data.len() > 0)
    /// ```
    pub fn enchantment_types() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/enchantment-types")
    }

    /// Returns a Catalog of all spell types in Scryfall’s database. Values are updated as soon as
    /// a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::spell_types().unwrap().data.len() > 0)
    /// ```
    pub fn spell_types() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/spell-types")
    }

    /// Returns a Catalog of all possible values for a creature or vehicle’s power in Scryfall’s
    /// database. Values are updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::powers().unwrap().data.len() > 0)
    /// ```
    pub fn powers() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/powers")
    }

    /// Returns a Catalog of all possible values for a creature or vehicle’s toughness in
    /// Scryfall’s database. Values are updated as soon as a new card is entered for spoiler
    /// seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::toughnesses().unwrap().data.len() > 0)
    /// ```
    pub fn toughnesses() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/toughnesses")
    }

    /// Returns a Catalog of all possible values for a Planeswalker’s loyalty in Scryfall’s
    /// database. Values are updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::loyalties().unwrap().data.len() > 0)
    /// ```
    pub fn loyalties() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/loyalties")
    }

    /// Returns a Catalog of all card watermarks in Scryfall’s database. Values are updated as soon
    /// as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::watermarks().unwrap().data.len() > 0)
    /// ```
    pub fn watermarks() -> crate::Result<Self> {
        url_fetch("https://api.scryfall.com/catalog/watermarks")
    }
}
