use serde::{Deserialize, Serialize};
use url::Url;

/// Enum defining the types of marketplace URIs available for purchasing cards.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub struct PurchaseUris {
    #[serde(rename = "tcgplayer")]
    pub tcg_player: Option<Url>,
    #[serde(rename = "cardmarket")]
    pub card_market: Option<Url>,
    #[serde(rename = "cardhoarder")]
    pub card_hoarder: Option<Url>,
}
