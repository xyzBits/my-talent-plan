use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::net::{TcpStream, ToSocketAddrs};

use serde::Deserialize;
use serde_json::de::IoRead;
use serde_json::Deserializer;

use crate::{KvsError, Result};
use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};

/// Key value store client
pub struct KvsClient {
    // wrap TcpStream using serde_json::Deserializer
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    /// Connect to `addr` to access `KvsServer`.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        // Opens a TCP connection to a remote host
        // ToSocketAddrs is an address of the remote host. Anything which implements ToSocketAddrs trait can be supplied for the address
        let tcp_reader = TcpStream::connect(addr)?;

        // Creates a new independently owned handle to the underlying socket
        let tcp_writer = tcp_reader.try_clone()?;

        Ok(KvsClient {
            // Creates a JSON deserializer from an io::Read
            reader: Deserializer::from_reader(BufReader::new(tcp_reader)),

            // creates a new BufWriter<W> with a default buffer capacity. The default is currently 8 KiB, but many change in the future.
            writer: BufWriter::new(tcp_writer),
        })
    }


    /// Get the value of a given key from the server
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        // Serialize the given data structure as JSON into the I/O stream
        serde_json::to_writer(&mut self.writer, &Request::Get { key })?;
        self.writer.flush()?;

        let resp = GetResponse::deserialize(&mut self.reader)?;
        match resp {
            GetResponse::Ok(value) => { Ok(value) }
            GetResponse::Err(msg) => { Err(KvsError::StringError(msg)) }
        }
    }

    /// Set the value of a string key in the server
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Set { key, value })?;
        self.writer.flush()?;

        let resp = SetResponse::deserialize(&mut self.reader)?;
        match resp {
            SetResponse::Ok(_) => { Ok(()) }
            SetResponse::Err(msg) => { Err(KvsError::StringError(msg)) }
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        serde_json::to_writer(&mut self.writer, &Request::Remove { key })?;
        self.writer.flush()?;

        let resp = RemoveResponse::deserialize(&mut self.reader)?;
        match resp {
            RemoveResponse::Ok(_) => { Ok(()) }
            RemoveResponse::Err(msg) => { Err(KvsError::StringError(msg)) }
        }
    }
}

#[test]
fn test_serde_json_write() -> Result<()> {
    // File::open() open's a file in read-only mode. Instead you have to use File::create() which
    // opens a file write-only mode. Alternatively yu can also use OpenOptions, to further specify if you
    // want to append() to a file instead.
    let file = OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(true)
        .open("2.log")?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &Request::Get { key: "hello".to_string() })?;
    writer.flush()?;

    let mut reader = BufReader::new(File::open("2.log")?);
    let mut reader = Deserializer::from_reader(BufReader::new(reader));

    let resp = Request::deserialize(&mut reader)?;
    println!("{:?}", resp);

    Ok(())
}