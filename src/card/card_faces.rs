use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::card::Color;
use crate::card::ImageUris;

use super::Layout;

/// Multiface cards have a card_faces property containing at least two Card Face
/// objects.
///
/// ---
///
/// For more information, refer to the [official docs](https://scryfall.com/docs/api/cards#card-face-objects).
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[non_exhaustive]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct CardFace {
    /// The name of the illustrator of this card face. Newly spoiled cards may
    /// not have this field yet.
    pub artist: Option<String>,

    /// The colors in this face’s color indicator, if any.
    pub color_indicator: Option<Vec<Color>>,

    /// This face’s colors, if the game defines colors for the individual face
    /// of this card.
    pub colors: Option<Vec<Color>>,

    /// The mana value of this particular face, if the card is reversible.
    pub cmc: Option<f32>,

    /// The flavor text printed on this face, if any.
    pub flavor_text: Option<String>,

    /// A unique identifier for the card face artwork that remains consistent
    /// across reprints. Newly spoiled cards may not have this field yet.
    pub illustration_id: Option<Uuid>,

    /// An object providing URIs to imagery for this face, if this is a
    /// double-sided card. If this card is not double-sided, then the image_uris
    /// property will be part of the parent object instead.
    pub image_uris: Option<ImageUris>,

    /// This face’s loyalty, if any.
    pub loyalty: Option<String>,

    /// The mana cost for this face. This value will be any empty string "" if
    /// the cost is absent. Remember that per the game rules, a missing mana
    /// cost and a mana cost of `{0}` are different values.
    pub mana_cost: String,

    /// The name of this particular face.
    pub name: String,

    /// The Oracle ID of this particular face, if the card is reversible.
    pub oracle_id: Option<Uuid>,

    /// The Oracle text for this face, if any.
    pub oracle_text: Option<String>,

    /// This face’s power, if any. Note that some cards have powers that are not
    /// numeric, such as `*`.
    pub power: Option<String>,

    /// The localized name printed on this face, if any.
    pub printed_name: Option<String>,

    /// The localized text printed on this face, if any.
    pub printed_text: Option<String>,

    /// The type line as printed on the card.
    pub printed_type_line: Option<String>,

    /// This face’s toughness, if any. Note that some cards have powers that are not
    /// numeric, such as `*`.
    pub toughness: Option<String>,

    /// The type line of this particular face.
    pub type_line: Option<String>,

    /// The watermark on this particulary card face, if any.
    pub watermark: Option<String>,

    /// The ID of the illustrator of this card face. Newly spoiled cards may not have this field yet.
    pub artist_id: Option<Uuid>,

    /// The just-for-fun name printed on the card (such as for Godzilla series cards).
    pub flavor_name: Option<String>,

    /// This face’s defense, if it's a battle.
    pub defense: Option<String>,

    /// The layout of this card face, if the card is reversible.
    pub layout: Option<Layout>,

    #[cfg(test)]
    #[serde(rename = "object")]
    _object: String,
}
