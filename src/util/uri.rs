use crate::card::{CardError, CardResult};
use std::marker::PhantomData;

use serde::Deserialize;
use serde_json::from_reader;

#[derive(Debug, Deserialize)]
pub struct URI<T>(String, PhantomData<T>);

impl<T> From<String> for URI<T> {
    fn from(s: String) -> Self {
        URI(s, PhantomData)
    }
}

impl<T> From<URI<T>> for String {
    fn from(s: URI<T>) -> Self {
        s.0
    }
}

impl<T> URI<T> {
    fn fetch(&self) -> CardResult<T>
    where
        for<'de> T: Deserialize<'de>,
    {
        url_fetch(&self.0)
    }
}

pub fn url_fetch<T>(url: &str) -> CardResult<T>
where
    for<'de> T: Deserialize<'de>,
{
    let resp = reqwest::get(url)?;
    if resp.status().is_success() {
        Ok(from_reader(resp)?)
    } else {
        Err(CardError::Other(format!("{:?}", resp.status())))
    }
}
