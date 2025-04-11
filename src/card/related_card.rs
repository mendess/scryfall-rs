use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::card::Card;
use crate::uri::Uri;

/// Cards that are closely related to other cards (because they call them by
/// name, or generate a token, or meld, etc) have a `all_parts` property that
/// contains Related Card objects.
///
/// For more information, refer to the [official docs](https://scryfall.com/api/cards#related-card-objects).
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct RelatedCard {
    /// An unique ID for this card in Scryfall’s database.
    pub id: Uuid,

    /// A content type for this object, always related_card.
    pub component: Component,

    /// A field explaining what role this card plays in this relationship.
    pub name: String,

    /// The name of this particular related card.
    pub type_line: String,

    /// The name of this particular related card.
    pub uri: Uri<Card>,

    #[cfg(test)]
    #[serde(rename = "object")]
    _object: String,
}

/// The kind of related card.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Component {
    Token,
    MeldPart,
    MeldResult,
    ComboPiece,
}
