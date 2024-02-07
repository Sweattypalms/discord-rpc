use std::array::TryFromSliceError;
use std::string::FromUtf8Error;

/// Error type for the library.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error("{0}")]
    DiscordConnError(String),
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    TryFromSlice(#[from] TryFromSliceError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}