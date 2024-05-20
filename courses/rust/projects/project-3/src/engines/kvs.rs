use std::path::PathBuf;

use super::KvsEngine;
use crate::Result;

pub struct KvStore {}

impl KvStore {
    pub fn open<T: Into<PathBuf>>(path: T) -> Result<KvStore> {
        Ok(KvStore {})
    }
}

impl KvsEngine for KvStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        todo!()
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        todo!()
    }

    fn remove(&mut self, key: String) -> Result<()> {
        todo!()
    }
}
