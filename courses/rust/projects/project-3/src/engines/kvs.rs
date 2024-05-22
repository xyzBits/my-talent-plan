use std::io::{BufReader, Read, Seek};
use std::ops::Range;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::Result;

use super::KvsEngine;

/// The `KvStore` store string key/value pairs.
///
/// Key/value pair are persisted to disk in log file. Log files are name after
/// monotonically increasing generation numbers with a `log` extension name.
/// A `BTreeMap` in memory stores the keys and the value locations for fast query.
///
/// ```rust
/// # use kvs::{KvsEngine, KvStore, Result};
/// # fn try_main() -> Result<()> {
/// use std::env::current_dir;
/// use kvs::KvsEngine;
/// let mut store = KvStore::open(current_dir()?)?;
/// store.set("key".to_owned(), "value".to_owned())?;
/// let val = store.get("key".to_owned())?;
/// assert_eq!(val, Some("value".to_owned()));
/// # Ok(())
/// # }
/// ```
pub struct KvStore {
    path: PathBuf,

}

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

/// Struct representing a command
#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },

    Remove { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }

    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}

/// Represents the position and length of a json-serialized command in the log file
struct CommandPos {
    gen: u64, // file number
    pos: u64, // start byte position
    len: u64, // length of the bytes
}

impl From<(u64, Range<u64>)> for CommandPos {
    fn from((gen, range): (u64, Range<u64>)) -> Self {
        CommandPos {
            gen,
            pos: range.start,
            len: range.end - range.start,
        }
    }
}


struct BufReaderWitPos<R: Seek + Read> {
    reader: BufReader<R>,
    pos: u64,
}

