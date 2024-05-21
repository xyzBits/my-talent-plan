//! This module provides various key value storage engines.

// import the module named Result from the current crate. The crate keyword is used
// to refer to the current crate or package in Rust.
use crate::Result;

/// Trait for a key value storage engine.
pub trait KvsEngine {
    /// Sets the value of a string key to a string
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// Remove a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    fn remove(&mut self, key: String) -> Result<()>;
}

mod kvs;
mod sled;

// put use keyword is used to make the following item publicly available for use
// outside of the module where it is defined
// self reference the current module where the code is located.
pub use self::kvs::KvStore;
pub use self::sled::SledKvsEngine;
