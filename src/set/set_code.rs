//! This module defines a set code.
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

use std::convert::AsRef;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::str;

/// A 3 or 4 letter set code, like 'war' for 'War of the Spark'.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SetCode(CodeInner);

#[allow(dead_code)]
impl SetCode {
    /// Creates a set code from a str.
    ///
    /// Valid set codes are ascii and 3 our 4 letters long. If any of these conditions
    /// fails, the conversion fails.
    ///
    /// ```rust
    /// use scryfall::set::SetCode;
    ///
    /// assert_eq!(SetCode::new("war").unwrap().as_ref(), "war")
    /// ```
    pub fn new(code: &str) -> Result<Self, ()> {
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
    type Error = ();
    /// See [`new`](#method.new) for documentation on why this might return an `Err`.
    fn try_from(code: &str) -> Result<Self, ()> {
        if !code.is_ascii() {
            return Err(());
        }
        let code = code.as_bytes();
        Ok(SetCode(match code.len() {
            3 => CodeInner::Code3(<[u8; 3]>::try_from(code).unwrap()),
            4 => CodeInner::Code4(<[u8; 4]>::try_from(code).unwrap()),
            _ => return Err(()),
        }))
    }
}

impl AsRef<str> for SetCode {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

struct SetCodeVisior;

impl<'de> Visitor<'de> for SetCodeVisior {
    type Value = SetCode;

    fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "invalid set code")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        SetCode::try_from(s).map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &self))
    }
}

impl<'de> Deserialize<'de> for SetCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SetCodeVisior)
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

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum CodeInner {
    Code3([u8; 3]),
    Code4([u8; 4]),
}

impl CodeInner {
    fn get(&self) -> &[u8] {
        use CodeInner::*;
        match self {
            Code3(c) => &c[..],
            Code4(c) => &c[..],
        }
    }
}
