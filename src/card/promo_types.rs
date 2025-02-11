use serde::{Deserialize, Serialize};

/// The finish the card can come in.
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(not(feature = "unknown_variants"), derive(Copy))]
#[cfg_attr(
    all(
        not(feature = "unknown_variants"),
        not(feature = "unknown_variants_slim")
    ),
    non_exhaustive
)]
#[cfg_attr(test, serde(deny_unknown_fields))]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum PromoType {
    Alchemy,
    Arenaleague,
    Beginnerbox,
    Boosterfun,
    Boxtopper,
    Brawldeck,
    Bringafriend,
    Bundle,
    Buyabox,
    Commanderparty,
    Concept,
    Confettifoil,
    Convention,
    Datestamped,
    Dossier,
    Doubleexposure,
    Doublerainbow,
    Draculaseries,
    Draftweekend,
    Duels,
    Embossed,
    Event,
    FirstPlaceFoil,
    Fnm,
    Fracturefoil,
    Galaxyfoil,
    Gameday,
    Giftbox,
    Gilded,
    Glossy,
    Godzillaseries,
    Halofoil,
    Imagine,
    Instore,
    Intropack,
    Invisibleink,
    Jpwalker,
    Judgegift,
    League,
    Magnified,
    Manafoil,
    Mediainsert,
    Moonlitland,
    Neonink,
    Oilslick,
    Openhouse,
    Planeswalkerdeck,
    Plastic,
    Playerrewards,
    Playpromo,
    Playtest,
    Portrait,
    Poster,
    Premiereshop,
    Prerelease,
    Promopack,
    Rainbowfoil,
    Raisedfoil,
    Ravnicacity,
    Rebalanced,
    Release,
    Resale,
    Ripplefoil,
    Schinesealtart,
    Scroll,
    Serialized,
    Setextension,
    Setpromo,
    Silverfoil,
    Sldbonus,
    Stamped,
    Startercollection,
    Starterdeck,
    Stepandcompleat,
    Storechampionship,
    Surgefoil,
    Textured,
    Themepack,
    Thick,
    Tourney,
    UpsideDown,
    UpsideDownBack,
    Vault,
    Wizardsplaynetwork,
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "unknown_variants", feature = "unknown_variants_slim")))
    )]
    #[cfg(feature = "unknown_variants")]
    #[serde(untagged)]
    /// Unknown variant
    Unknown(Box<str>),
    #[cfg_attr(
        docsrs,
        doc(cfg(any(feature = "unknown_variants", feature = "unknown_variants_slim")))
    )]
    #[cfg(all(not(feature = "unknown_variants"), feature = "unknown_variants_slim"))]
    #[serde(other)]
    Unknown,
}
