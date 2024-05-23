use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use crate::{KvsError, Result};

use super::KvsEngine;

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;

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
    // directory for the log and other data
    path: PathBuf,

    // map generation number to the file reader
    readers: HashMap<u64, BufReaderWitPos<File>>,

    // writer of the current log
    writer: BufWriterWithPos<File>,

    current_gen: u64,

    index: BTreeMap<String, CommandPos>,

    // deleted during a compaction
    uncompacted: u64,
}

impl KvStore {
    /// Opens a `KvStore` with the given path
    ///
    /// This will create a new directory if the given one does not exit.
    ///
    /// # Errors
    ///
    /// It propagates I/O or deserialization errors during the log replay
    // pub fn open<T: Into<PathBuf>>(path: T) -> Result<KvStore> {
    pub fn open<T: Into<PathBuf> + AsRef<Path>>(path: T) -> Result<KvStore> {
        // the type of path is T, after into, the type is pathBuf, it is convenient to the future use
        let path = path.into();
        fs::create_dir_all(&path)?;

        let mut readers = HashMap::new();
        let mut index = BTreeMap::new();

        let gen_list = sorted_gen_list(&path)?;
        let mut uncompacted = 0;

        for &gen in &gen_list {
            let mut reader = BufReaderWitPos::new(File::open(log_path(&path, gen))?)?;
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

    /// Clears stale entries in the log
    pub fn compact(&mut self) -> Result<()> {
        // increase current gen by 2. current_gen + 1 is for the compaction file
        let compaction_gen = self.current_gen + 1;
        self.current_gen += 2;

        self.writer = self.new_log_file(self.current_gen)?;

        let mut compaction_writer = self.new_log_file(compaction_gen)?;

        let mut new_pos = 0; // pos in the new log file
        for cmd_pos in &mut self.index.values_mut() {

            let reader = self
                .readers
                .get_mut(&cmd_pos.gen)
                .expect("Cannot find log reader");

            if reader.pos != cmd_pos.pos {
                // get command will let pos seek to a specific position,
                // so here need to let the reader back to origin position
                reader.seek(SeekFrom::Start(cmd_pos.pos))?;
            }

            // just read len size and wrap content as a bufReader stream
            let mut entry_reader = reader.take(cmd_pos.len);
            let len = io::copy(&mut entry_reader, &mut compaction_writer)?;
            *cmd_pos = (compaction_gen, new_pos..new_pos + len).into();
            new_pos += len;
        }
        compaction_writer.flush()?;

        // remove stale log files
        let stale_gens = self.
            readers
            .keys()
            .filter(|&&gen| gen < compaction_gen)
            // create an iterator which clone all if its element
            .cloned()
            .collect::<Vec<_>>();

        for stale_gen in stale_gens {
            self.readers.remove(&stale_gen);
            fs::remove_file(log_path(&self.path, stale_gen))?;
        }

        self.uncompacted = 0;

        Ok(())
    }

    /// Create a new log file with given generation number and add the reader to the readers map
    fn new_log_file(&mut self, gen: u64) -> Result<BufWriterWithPos<File>> {
        new_log_file(&self.path, gen, &mut self.readers)
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

/// Create a new log file with given generation number and add the reader to the readers map
///
/// Returns the writer to the log
fn new_log_file(
    path: &Path,
    gen: u64,
    readers: &mut HashMap<u64, BufReaderWitPos<File>>,
) -> Result<BufWriterWithPos<File>> {
    let path_buf = log_path(path, gen);
    let writer = BufWriterWithPos::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path_buf)?,
    )?;

    readers.insert(gen, BufReaderWitPos::new(File::open(&path_buf)?)?);
    Ok(writer)
}

/// Returns sorted generation numbers in the given directory
fn sorted_gen_list(path: &Path) -> Result<Vec<u64>> {
    let mut gen_list = fs::read_dir(path)?
        // .flat_map(|res | -> Result<PathBuf>{Ok(res?.path())})
        // .flat_map(|res| Ok(res?.path()))
        .flat_map(|res| Ok::<_, KvsError>(res?.path()))

        // cannot use the '?' operator in a closure that returns 'PathBuf'
        // .map(|item| {item?})
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))

        .flat_map(|path|
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        )
        .flatten()
        .collect::<Vec<u64>>();

    gen_list.sort_unstable();
    Ok(gen_list)
}

/// Load the whole log file and store value locations in the index map
///
/// Returns how many bytes can be saved after a compaction
fn load(
    gen: u64,
    reader: &mut BufReaderWitPos<File>,
    index: &mut BTreeMap<String, CommandPos>,
) -> Result<u64> {
    // To make sure we read from the beginning of the file
    let mut pos = reader.seek(SeekFrom::Start(0))?;

    let mut stream =
        Deserializer::from_reader(reader).into_iter::<Command>();

    // number of bytes that can be saved after a compaction
    let mut uncompacted = 0;

    while let Some(cmd) = stream.next() {
        // if this is first command, and then  new_pos = cmd.len
        let new_pos = stream.byte_offset() as u64;

        match cmd? {
            Command::Set { key, .. } => {
                // old value is returned
                if let Some(old_cmd) = index.insert(key, (gen, pos..new_pos).into()) {
                    uncompacted += old_cmd.len;
                }
            }

            Command::Remove { key } => {
                if let Some(old_cmd) = index.remove(&key) {// if key is remove, it must be in index map
                    uncompacted += old_cmd.len;
                }

                // the "remove" command itself can be deleted in the next compaction
                // so we add its length to 'uncompacted'
                uncompacted += new_pos - pos;
            }
        }
        pos = new_pos;
    }


    Ok(uncompacted)
}

fn log_path(dir: &Path, gen: u64) -> PathBuf {
    let file_name = format!("{}.log", gen);
    //impl AsRef<Path> for String
    dir.join(file_name)
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


impl<R: Seek + Read> BufReaderWitPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWitPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

impl<R: Seek + Read> Seek for BufReaderWitPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

impl<R: Seek + Read> Read for BufReaderWitPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = self.read(buf)?;
        self.pos += len as u64;
        Ok(len)
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


#[cfg(test)]
mod kvs_test_mod {
    use std::env::current_dir;
    use std::fmt::Debug;

    use crate::{KvStore, Result};

    #[test]
    fn test_path_buf() -> Result<()> {
        let path_buf = current_dir()?;
        let store = KvStore::open(&path_buf)?;

        println!("{:?}", &path_buf);

        Ok(())
    }

    fn hello<T: Into<String> + Debug>(input: T) {
        println!("{:?}", input);
    }

    #[test]
    fn test_hello() {
        let input = String::from("hello");
        hello(&input);

        let input = "hello";
        hello(input);
    }
}