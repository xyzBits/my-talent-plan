use sled::Db;

use super::KvsEngine;
use crate::{KvsError, Result};

/// Wrapper of `sled::Db`
#[derive(Clone)]
pub struct SledKvsEngine(Db);

impl SledKvsEngine {
    /// Creates a `SledKvsEngine` from `sled::Db`
    pub fn new(db: Db) -> Self {
        SledKvsEngine(db)
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(None)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        Ok(())
    }
}
