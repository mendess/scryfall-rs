//! Scryfall provides an overall categorization for each Set in the set_type
//! property.
use std::fmt;

use serde::{Deserialize, Serialize};

/// Scryfall provides an overall categorization for each Set in the set_type
/// property.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(not(feature = "unknown_variants"), derive(Copy))]
#[cfg_attr(
    all(
        not(feature = "unknown_variants"),
        not(feature = "unknown_variants_slim")
    ),
    non_exhaustive
)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "snake_case")]
pub enum SetType {
    /// A yearly Magic core set (Tenth Edition, etc)
    Core,
    ///  A set of new cards that only get added to high-power formats
    Eternal,
    /// A rotational expansion set in a block (Zendikar, etc)
    Expansion,
    /// A reprint set that contains no new cards (Modern Masters, etc)
    Masters,
    /// Masterpiece Series premium foil cards
    Masterpiece,
    /// From the Vault gift sets
    FromTheVault,
    /// Spellbook series gift sets
    Spellbook,
    /// Premium Deck Series decks
    PremiumDeck,
    /// Duel Decks
    DuelDeck,
    /// Special draft sets, like Conspiracy and Battlebond
    DraftInnovation,
    /// Magic Online treasure chest prize sets
    TreasureChest,
    /// Commander preconstructed decks
    Commander,
    /// Planechase sets
    Planechase,
    /// Archenemy sets
    Archenemy,
    /// Vanguard card sets
    Vanguard,
    /// A funny un-set or set with funny promos (Unglued, Happy Holidays, etc)
    Funny,
    /// A starter/introductory set (Portal, etc)
    Starter,
    /// A gift box set
    #[serde(rename = "box")]
    GiftBox,
    /// A set that contains purely promotional cards
    Promo,
    /// A set made up of tokens and emblems.
    Token,
    /// A set made up of gold-bordered, oversize, or trophy cards that are not
    /// legal
    Memorabilia,
    /// Alchemy sets
    Alchemy,
    /// Arsenal sets
    Arsenal,
    /// Mini game sets
    Minigame,
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "unknown_variants", feature = "unknown_variants_slim")))
    )]
    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    /// Unknown set type
    Unknown(Box<str>),
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "unknown_variants", feature = "unknown_variants_slim")))
    )]
    #[cfg(all(not(feature = "unknown_variants"), feature = "unknown_variants_slim"))]
    #[serde(other)]
    /// Unknown set type
    Unknown,
}

impl fmt::Display for SetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SetType::Core => "core",
                SetType::Eternal => "eternal",
                SetType::Expansion => "expansion",
                SetType::Masters => "masters",
                SetType::Masterpiece => "masterpiece",
                SetType::FromTheVault => "from_the_vault",
                SetType::Spellbook => "spellbook",
                SetType::PremiumDeck => "premium_deck",
                SetType::DuelDeck => "duel_deck",
                SetType::DraftInnovation => "draft_innovation",
                SetType::TreasureChest => "treasure_chest",
                SetType::Commander => "commander",
                SetType::Planechase => "planechase",
                SetType::Archenemy => "archenemy",
                SetType::Vanguard => "vanguard",
                SetType::Funny => "funny",
                SetType::Starter => "starter",
                SetType::GiftBox => "gift_box",
                SetType::Promo => "promo",
                SetType::Token => "token",
                SetType::Memorabilia => "memorabilia",
                SetType::Alchemy => "alchemy",
                SetType::Arsenal => "arsenal",
                SetType::Minigame => "minigame",
                #[cfg(feature = "unknown_variants")]
                SetType::Unknown(s) => s,
                #[cfg(all(not(feature = "unknown_variants"), feature = "unknown_variants_slim"))]
                SetType::Unknown => "unknown-set-type",
            }
        )
    }
}
