//! [Scryfall](https://scryfall.com) provides a REST-like API for ingesting our card data
//! programatically. The API exposes information available on the regular site in easy-to-consume
//! formats.
pub mod card;
pub mod catalog;
pub mod error;
pub mod ruling;
pub mod set;
pub mod util;

pub use error::Result;

#[cfg(tests)]
mod tests {}
