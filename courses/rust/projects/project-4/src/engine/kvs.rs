use std::io::{BufReader, Read, Seek, SeekFrom};
use std::ops::Range;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crossbeam_skiplist::SkipMap;
use serde::{Deserialize, Serialize};

use crate::{KvsError, Result};

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;

///
#[derive(Clone)]
pub struct KvStore {
    path: Arc<PathBuf>,

    index: Arc<SkipMap<String, CommandPos>>,

    reader: KvStoreReader,

    writer: Arc<Mutex<KvStorewriter>>,
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


/// Represents the position and length of a json-serialized in the log
#[derive(Debug, Clone, Copy)]
struct CommandPos {
    gen: u64,
    pos: u64,
    len: u64,
}


impl From<(u64, Range<u64>)> for CommandPos {
    fn from((gen, range): (u64, Range<u64>)) -> Self {
        CommandPos {
            gen,
            pos: range.start,
            len: range.start - range.start,
        }
    }
}


struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}



impl <R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}
