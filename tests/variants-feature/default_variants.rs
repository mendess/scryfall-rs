use scryfall::{
    card::{FrameEffect, Layout, PromoType, SecurityStamp},
    format::Format,
    set::SetType,
};

use static_assertions as sa;

sa::assert_impl_all!(Format: Copy);
sa::assert_impl_all!(FrameEffect: Copy);
sa::assert_impl_all!(Layout: Copy);
sa::assert_impl_all!(SetType: Copy);
sa::assert_impl_all!(PromoType: Copy);
sa::assert_impl_all!(SecurityStamp: Copy);

sa::assert_eq_size!(Format, u8);
sa::assert_eq_size!(FrameEffect, u8);
sa::assert_eq_size!(Layout, u8);
sa::assert_eq_size!(SetType, u8);
sa::assert_eq_size!(PromoType, u8);
sa::assert_eq_size!(SecurityStamp, u8);
