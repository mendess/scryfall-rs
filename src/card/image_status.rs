use serde::{Deserialize, Serialize};

/// As a card goes through spoiler season or other data entry, it may have no
/// imagery for a period, or low-quality imagery. You can get a
/// computer-readable value of the image’s state using the image_status field
/// on card objects.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "snake_case")]
pub enum ImageStatus {
    /// The card has no image, or the image is being processed.
    /// This value should only be temporary for very new cards.
    Missing,
    /// Scryfall doesn’t have an image of this card, but we know it exists and
    /// we have uploaded a placeholder in the meantime. This value is most
    /// common on localized cards.
    Placeholder,
    /// The card’s image is low-quality, either because it was just spoiled or
    /// we don’t have better photography for it yet.
    Lowres,
    /// This card has a full-resolution scanner image. Crisp and glossy!
    HighresScan,
}
