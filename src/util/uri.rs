//! Module for handling unresolved URLs returned by the scryfall api
//!
//! Some fields of the scryfall api have URLs referring to queries that can be
//! run to obtain more information. This module abstracts the work of fetching
//! that data.
use std::marker::PhantomData;

use itertools::Itertools;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use ureq::{Agent, Error as UreqError};
use url::Url;

use crate::error::Error;
use crate::list::{List, ListIter};

thread_local!(static CLIENT: Agent = Agent::new());

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(transparent)]
pub struct Uri<T> {
    url: Url,
    _marker: PhantomData<fn() -> T>,
}

impl<T: DeserializeOwned> From<&str> for Uri<T> {
    fn from(url: &str) -> Self {
        Uri::from(Url::parse(url).unwrap())
    }
}

impl<T: DeserializeOwned> From<Url> for Uri<T> {
    fn from(url: Url) -> Self {
        Uri {
            url,
            _marker: PhantomData,
        }
    }
}

impl<T: DeserializeOwned> Uri<T> {
    pub fn fetch(&self) -> crate::Result<T> {
        let response = CLIENT.with(|client| client.request_url("GET", &self.url).call());
        match response {
            Ok(response) => match response.status() {
                200..=299 => Ok(serde_json::from_reader(response.into_reader())?),
                status => Err(Error::HttpError(status, response.status_text().to_string())),
            },
            Err(UreqError::Status(400..=499, response)) => Err(Error::ScryfallError(
                serde_json::from_reader(response.into_reader())?,
            )),
            Err(error) => Err(Error::UreqError(error, self.url.to_string())),
        }
    }
}

impl<T: DeserializeOwned> Uri<List<T>> {
    pub fn fetch_iter(&self) -> crate::Result<ListIter<T>> {
        Ok(self.fetch()?.into_iter())
    }

    pub fn fetch_all(&self) -> crate::Result<Vec<T>> {
        self.fetch_iter()?.try_collect()
    }
}
