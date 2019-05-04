use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SetType {
    Core,
    Expansion,
    Masters,
    Masterpiece,
    FromTheVault,
    Spellbook,
    PremiumDeck,
    DuelDeck,
    DraftInnovation,
    TreasureChest,
    Commander,
    Planechase,
    Archenemy,
    Vanguard,
    Funny,
    Starter,
    #[serde(rename = "box")]
    GiftBox,
    Promo,
    Token,
    Memorabilia,
}