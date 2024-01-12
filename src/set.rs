//! A Set object represents a group of related Magic cards. All Card objects on
//! Scryfall belong to exactly one set.
//!
//! Due to Magic’s long and complicated history, Scryfall includes many
//! un-official sets as a way to group promotional or outlier cards together.
//! Such sets will likely have a code that begins with `p` or `t`, such as
//! `pcel` or `tori`.
//!
//! Official sets always have a three-letter set code, such as `zen`.
mod set_code;
mod set_type;

use chrono::NaiveDate;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use self::set_code::SetCode;
pub use self::set_type::SetType;
use crate::card::Card;
use crate::list::{List, ListIter};
use crate::uri::Uri;
use crate::util::SETS_URL;

/// A Set object containing all fields that `scryfall` provides.
///
/// For more details visit the [official docs](https://scryfall.com/docs/api/sets).
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Set {
    /// A unique ID for this set on Scryfall that will not change.
    pub id: Uuid,

    /// The unique three to five-letter code for this set.
    pub code: SetCode,

    /// The unique code for this set on MTGO, which may differ from the regular
    /// code.
    pub mtgo_code: Option<String>,

    /// This set’s ID on TCGplayer’s API, also known as the groupId.
    pub tcgplayer_id: Option<u64>,

    /// The English name of the set.
    pub name: String,

    /// A computer-readable classification for this set.
    pub set_type: SetType,

    /// The date the set was released or the first card was printed in the set
    /// (in GMT-8 Pacific time).
    pub released_at: Option<NaiveDate>,

    /// The block code for this set, if any.
    pub block_code: Option<String>,

    /// The block or group name code for this set, if any.
    pub block: Option<String>,

    /// The set code for the parent set, if any. promo and token sets often have
    /// a parent set.
    pub parent_set_code: Option<String>,

    /// The number of cards in this set.
    pub card_count: usize,

    /// The denominator for the set’s printed collector numbers.
    pub printed_size: Option<usize>,

    /// True if this set was only released in a video game.
    pub digital: bool,

    /// True if this set contains only foil cards.
    pub foil_only: bool,

    /// True if this set contains only nonfoil cards.
    pub nonfoil_only: bool,

    /// A link to this set’s permapage on Scryfall’s website.
    pub scryfall_uri: String,

    /// A link to this set object on Scryfall’s API.
    pub uri: Uri<Set>,

    /// A URI to an SVG file for this set’s icon on Scryfall’s CDN. Hotlinking
    /// this image isn’t recommended, because it may change slightly over time.
    /// You should download it and use it locally for your particular user
    /// interface needs.
    pub icon_svg_uri: String,

    /// A Scryfall API URI that you can request to begin paginating over the
    /// cards in this set.
    pub search_uri: Uri<List<Card>>,
}

impl Set {
    /// Returns a [`ListIter`] of all the sets in the `scryfall` database.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::set::Set;
    /// # tokio_test::block_on(async {
    /// let sets = Set::all().await.unwrap().into_inner().collect::<Vec<_>>();
    /// assert!(sets.len() > 0);
    /// # })
    /// ```
    pub async fn all() -> crate::Result<ListIter<Set>> {
        let mut url = SETS_URL.clone();
        url.query_pairs_mut().append_pair("page", "1");
        Uri::from(url).fetch_iter().await
    }

    /// Returns a `Set` with the given set code.
    ///
    /// The code can be either the `code` or the `mtgo_code` for the set.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::set::Set;
    /// # tokio_test::block_on(async {
    /// assert_eq!(Set::code("mmq").await.unwrap().name, "Mercadian Masques")
    /// # })
    /// ```
    pub async fn code(code: &str) -> crate::Result<Set> {
        Uri::from(SETS_URL.join(&percent_encode(code.as_bytes(), NON_ALPHANUMERIC).to_string())?)
            .fetch()
            .await
    }

    /// Returns a `Set` with the given `tcgplayer_id`.
    ///
    /// Also known as the `groupId` on [TCGplayer’s API](https://docs.tcgplayer.com/docs).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use scryfall::set::Set;
    /// # tokio_test::block_on(async {
    /// assert_eq!(Set::tcgplayer(1909).await.unwrap().name, "Amonkhet Invocations")
    /// # })
    /// ```
    pub async fn tcgplayer<T: std::fmt::Display>(code: T) -> crate::Result<Set> {
        Uri::from(
            SETS_URL
                .join("tcgplayer/")?
                .join(&percent_encode(code.to_string().as_bytes(), NON_ALPHANUMERIC).to_string())?,
        )
        .fetch()
        .await
    }

    /// Returns a Set with the given Scryfall `uuid`.
    ///
    /// # Examples
    /// ```rust
    /// use scryfall::set::Set;
    /// # tokio_test::block_on(async {
    /// assert_eq!(
    ///     Set::uuid("2ec77b94-6d47-4891-a480-5d0b4e5c9372".parse().unwrap())
    ///         .await
    ///         .unwrap()
    ///         .name,
    ///     "Ultimate Masters"
    /// )
    /// # })
    /// ```
    pub async fn uuid(uuid: Uuid) -> crate::Result<Set> {
        Uri::from(SETS_URL.join(&uuid.to_string())?).fetch().await
    }

    /// Returns an iterator over the cards of the set.
    pub async fn cards(&self) -> crate::Result<ListIter<Card>> {
        self.search_uri.fetch_iter().await
    }
}
