# scryfall-rs

A wrapper around the scryfall magic the gathering API

[![Crates.io](https://img.shields.io/crates/v/scryfall.svg)](https://crates.io/crates/scryfall)
[![Documentation](https://docs.rs/scryfall/badge.svg)](https://docs.rs/scryfall)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://github.com/mendess/scryfall-rs/actions/workflows/rust.yml/badge.svg)

It wraps the scryfall API as close to it as possible and I try to keep it up to
date


## Cards

The main way to fetch cards from this API is the `Card` struct.

This allows you to get cards from `scryfall` using all of their available
REST Apis

```rust
use scryfall::card::Card;
match Card::named_fuzzy("Light Bolt") {
    Ok(card) => assert_eq!(card.name, "Lightning Bolt"),
    Err(e) => panic!(format!("{:?}", e))
}
```

## Sets

You can also fetch information about a card set.

The available routes for this can be seen on `Set`

```rust
use scryfall::set::Set;
assert_eq!(Set::code("mmq").unwrap().name, "Mercadian Masques")
```

## Dealing with breaking changes

Scryfall makes a lot of breaking api changes, mostly because magic makes a lot
of breaking changes 😅. Due to the strong typing of this crate, this means that
sometimes code that works one day breaks the next day. For example, there's a
[`PromoType`][promo-type-enum] enum. This enum, when deserializing, will strictly
reject any format it doesn't know about. This means that everytime wizards adds
a new format, scryfall will start returning this new format from its API
which will make your code fail at runtime.

To cope with this I've added a feature called `unknown_variants`. This feature
adds to these troublesome enums a variant called [`Unknown`][promo-type-unknown], which contains the
string representation of the unknown format.

This has a few pros and cons:

- Pros:
  - Your code is much less likely to stop working from one day to the next.
  - You can exhaustively match on the enum
- Cons:
  - The size of the enum is now 24 bytes, instead of 1
  - It is no longer Copy
  - If you ever depend on a variant being passed through the unknown variant,
      when the new variant is added to the enum, it will stop showing up in the
      unknown variant. For example, if tomorrow wizards adds a promo type called
      "Transparent" and you have `unknown_variants` enabled, `"transparent"` will
      start showing up inside the [`PromoType::Unknown`][promo-type-unknown] variant. But in the next
      version of this crate, I will add `PromoType::Transparent`, which means that if
      you upgrade your dependency on this crate, `"transparent"` will no longer
      show up inside the [`PromoType::Unknown`][format-unknown] variant. If you depend on that
      behaviour it will be considered a breaking change.

If you want to have the unknown variant but don't want to pay for the 24 byte
cost, you can opt for the `unknown_variants_slim` feature, which will simply add
an empty `Unknown` variant instead.

These two features are incompatible and `unknown_variants` will take
precedence if both are present.

[promo-type-enum]: https://docs.rs/scryfall/latest/scryfall/card/enum.PromoType.html
[promo-type-unknown]: https://docs.rs/scryfall/latest/scryfall/card/enum.PromoType.html#variant.Unknown
