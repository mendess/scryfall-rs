use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "lowercase")]
pub struct RelatedUris {
    pub gatherer: Option<Url>,
    #[serde(rename = "tcgplayer_infinite_articles")]
    pub tcg_player_infinite_articles: Option<Url>,
    #[serde(rename = "tcgplayer_infinite_decks")]
    pub tcg_player_infinite_decks: Option<Url>,
    pub edhrec: Option<Url>,
}
