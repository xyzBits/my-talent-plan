use std::convert::TryInto;
use crate::{Result};


pub mod kvs;
pub mod sled;

/// Trait for a key value store engine.
pub trait KvsEngine: Clone + Send + 'static {
    /// Sets the value of a string key to a string
    ///
    /// If the key already exists, the previous value will be overwritten.
    fn set(&self, key: String, value: String) -> Result<()>;

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    fn get(&self, key: String) -> Result<Option<String>>;

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    fn remove(&self, key: String) -> Result<()>;
}