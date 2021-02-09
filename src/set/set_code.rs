//! This module defines a set code.
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use std::convert::AsRef;
use std::convert::TryFrom;
use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::str;

/// A 3 or 4 letter set code, like 'war' for 'War of the Spark'.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SetCode(CodeInner);

#[allow(dead_code)]
impl SetCode {
    /// Creates a set code from a str.
    ///
    /// Valid set codes are ascii and 3 our 6 letters long. If any of these conditions
    /// fails, the conversion fails.
    ///
    /// The error value is None if the `str` was no ascii, otherwise it holds the size
    /// of the `str`.
    ///
    /// ```rust
    /// use scryfall::set::SetCode;
    ///
    /// assert_eq!(SetCode::new("war").unwrap().as_ref(), "war")
    /// ```
    pub fn new(code: &str) -> Result<Self, Option<usize>> {
        SetCode::try_from(code)
    }

    /// Returns a reference to the inner set code.
    pub fn get(&self) -> &str {
        // The inner code is always a valid utf8 str since it can
        // only be created from a valid &str.
        unsafe { str::from_utf8_unchecked(self.0.get()) }
    }
}

impl TryFrom<&str> for SetCode {
    type Error = Option<usize>;
    /// See [`new`](#method.new) for documentation on why this might return an `Err`.
    fn try_from(code: &str) -> Result<Self, Option<usize>> {
        if !code.is_ascii() {
            return Err(None);
        }
        let code = code.as_bytes();
        Ok(SetCode(match code.len() {
            3 => CodeInner::Code3(<[u8; 3]>::try_from(code).unwrap()),
            4 => CodeInner::Code4(<[u8; 4]>::try_from(code).unwrap()),
            5 => CodeInner::Code5(<[u8; 5]>::try_from(code).unwrap()),
            6 => CodeInner::Code6(<[u8; 6]>::try_from(code).unwrap()),
            invalid => return Err(Some(invalid)),
        }))
    }
}

impl AsRef<str> for SetCode {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

#[derive(Default)]
struct SetCodeVisior {
    size: Option<usize>,
}

impl<'de> Visitor<'de> for SetCodeVisior {
    type Value = SetCode;

    fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        match self.size {
            Some(size) => write!(f, "set code size between 3 and 6, found {}", size),
            None => write!(f, "set code to be ascii"),
        }
    }

    fn visit_str<E>(mut self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        SetCode::try_from(s).map_err(|size| {
            self.size = size;
            de::Error::invalid_value(de::Unexpected::Str(s), &self)
        })
    }
}

impl<'de> Deserialize<'de> for SetCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SetCodeVisior::default())
    }
}

impl Serialize for SetCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(str::from_utf8(self.0.get()).unwrap())
    }
}

impl Display for SetCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[allow(clippy::enum_variant_names)]
enum CodeInner {
    Code3([u8; 3]),
    Code4([u8; 4]),
    Code5([u8; 5]),
    Code6([u8; 6]),
}

impl PartialOrd for CodeInner {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get().partial_cmp(other.get())
    }
}

impl Ord for CodeInner {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl CodeInner {
    fn get(&self) -> &[u8] {
        use CodeInner::*;
        match self {
            Code3(c) => &c[..],
            Code4(c) => &c[..],
            Code5(c) => &c[..],
            Code6(c) => &c[..],
        }
    }
}
