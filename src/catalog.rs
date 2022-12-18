//! A Catalog object contains an array of Magic datapoints (words, card values,
//! etc). Catalog objects are provided by the API as aids for building other
//! Magic software and understanding possible values for a field on Card
//! objects.
//!
//! Visit the official [docs](https://scryfall.com/docs/api/catalogs) for more documentation.

use serde::{Deserialize, Serialize};

use crate::uri::Uri;
use crate::util::CATALOG_URL;

/// A Catalog object contains an array of Magic datapoints (words, card values,
/// etc). Catalog objects are provided by the API as aids for building other
/// Magic software and understanding possible values for a field on Card
/// objects.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Catalog {
    /// A link to the current catalog on Scryfall’s API.
    pub uri: Uri<Catalog>,

    /// The number of items in the `data` array.
    pub total_values: usize,

    /// An array of datapoints, as strings.
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
    pub async fn card_names() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("card-names")?).fetch().await
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
    pub async fn artist_names() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("artist-names")?).fetch().await
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
    pub async fn word_bank() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("word-bank")?).fetch().await
    }

    /// Returns a Catalog of all creature types in Scryfall’s database. Values
    /// are updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::creature_types().unwrap().data.len() > 0)
    /// ```
    pub async fn creature_types() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("creature-types")?).fetch().await
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
    pub async fn planeswalker_types() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("planeswalker-types")?)
            .fetch()
            .await
    }

    /// Returns a Catalog of all Land types in Scryfall’s database. Values are
    /// updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::land_types().unwrap().data.len() > 0)
    /// ```
    pub async fn land_types() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("land-types")?).fetch().await
    }

    /// Returns a Catalog of all artifact types in Scryfall’s database. Values
    /// are updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::artifact_types().unwrap().data.len() > 0)
    /// ```
    pub async fn artifact_types() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("artifact-types")?).fetch().await
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
    pub async fn enchantment_types() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("enchantment-types")?)
            .fetch()
            .await
    }

    /// Returns a Catalog of all spell types in Scryfall’s database. Values are
    /// updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::spell_types().unwrap().data.len() > 0)
    /// ```
    pub async fn spell_types() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("spell-types")?).fetch().await
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
    pub async fn powers() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("powers")?).fetch().await
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
    pub async fn toughnesses() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("toughnesses")?).fetch().await
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
    pub async fn loyalties() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("loyalties")?).fetch().await
    }

    /// Returns a Catalog of all card watermarks in Scryfall’s database. Values
    /// are updated as soon as a new card is entered for spoiler seasons.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::catalog::Catalog;
    /// assert!(Catalog::watermarks().unwrap().data.len() > 0)
    /// ```
    pub async fn watermarks() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("watermarks")?).fetch().await
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
    pub async fn keyword_abilities() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("keyword-abilities")?)
            .fetch()
            .await
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
    pub async fn keyword_actions() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("keyword-actions")?)
            .fetch()
            .await
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
    pub async fn ability_words() -> crate::Result<Self> {
        Uri::from(CATALOG_URL.join("ability-words")?).fetch().await
    }
}
