use serde::{Deserialize, Serialize};

/// The layout property categorizes the arrangement of card parts, faces, and
/// other bounded regions on cards. The layout can be used to programmatically
/// determine which other properties on a card you can expect.
///
/// Specifically:
///
/// * Cards with the layouts split, flip, transform, and double_faced_token will
///   always have a card_faces property describing the distinct faces.
///
/// * Cards with the layout meld will always have a related_cards property
///   pointing to the other meld parts.
///
/// [Official docs](https://scryfall.com/docs/api/layouts#layout)
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(not(feature = "unknown_variants"), derive(Copy))]
#[cfg_attr(
    all(
        not(feature = "unknown_variants"),
        not(feature = "unknown_variants_slim")
    ),
    non_exhaustive
)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    /// A standard Magic card with one face.
    Normal,
    /// A split-faced card.
    Split,
    /// Cards that invert vertically with the flip keyword.
    Flip,
    /// Double-sided cards that transform.
    Transform,
    /// Double-sided cards that can be played either-side.
    ModalDfc,
    /// Cards with meld parts printed on the back.
    Meld,
    /// Cards with Level Up.
    Leveler,
    /// Class-type enchantment cards
    Class,
    /// Saga-type cards.
    Saga,
    /// Cards with an Adventure spell part.
    Adventure,
    /// Plane and Phenomenon-type cards.
    Planar,
    /// Scheme-type cards.
    Scheme,
    /// Vanguard-type cards.
    Vanguard,
    /// Token cards.
    Token,
    /// Tokens with another token printed on the back.
    DoubleFacedToken,
    /// Emblem cards.
    Emblem,
    /// Cards with Augment.
    Augment,
    /// Host-type cards.
    Host,
    /// Art Series collectable double-faced cards.
    ArtSeries,
    /// A Magic card with two sides that are unrelated.
    ReversibleCard,
    /// Prototype
    Prototype,
    /// Mutate
    Mutate,
    /// Case
    Case,
    #[cfg_attr(
        doc,
        doc(cfg(any(feature = "unknown_variants", feature = "unknown_variants_slim")))
    )]
    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    /// Unknown layout
    Unknown(Box<str>),
    #[cfg_attr(
        doc,
        doc(cfg(any(feature = "unknown_variants", feature = "unknown_variants_slim")))
    )]
    #[cfg(all(not(feature = "unknown_variants"), feature = "unknown_variants_slim"))]
    #[serde(other)]
    /// Unknown layout
    Unknown,
}
