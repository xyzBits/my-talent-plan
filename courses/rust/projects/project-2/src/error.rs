use std::io;
use std::io::{Error, ErrorKind};

use failure::Fail;

/// Error type for kvs
#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO error.
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    /// Serialization or deserialization error.
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    /// Removing non-existent key error.
    #[fail(display = "Key not found")]
    KeyNotFound,

    /// Unexpected command type error.
    /// It indicated a corrupted log or a program bug
    #[fail(display = "Unexpected command")]
    UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
    fn from(err: Error) -> Self {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> Self {
        KvsError::Serde(err)
    }
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;

#[cfg(test)]
mod error_test {
    #[test]
    fn test_from_trait() {
        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl From<i32> for Number {
            fn from(value: i32) -> Self {
                Number { value }
            }
        }
        let x = 5;
        let num = Number::from(x);

        println!("{:?}", num);
    }

    #[test]
    fn test_into_trait() {
        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl Into<Number> for i32 {
            fn into(self) -> Number {
                Number { value: self }
            }
        }

        let x = 5;
        let number: Number = x.into();
        println!("{:?}", number);
    }
}

#[test]
fn test_kvs_error_from() {
    let addr_in_use_error = Error::new(ErrorKind::AddrInUse, "addr in use");

    let kvs_error = KvsError::from(addr_in_use_error);

    println!("{}", kvs_error);
}

#[test]
fn test_kvs_error_into() {
    let invalid_data_error = Error::new(ErrorKind::InvalidData, "invalid data");

    let kvs_error: KvsError = invalid_data_error.into();

    println!("{}", kvs_error);
}
