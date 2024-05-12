use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::Result;

// 使用其他 lib 中的 crate，一定要用 crate:: 声明
const COMPACTION_THRESHOLD: u64 = 1024 * 1024;


/// The `KvStore` stores string key/value pairs
/// Key/value paris are persisted to disk on log files. Log files are name after
/// Monotonically increasing generation numbers with a `log` extension name.
/// A `BTreeMap` in memory stores the keys and the value locations for fast query.
///
/// ```rust
/// # use kvs::{KvsError, KvStore, Result};
/// # fn try_main() -> Result<()> {
/// use std::env::current_dir;
/// let mut store = KvStore::open(current_dir()?)?;
/// store.set("key".to_owned(), "value".to_owned())?;
/// let val = store.get("key".to_owned())?;
/// assert_eq!(val, Some("value".to_owned()));
/// # Ok(())
/// # }
///
/// ```
///
///
pub struct KvStore {
    // directory for the log and other data
    path: PathBuf,

    // map generation number to the file reader
    readers: HashMap<u64, BufReaderWithPos<File>>,

    // writer of the current log
    writer: BufWriterWithPos<File>,

    current_gen: u64,

    index: BTreeMap<String, CommandPos>,

    // the number of bytes representing "stale" commands that could be
    // delete during a compaction
    uncompacted: u64,
}


impl KvStore {
    /// Opens a `KvStore` with the given path.
    ///
    /// This will create a new directory if the given one does not exist.
    ///
    /// # Errors
    ///
    /// It propagates I/O or deserialization errors during the log replay.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();

        fs::create_dir_all(&path)?;

        let mut readers = HashMap::new();
        let mut index = BTreeMap::new();

        let gen_list = sorted_gen_list(&path)?;
        let mut uncompacted = 0;

        for &gen in &gen_list {
            let mut reader = BufReaderWithPos::new(File::open(log_path(&path, gen))?)?;
            uncompacted += load(gen, &mut reader, &mut index)?;
            readers.insert(gen, reader);
        }

        let current_gen = gen_list.last().unwrap_or(&0) + 1;
        let writer = new_log_file(&path, current_gen, &mut readers)?;


        Ok(KvStore {
            path,
            readers,
            writer,
            current_gen,
            index,
            uncompacted,
        })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(None)
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        Ok(())
    }

    pub fn compact(&mut self) -> Result<()> {
        Ok(())
    }

    fn new_log_file(&mut self, gen: u64) -> Result<BufWriterWithPos<File>> {
        new_log_file(&self.path, gen, &mut self.readers)
    }
}

/// Create a new log file with given generation number and add the reader to the readers to map.
///
/// Returns the writer to the log
fn new_log_file(
    path: &Path,
    gen: u64,
    readers: &mut HashMap<u64, BufReaderWithPos<File>>,
) -> Result<BufWriterWithPos<File>> {
    let path = log_path(&path, gen);

    let writer = BufWriterWithPos::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?,
    )?;

    readers.insert(gen, BufReaderWithPos::new(File::open(&path)?)?);

    Ok(writer)
}

/// Returns sorted generation numbers in the given directory
fn sorted_gen_list(path: &Path) -> Result<Vec<u64>> {
    let mut gen_list = fs::read_dir(&path);

    Ok(vec![])
}

fn load(
    gen: u64,
    reader: &mut BufReaderWithPos<File>,
    index: &mut BTreeMap<String, CommandPos>,
) -> Result<u64> {
    Ok(0)
}

fn log_path(dir: &Path, gen: u64) -> PathBuf {
    dir.join(format!("{}.log", gen))
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

impl<R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;

        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}


impl<R: Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;

        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos,
        })
    }
}


impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}













