//! Module defining a related card.
//!
//! Cards that are closely related to other cards (because they call them by name, or generate a
//! token, or meld, etc) have a all_parts property that contains `RelatedCard` objects.
use serde::Deserialize;

use crate::card::Card;
use crate::util::{uri::URI, UUID};

/// Related card object. Refer to the official [docs](https://scryfall.com/docs/api/cards)
/// for information on the fields.
#[derive(Debug, Deserialize, Clone)]
#[allow(missing_docs)]
pub struct RelatedCard {
    pub id: UUID,
    pub component: Component,
    pub name: String,
    pub type_line: String,
    pub uri: URI<Card>,
}

/// The kind of related card.
#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Component {
    Token,
    MeldPart,
    MeldResult,
    ComboPiece,
}
