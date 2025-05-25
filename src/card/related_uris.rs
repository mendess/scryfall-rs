use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "lowercase")]
pub enum RelatedUris {
    Gatherer,
    #[serde(rename = "tcgplayer_infinite_articles")]
    TcgPlayerInfiniteArticles,
    #[serde(rename = "tcgplayer_infinite_decks")]
    TcgPlayerInfiniteDecks,
    EdhRec,
}

impl std::fmt::Display for RelatedUris {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use RelatedUris::*;
        write!(
            f,
            "{}",
            match self {
                Gatherer => "gatherer",
                TcgPlayerInfiniteArticles => "tcgplayer_infinite_articles",
                TcgPlayerInfiniteDecks => "tcgplayer_infinite_decks",
                EdhRec => "edhrec",
            }
        )
    }
}