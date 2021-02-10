//! Sub card object used when a magic card has more then one card face.
//!
//! For documentation about the fields, please refer to the official scryfall
//! [documentation](https://scryfall.com/docs/api/cards)
use serde::{Deserialize, Serialize};

use crate::card::Colour;
use crate::util::Uuid;

use std::collections::HashMap;

/// Sub card object used when a magic card has more then one card face.
///
/// For documentation about the fields, please refer to the official scryfall
/// [documentation](https://scryfall.com/docs/api/cards)
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct CardFace {
    pub artist: Option<String>,
    pub color_indicator: Option<Vec<Colour>>,
    #[serde(default)]
    pub colors: Vec<Colour>,
    pub flavor_text: Option<String>,
    pub illustration_id: Option<Uuid>,
    pub image_uris: Option<HashMap<String, String>>,
    pub loyalty: Option<String>,
    pub mana_cost: String,
    pub name: String,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub toughness: Option<String>,
    pub type_line: Option<String>,
    pub watermark: Option<String>,
}
