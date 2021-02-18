//! A Catalog object contains an array of Magic datapoints (words, card values,
//! etc). Catalog objects are provided by the API as aids for building other
//! Magic software and understanding possible values for a field on Card
//! objects.
//!
//! Visit the official [docs](https://scryfall.com/docs/api/catalogs) for more documentation.

use serde::{Deserialize, Serialize};

use crate::uri::Uri;
use crate::util::CATALOG_URL;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
#[allow(missing_docs)]
pub struct Catalog {
    pub uri: Uri<Catalog>,
    pub data: Vec<String>,
}

impl Catalog {
    /// Returns a list of all nontoken English card names in Scryfall’s
    /// database. Values are updated as soon as a new card is entered for
    /// spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::card_names().unwrap().data.len() > 0)
    /// ```
    pub fn card_names() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("card-names")?).fetch()
    }

    /// Returns a list of all canonical artist names in Scryfall’s database.
    /// This catalog won’t include duplicate, misspelled, or funny names for
    /// artists. Values are updated as soon as a new card is entered for
    /// spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::artist_names().unwrap().data.len() > 0)
    /// ```
    pub fn artist_names() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("artist-names")?).fetch()
    }

    /// Returns a Catalog of all English words, of length 2 or more, that could
    /// appear in a card name. Values are drawn from cards currently in
    /// Scryfall’s database. Values are updated as soon as a new card is
    /// entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::word_bank().unwrap().data.len() > 0)
    /// ```
    pub fn word_bank() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("word-bank")?).fetch()
    }

    /// Returns a Catalog of all creature types in Scryfall’s database. Values
    /// are updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::creature_types().unwrap().data.len() > 0)
    /// ```
    pub fn creature_types() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("creature-types")?).fetch()
    }

    /// Returns a Catalog of all Planeswalker types in Scryfall’s database.
    /// Values are updated as soon as a new card is entered for spoiler
    /// seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::planeswalker_types().unwrap().data.len() > 0)
    /// ```
    pub fn planeswalker_types() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("planeswalker-types")?).fetch()
    }

    /// Returns a Catalog of all Land types in Scryfall’s database. Values are
    /// updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::land_types().unwrap().data.len() > 0)
    /// ```
    pub fn land_types() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("land-types")?).fetch()
    }

    /// Returns a Catalog of all artifact types in Scryfall’s database. Values
    /// are updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::artifact_types().unwrap().data.len() > 0)
    /// ```
    pub fn artifact_types() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("artifact-types")?).fetch()
    }

    /// Returns a Catalog of all enchantment types in Scryfall’s database.
    /// Values are updated as soon as a new card is entered for spoiler
    /// seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::enchantment_types().unwrap().data.len() > 0)
    /// ```
    pub fn enchantment_types() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("enchantment-types")?).fetch()
    }

    /// Returns a Catalog of all spell types in Scryfall’s database. Values are
    /// updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::spell_types().unwrap().data.len() > 0)
    /// ```
    pub fn spell_types() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("spell-types")?).fetch()
    }

    /// Returns a Catalog of all possible values for a creature or vehicle’s
    /// power in Scryfall’s database. Values are updated as soon as a new
    /// card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::powers().unwrap().data.len() > 0)
    /// ```
    pub fn powers() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("powers")?).fetch()
    }

    /// Returns a Catalog of all possible values for a creature or vehicle’s
    /// toughness in Scryfall’s database. Values are updated as soon as a
    /// new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::toughnesses().unwrap().data.len() > 0)
    /// ```
    pub fn toughnesses() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("toughnesses")?).fetch()
    }

    /// Returns a Catalog of all possible values for a Planeswalker’s loyalty in
    /// Scryfall’s database. Values are updated as soon as a new card is
    /// entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::loyalties().unwrap().data.len() > 0)
    /// ```
    pub fn loyalties() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("loyalties")?).fetch()
    }

    /// Returns a Catalog of all card watermarks in Scryfall’s database. Values
    /// are updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::watermarks().unwrap().data.len() > 0)
    /// ```
    pub fn watermarks() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("watermarks")?).fetch()
    }

    /// Returns a Catalog of all keyword abilities in Scryfall’s database.
    /// Values are updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// # use scryfall::catalog::Catalog;
    /// assert!(
    ///     Catalog::keyword_abilities()
    ///         .unwrap()
    ///         .data
    ///         .iter()
    ///         .find(|a| a.as_str() == "Haste")
    ///         .is_some()
    /// );
    /// ```
    pub fn keyword_abilities() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("keyword-abilities")?).fetch()
    }

    /// Returns a Catalog of all keyword actions in Scryfall’s database. Values
    /// are updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// # use scryfall::catalog::Catalog;
    /// assert!(
    ///     Catalog::keyword_actions()
    ///         .unwrap()
    ///         .data
    ///         .iter()
    ///         .find(|a| a.as_str() == "Scry")
    ///         .is_some()
    /// );
    /// ```
    pub fn keyword_actions() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("keyword-actions")?).fetch()
    }

    /// Returns a Catalog of all ability words in Scryfall’s database. Values
    /// are updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// # use scryfall::catalog::Catalog;
    /// assert!(
    ///     Catalog::ability_words()
    ///         .unwrap()
    ///         .data
    ///         .iter()
    ///         .find(|a| a.as_str() == "Landfall")
    ///         .is_some()
    /// );
    /// ```
    pub fn ability_words() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("ability-words")?).fetch()
    }
}
