# scryfall-rs

A wrapper around the scryfall magic the gathering API

[![Crates.io](https://img.shields.io/crates/v/scryfall.svg)](https://crates.io/crates/scryfall)
[![Documentation](https://docs.rs/scryfall/badge.svg)](https://docs.rs/scryfall)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://github.com/mendess/scryfall-rs/workflows/Rust/badge.svg)

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

