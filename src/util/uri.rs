use crate::error::Error;
use std::marker::PhantomData;

use serde::Deserialize;
use serde_json::from_reader;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
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
    pub fn fetch(&self) -> crate::Result<T>
    where
        for<'de> T: Deserialize<'de>,
    {
        url_fetch(&self.0)
    }
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct PaginatedURI<T> {
    next: Option<URI<JsonParser<T>>>,
}

impl<T> PaginatedURI<T>
where
    for<'de> T: Deserialize<'de>,
{
    pub fn new(url: URI<T>) -> Self {
        PaginatedURI {
            next: Some(URI(url.0, PhantomData)),
        }
    }
}

#[derive(Deserialize, Debug)]
struct JsonParser<T> {
    next_page: Option<URI<JsonParser<T>>>,
    data: Vec<T>,
}

impl<T> Iterator for PaginatedURI<T>
where
    for<'de> T: Deserialize<'de>,
{
    type Item = crate::Result<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(url) = self.next.take() {
            match url_fetch::<JsonParser<T>>(&String::from(url)) {
                Ok(cards) => {
                    *self = PaginatedURI {
                        next: cards.next_page,
                    };
                    Some(Ok(cards.data))
                }
                Err(error) => Some(Err(error)),
            }
        } else {
            None
        }
    }
}

pub fn url_fetch<T>(url: &str) -> crate::Result<T>
where
    for<'de> T: Deserialize<'de>,
{
    let resp = reqwest::get(url)?;
    if resp.status().is_success() {
        Ok(from_reader(resp)?)
    } else {
        Err(Error::Other(format!("{:?}", resp.status())))
    }
}
