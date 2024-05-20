use std::io;
use std::io::Error;
use std::string::FromUtf8Error;

use failure::Fail;

/// Error type for kvs
#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO Error
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),

    /// Serialization or deserialization error
    #[fail(display = "serde_json error: {}", _0)]
    Serde(#[cause] serde_json::Error),

    /// Removing non-existent key error
    #[fail(display = "Key not found")]
    KeyNotFound,

    /// Unexpected command type error.
    /// It indicated a corrupted log or program bug.
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,

    /// Key or value is invalid UTF-8 sequence
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[cause] FromUtf8Error),

    /// Sled error
    #[fail(display = "sled error: {}", _0)]
    Sled(#[cause] sled::Error),

    /// Error with a string message
    #[fail(display = "{}", _0)]
    StringError(String),
}

impl From<io::Error> for KvsError {
    fn from(value: Error) -> Self {
        KvsError::Io(value)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(value: serde_json::Error) -> Self {
        KvsError::Serde(value)
    }
}

impl From<FromUtf8Error> for KvsError {
    fn from(value: FromUtf8Error) -> Self {
        KvsError::Utf8(value)
    }
}

impl From<sled::Error> for KvsError {
    fn from(value: sled::Error) -> Self {
        KvsError::Sled(value)
    }
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;

#[test]
fn test_cause() {}
