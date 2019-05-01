pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    JsonError(serde_json::Error),
    ReqwestError(reqwest::Error),
    Other(String),
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::JsonError(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}
