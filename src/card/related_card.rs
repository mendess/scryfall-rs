//! Module defining a related card.
//!
//! Cards that are closely related to other cards (because they call them by name, or generate a
//! token, or meld, etc) have a all_parts property that contains `RelatedCard` objects.
use serde::{Deserialize, Serialize};

use crate::card::Card;
use crate::util::{uri::URI, Uuid};

/// Related card object. Refer to the official [docs](https://scryfall.com/docs/api/cards)
/// for information on the fields.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct RelatedCard {
    pub id: Uuid,
    pub component: Component,
    pub name: String,
    pub type_line: String,
    pub uri: URI<Card>,
}

/// The kind of related card.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
#[non_exhaustive]
pub enum Component {
    Token,
    MeldPart,
    MeldResult,
    ComboPiece,
}
