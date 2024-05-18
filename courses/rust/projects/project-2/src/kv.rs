use std::{fs, io};
use std::collections::{BTreeMap, HashMap};
use std::env::current_dir;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use crate::{KvsError, Result};

// 使用其他 lib 中的 crate，一定要用 crate:: 声明

// unit of this threshold is bytes, 1024 * 1024 bytes = 1 KB
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

    // map generation number to the file reader // each x.log file has its own reader
    readers: HashMap<u64, BufReaderWithPos<File>>,

    // writer of the current log
    writer: BufWriterWithPos<File>,

    current_gen: u64,

    // key: <"hello", "world"> hello is key, commandPos is the position of command in xxx.log file
    // <key, key position in log file>
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
    // pub fn open(path: impl Into<PathBuf>) -> Result<KvStore>  {
    pub fn open<T>(path: T) -> Result<KvStore>
        where T: Into<PathBuf> {
        // In Rust, Impl Trait in an argument position is syntactic sugar for a generic type parameter like <T: Trait>.
        // However, the type is anonymous and doesn't appear in the GenericParam list.


        let path = path.into();

        // Recursively create a directory and all of its parent components if they are missing.
        fs::create_dir_all(&path)?;

        let mut readers = HashMap::new();
        let mut index = BTreeMap::new();

        let gen_list = sorted_gen_list(&path)?;
        let mut uncompacted = 0;

        for &gen in &gen_list {
            // initiate a reader for each x.log file and insert it into readers map
            let mut reader = BufReaderWithPos::new(File::open(log_path(&path, gen))?)?;

            uncompacted += load(gen, &mut reader, &mut index)?;
            readers.insert(gen, reader);
        }

        // unwrap_or: returned the contained Some value or a provided default value
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

    /// Sets the value of a string to a string
    ///
    /// If the key already exists, the previous value will be overwritten.
    ///
    /// # Errors
    /// It propagates I/O or serialization errors during writing the log.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key, value);
        let pos = self.writer.pos;

        // Serialize the given data structure as JSON into the I/O stream
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;

        if let Command::Set { key, .. } = cmd {
            if let Some(old_cmd) = self
                .index
                .insert(key, (self.current_gen, pos..self.writer.pos).into())
            {
                self.uncompacted += old_cmd.len;
            }
        }

        if self.uncompacted > COMPACTION_THRESHOLD {
            self.compact()?;
        }

        Ok(())
    }

    /// Get the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    ///
    /// # Errors
    ///
    /// If returns `KvsError::UnexpectedCommandType` if the given command type unexpected.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(cmd_pos) = self.index.get(&key) {
            let reader = self
                .readers
                .get_mut(&cmd_pos.gen)
                .expect("Cannot find log reader");

            //Seek to an offset, in bytes, in a stream.
            reader.seek(SeekFrom::Start(cmd_pos.pos))?;

            let cmd_reader = reader.take(cmd_pos.len);
            if let Command::Set { value, .. } = serde_json::from_reader(cmd_reader)? {
                Ok(Some(value))
            } else {
                Err(KvsError::UnexpectedCommandType)
            }
        } else {
            Ok(None)
        }
    }

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    ///
    /// It propagates I/O or serialization errors during writing the log.
    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            let cmd = Command::remove(key);
            serde_json::to_writer(&mut self.writer, &cmd)?;
            self.writer.flush()?;
            if let Command::Remove { key } = cmd {
                let old_cmd = self.index.remove(&key).expect("key not found");
                self.uncompacted += old_cmd.len;
            }
            Ok(())
        } else {
            Err(KvsError::KeyNotFound)
        }
    }

    /// Clears stale entries in the log
    pub fn compact(&mut self) -> Result<()> {
        // increase current gen by 2. current_gen + 1 is for the compaction file
        let compaction_gen = self.current_gen + 1;
        self.current_gen += 2;

        // self.writer point to the 3.log
        self.writer = self.new_log_file(self.current_gen)?;

        // this writer point to 2.log
        let mut compaction_writer = self.new_log_file(compaction_gen)?;

        let mut new_pos = 0; // pos in the new log file


        // what is the order of this mutable iterator
        for cmd_pos in &mut self.index.values_mut() {// get mutable iterator over the values of map
            let reader = self// get reader of each gen.log file
                .readers
                // return a mutable reference to the value for corresponding to the key
                .get_mut(&cmd_pos.gen)
                .expect("Cannot find log reader");

            if reader.pos != cmd_pos.pos {
                reader.seek(SeekFrom::Start(cmd_pos.pos))?;// seek to an offset, in bytes, in a stream
            }

            // This function returns a new instance of Read which will read at most limit bytes,
            // after which it will always return EOF
            let mut entry_reader = reader.take(cmd_pos.len);

            // copy the entire contents of a reader into a writer
            let len = io::copy(&mut entry_reader, &mut compaction_writer)?;
            *cmd_pos = (compaction_gen, new_pos..new_pos + len).into();
            new_pos += len;
        }

        compaction_writer.flush()?;

        // remove stale log files.
        let stale_gens: Vec<_> = self
            .readers
            .keys()
            .filter(|&&gen| gen < compaction_gen)
            .cloned()
            .collect();

        for stale_gen in stale_gens {
            self.readers.remove(&stale_gen);
            fs::remove_file(log_path(&self.path, stale_gen))?;
        }
        self.uncompacted = 0;

        Ok(())
    }

    /// Create a new log file with given generation number and add the reader to the readers map.
    ///
    /// Return the writer to the log
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
        // Options and flags which can be used to configure how a file is opened.
        // Opening a file for both reading and writing, as well as creating if if it doesn't exist
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)?,
    )?;

    // add reader into map
    readers.insert(gen, BufReaderWithPos::new(File::open(&path)?)?);

    // return the writer
    Ok(writer)
}

/// Returns sorted generation numbers in the given directory
pub fn sorted_gen_list(path: &Path) -> Result<Vec<u64>> {

    // return an iterator over the entry within a directory
    // let mut gen_list = fs::read_dir(&path)?
    let mut gen_list = fs::read_dir(path)?

        .flat_map(|res| -> Result<PathBuf> { Ok(res?.path()) })
        // .flat_map(|res| Ok::<PathBuf, KvsError>(res?.path()))

        // single expression, you do not need to write -> {} can be ignored
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))


        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
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
    reader: &mut BufReaderWithPos<File>,
    index: &mut BTreeMap<String, CommandPos>,
) -> Result<u64> {
    // To make sure we read from the beginning of the file.
    let mut pos = reader.seek(SeekFrom::Start(0))?;


    let mut stream =
        // create a json deserializer from an io::Reader
        Deserializer::from_reader(reader)

            // Turns a json deserializer into an iterator over value of type T
            .into_iter::<Command>();


    let mut uncompacted = 0; // number of bytes that can be saved after a compaction

    while let Some(cmd) = stream.next() {
        let new_pos = stream.byte_offset() as u64;
        match cmd? {
            Command::Set { key, .. } => {

                // If the map did not have this key present, None is returned
                // If the map did have this key present, the value is updated and the old value is returned.
                if let Some(old_cmd) = index.insert(key, (gen, pos..new_pos).into()) {
                    uncompacted += old_cmd.len;
                }
            }
            Command::Remove { key } => {
                if let Some(old_cmd) = index.remove(&key) {
                    uncompacted += old_cmd.len;
                }
                // the "remove" command itself can be deleted in the next compaction
                // so we add its length to `uncompacted`
                uncompacted += new_pos - pos;
            }
        }
        pos = new_pos;
    }
    Ok(uncompacted)
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
#[derive(Debug)]
struct CommandPos {
    // {"Set":{"key":"key1","value":"value1"}}, if this is first command in 1.log,
    // gen = 1, pos = 0, len = 39
    gen: u64,// prefix of the gen.log file
    pos: u64,// start position of command
    len: u64,// the length of the serialized command bytes
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


// File has impl the Read and Seek trait
impl<R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {

        // seek to an offset, in bytes, in a stream
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


#[test]
fn test_current_dir() -> crate::Result<()> {
    let current_dir = current_dir();

    let path: PathBuf = current_dir?.into();

    fs::create_dir_all(&path)?;

    let read_dir = fs::read_dir(&path)?;// iterator


    let step_one
        = read_dir.flat_map(|res| -> crate::Result<_> { Ok(res?.path()) });


    let step_two =
        step_one.filter(|path| path.is_file() && path.extension() == Some("log".as_ref()));


    let step_three = step_two.flat_map(|path| {
        path.file_name()
            .and_then(OsStr::to_str)
            .map(|s| s.trim_end_matches(".log"))
            .map(str::parse::<u64>)
    });

    let mut gen_list = step_three
        .flatten()
        .collect::<Vec<u64>>();

    gen_list.sort_unstable();

    let gen = 1;
    let new_path = &path.join(format!("{}.log", gen));

    let file = File::open(new_path)?;

    let mut readers: HashMap<u64, BufReaderWithPos<File>> = HashMap::new();
    let mut index: BTreeMap<String, CommandPos> = BTreeMap::new();

    let mut reader = BufReaderWithPos::new(file)?;

    let mut pos = (&mut reader).seek(SeekFrom::Start(0))?;

    let mut stream =
        Deserializer::from_reader(&mut reader).into_iter::<Command>();

    let mut uncompacted = 0;

    while let Some(cmd) = stream.next() {
        let new_pos = stream.byte_offset() as u64;
        match cmd? {
            Command::Set { key, .. } => {
                if let Some(old_cmd) = index.insert(key, (gen, pos..new_pos).into()) {
                    uncompacted += old_cmd.len;
                }
            }
            _ => {}
        }
        pos = new_pos;
    }


    println!();

    Ok(())
}


#[test]
fn test_load() -> Result<()> {
    let path = current_dir()?;
    let gen = 1;
    let mut reader = BufReaderWithPos::new(File::open(log_path(&path, gen))?)?;

    let mut deserializer = Deserializer::from_reader(reader);

    let mut stream = deserializer.into_iter::<Command>();

    while let Some(cmd) = stream.next() {
        let pos = stream.byte_offset() as u64;
        println!("{:?}", cmd?);
        println!();
    }


    Ok(())
}


#[test]
fn test_compact_log_file() -> Result<()> {
    let mut current_gen = 1;

    let compaction_gen = current_gen + 1;

    current_gen += 2;

    let mut readers: HashMap<u64, BufReaderWithPos<File>> = HashMap::new();

    let mut index = BTreeMap::<String, CommandPos>::new();

    let path = current_dir()?;


    let mut reader = BufReaderWithPos::new(File::open(log_path(&path, 1))?)?;

    load(1, &mut reader, &mut index)?;


    let mut compaction_writer = new_log_file(&path, compaction_gen, &mut readers)?;

    let mut new_pos = 0;


    for cmd_pos in &mut index.values_mut() {
        let reader = readers.get_mut(&cmd_pos.gen).expect("Cannot find log reader");

        if reader.pos != cmd_pos.pos {// let reader seek offset to the cmd
            reader.seek(SeekFrom::Start(cmd_pos.pos))?;
        }

        let mut entry_reader = reader.take(cmd_pos.len);

        let len = io::copy(&mut entry_reader, &mut compaction_writer)?;

        *cmd_pos = (compaction_gen, new_pos..new_pos + len).into();
        new_pos += len;
    }

    compaction_writer.flush()?;


    Ok(())
}


#[test]
fn test_command_pos_from() {

    // (gen, pos..new_pos).into()
    let command_pos = CommandPos::from((3, Range { start: 1, end: 10 }));

    println!("{:?}", command_pos);
}

#[test]
fn test_command_pos_into() {
    let command_pos: CommandPos = (2, Range { start: 0, end: 7 }).into();

    println!("{:?}", command_pos);

    let command_pos: CommandPos = (3, 34..67).into();
    println!("{:?}", command_pos);
}