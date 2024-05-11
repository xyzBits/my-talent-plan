use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::{KvsError, Result};// 使用其他 lib 中的 crate，一定要用 crate:: 声明
const COMPACTION_THRESHOLD: u64 = 1024 * 1024;


pub struct KvStore {
    path: PathBuf,

    readers: HashMap<u64, BufReaderWithPos<File>>,

    writer: BufWriterWithPos<File>,

    current_gen: u64,

    index: BTreeMap<String, CommandPos>,

    uncompacted: u64,
}




/// Struct representing a command
#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set {key: String, value: String},
    Remove {key: String},
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set {key, value}
    }

    fn remove(key: String) -> Command {
        Command::Remove {key}
    }
}



/// Represents the position and length of a json-serialized command in the log
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
            len: range.end - range.start,
        }
    }
}

struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

impl <R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self>{
        let pos = inner.seek(SeekFrom::Current(0))?;

        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}














