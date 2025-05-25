use serde::{Deserialize, Serialize};


/// Enum defining the types of marketplace URIs available for purchasing cards.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "lowercase")]
pub enum PurchaseUris {
    TcgPlayer,
    CardMarket,
    CardHoarder,
}

impl std::fmt::Display for PurchaseUris {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PurchaseUris::*;
        write!(
            f,
            "{}",
            match self {
                TcgPlayer => "tcgplayer",
                CardMarket => "cardmarket",
                CardHoarder => "cardhoarder",
            }
        )
    }
}