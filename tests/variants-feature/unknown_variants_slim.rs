use scryfall::{
    card::{Finishes, FrameEffect, Layout, PromoType, SecurityStamp},
    set::SetType,
};

use static_assertions as sa;

sa::assert_impl_all!(FrameEffect: Copy);
sa::assert_impl_all!(Layout: Copy);
sa::assert_impl_all!(SetType: Copy);
sa::assert_impl_all!(PromoType: Copy);
sa::assert_impl_all!(SecurityStamp: Copy);

sa::assert_eq_size!(FrameEffect, u8);
sa::assert_eq_size!(Layout, u8);
sa::assert_eq_size!(SetType, u8);
sa::assert_eq_size!(PromoType, u8);
sa::assert_eq_size!(SecurityStamp, u8);

#[allow(dead_code)]
fn match_on_frame_effect(f: FrameEffect) {
    match f {
        FrameEffect::Legendary => todo!(),
        FrameEffect::Miracle => todo!(),
        FrameEffect::Nyxtouched => todo!(),
        FrameEffect::Draft => todo!(),
        FrameEffect::Devoid => todo!(),
        FrameEffect::Tombstone => todo!(),
        FrameEffect::Colorshifted => todo!(),
        FrameEffect::Inverted => todo!(),
        FrameEffect::SunMoonDfc => todo!(),
        FrameEffect::CompassLandDfc => todo!(),
        FrameEffect::OriginPwDfc => todo!(),
        FrameEffect::MoonEldraziDfc => todo!(),
        FrameEffect::WaxingAndWaningMoonDfc => todo!(),
        FrameEffect::Showcase => todo!(),
        FrameEffect::ExtendedArt => todo!(),
        FrameEffect::Companion => todo!(),
        FrameEffect::Etched => todo!(),
        FrameEffect::Snow => todo!(),
        FrameEffect::Lesson => todo!(),
        FrameEffect::ShatteredGlass => todo!(),
        FrameEffect::ConvertDfc => todo!(),
        FrameEffect::FanDfc => todo!(),
        FrameEffect::UpsideDownDfc => todo!(),
        FrameEffect::MoonReverseMoonDfc => todo!(),
        FrameEffect::Enchantment => todo!(),
        FrameEffect::FullArt => todo!(),
        FrameEffect::Nyxborn => todo!(),
        FrameEffect::Booster => todo!(),
        FrameEffect::Textless => todo!(),
        FrameEffect::StorySpotlight => todo!(),
        FrameEffect::Thick => todo!(),
        FrameEffect::Borderless => todo!(),
        FrameEffect::Vehicle => todo!(),
        FrameEffect::Spree => todo!(),
        FrameEffect::Unknown => todo!(),
    }
}
#[allow(dead_code)]
fn match_on_layout(f: Layout) {
    match f {
        Layout::Normal => todo!(),
        Layout::Split => todo!(),
        Layout::Flip => todo!(),
        Layout::Transform => todo!(),
        Layout::ModalDfc => todo!(),
        Layout::Meld => todo!(),
        Layout::Leveler => todo!(),
        Layout::Class => todo!(),
        Layout::Saga => todo!(),
        Layout::Adventure => todo!(),
        Layout::Planar => todo!(),
        Layout::Scheme => todo!(),
        Layout::Vanguard => todo!(),
        Layout::Token => todo!(),
        Layout::DoubleFacedToken => todo!(),
        Layout::Emblem => todo!(),
        Layout::Augment => todo!(),
        Layout::Host => todo!(),
        Layout::ArtSeries => todo!(),
        Layout::ReversibleCard => todo!(),
        Layout::Prototype => todo!(),
        Layout::Mutate => todo!(),
        Layout::Case => todo!(),
        Layout::Unknown => todo!(),
    }
}

#[allow(dead_code)]
fn match_on_set_type(f: SetType) {
    match f {
        SetType::Core => todo!(),
        SetType::Expansion => todo!(),
        SetType::Masters => todo!(),
        SetType::Masterpiece => todo!(),
        SetType::FromTheVault => todo!(),
        SetType::Spellbook => todo!(),
        SetType::PremiumDeck => todo!(),
        SetType::DuelDeck => todo!(),
        SetType::DraftInnovation => todo!(),
        SetType::TreasureChest => todo!(),
        SetType::Commander => todo!(),
        SetType::Planechase => todo!(),
        SetType::Archenemy => todo!(),
        SetType::Vanguard => todo!(),
        SetType::Funny => todo!(),
        SetType::Starter => todo!(),
        SetType::GiftBox => todo!(),
        SetType::Promo => todo!(),
        SetType::Token => todo!(),
        SetType::Memorabilia => todo!(),
        SetType::Alchemy => todo!(),
        SetType::Arsenal => todo!(),
        SetType::Minigame => todo!(),
        SetType::Unknown => todo!(),
    }
}

#[allow(dead_code)]
fn match_on_promo_type(f: PromoType) {
    match f {
        PromoType::Alchemy => todo!(),
        PromoType::Arenaleague => todo!(),
        PromoType::Beginnerbox => todo!(),
        PromoType::Boosterfun => todo!(),
        PromoType::Boxtopper => todo!(),
        PromoType::Brawldeck => todo!(),
        PromoType::Bringafriend => todo!(),
        PromoType::Bundle => todo!(),
        PromoType::Buyabox => todo!(),
        PromoType::Commanderparty => todo!(),
        PromoType::Concept => todo!(),
        PromoType::Confettifoil => todo!(),
        PromoType::Convention => todo!(),
        PromoType::Datestamped => todo!(),
        PromoType::Dossier => todo!(),
        PromoType::Doubleexposure => todo!(),
        PromoType::Doublerainbow => todo!(),
        PromoType::Draculaseries => todo!(),
        PromoType::Draftweekend => todo!(),
        PromoType::Duels => todo!(),
        PromoType::Embossed => todo!(),
        PromoType::Event => todo!(),
        PromoType::FirstPlaceFoil => todo!(),
        PromoType::Fnm => todo!(),
        PromoType::Fracturefoil => todo!(),
        PromoType::Galaxyfoil => todo!(),
        PromoType::Gameday => todo!(),
        PromoType::Giftbox => todo!(),
        PromoType::Gilded => todo!(),
        PromoType::Glossy => todo!(),
        PromoType::Godzillaseries => todo!(),
        PromoType::Halofoil => todo!(),
        PromoType::Imagine => todo!(),
        PromoType::Instore => todo!(),
        PromoType::Intropack => todo!(),
        PromoType::Invisibleink => todo!(),
        PromoType::Jpwalker => todo!(),
        PromoType::Judgegift => todo!(),
        PromoType::League => todo!(),
        PromoType::Magnified => todo!(),
        PromoType::Manafoil => todo!(),
        PromoType::Mediainsert => todo!(),
        PromoType::Moonlitland => todo!(),
        PromoType::Neonink => todo!(),
        PromoType::Oilslick => todo!(),
        PromoType::Openhouse => todo!(),
        PromoType::Planeswalkerdeck => todo!(),
        PromoType::Plastic => todo!(),
        PromoType::Playerrewards => todo!(),
        PromoType::Playpromo => todo!(),
        PromoType::Playtest => todo!(),
        PromoType::Portrait => todo!(),
        PromoType::Poster => todo!(),
        PromoType::Premiereshop => todo!(),
        PromoType::Prerelease => todo!(),
        PromoType::Promopack => todo!(),
        PromoType::Rainbowfoil => todo!(),
        PromoType::Raisedfoil => todo!(),
        PromoType::Ravnicacity => todo!(),
        PromoType::Rebalanced => todo!(),
        PromoType::Release => todo!(),
        PromoType::Resale => todo!(),
        PromoType::Ripplefoil => todo!(),
        PromoType::Schinesealtart => todo!(),
        PromoType::Scroll => todo!(),
        PromoType::Serialized => todo!(),
        PromoType::Setextension => todo!(),
        PromoType::Setpromo => todo!(),
        PromoType::Silverfoil => todo!(),
        PromoType::Sldbonus => todo!(),
        PromoType::Stamped => todo!(),
        PromoType::Startercollection => todo!(),
        PromoType::Starterdeck => todo!(),
        PromoType::Stepandcompleat => todo!(),
        PromoType::Storechampionship => todo!(),
        PromoType::Surgefoil => todo!(),
        PromoType::Textured => todo!(),
        PromoType::Themepack => todo!(),
        PromoType::Thick => todo!(),
        PromoType::Tourney => todo!(),
        PromoType::Unknown => todo!(),
        PromoType::UpsideDown => todo!(),
        PromoType::UpsideDownBack => todo!(),
        PromoType::Vault => todo!(),
        PromoType::Wizardsplaynetwork => todo!(),
    }
}

#[allow(dead_code)]
fn match_on_security_stamp(s: SecurityStamp) {
    match s {
        SecurityStamp::Oval => todo!(),
        SecurityStamp::Triangle => todo!(),
        SecurityStamp::Acorn => todo!(),
        SecurityStamp::Circle => todo!(),
        SecurityStamp::Arena => todo!(),
        SecurityStamp::Heart => todo!(),
        SecurityStamp::Unknown => todo!(),
    }
}

#[allow(dead_code)]
fn match_on_finishes(f: Finishes) {
    match f {
        Finishes::Nonfoil => todo!(),
        Finishes::Foil => todo!(),
        Finishes::Etched => todo!(),
        Finishes::Unknown => todo!(),
    }
}

#[test]
fn deserialize() {
    assert_eq!(
        serde_json::from_str::<FrameEffect>(r#""frontier""#).unwrap(),
        FrameEffect::Unknown,
    );
    assert_eq!(
        serde_json::from_str::<Layout>(r#""frontier""#).unwrap(),
        Layout::Unknown,
    );
    assert_eq!(
        serde_json::from_str::<SetType>(r#""frontier""#).unwrap(),
        SetType::Unknown,
    );
    assert_eq!(
        serde_json::from_str::<PromoType>(r#""frontier""#).unwrap(),
        PromoType::Unknown,
    );
    assert_eq!(
        serde_json::from_str::<SecurityStamp>(r#""frontier""#).unwrap(),
        SecurityStamp::Unknown,
    );
    assert_eq!(
        serde_json::from_str::<Finishes>(r#""foo""#).unwrap(),
        Finishes::Unknown,
    );
}
