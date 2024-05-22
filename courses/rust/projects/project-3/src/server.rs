use std::io::{BufReader, BufWriter, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream, ToSocketAddrs};

use log::{debug, error};
use serde_json::Deserializer;

use crate::common::{GetResponse, RemoveResponse, Request, SetResponse};
use crate::KvsEngine;
use crate::Result;

/// The server of key value store
pub struct KvsServer<E: KvsEngine> {
    engine: E,
}

impl<E: KvsEngine> KvsServer<E> {
    /// Create a `KvsServer` with a given storage engine.
    pub fn new(engine: E) -> Self {
        KvsServer { engine }
    }

    pub fn run<A: ToSocketAddrs>(mut self, addr: A) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = self.server(stream) {
                        error!("Error on serving client: {}", e);
                    }
                }
                Err(e) => {
                    error!("Connection failed: {}", e);
                }
            }
        }

        Ok(())
    }

    fn server(&mut self, tcp: TcpStream) -> Result<()> {
        // Returns the socket address of the remote peer of this TCP connection.
        let peer_addr = tcp.peer_addr()?;
        let reader = BufReader::new(&tcp);
        let mut writer = BufWriter::new(&tcp);

        // create a JSON deserializer from an io::Read
        let req_reader = Deserializer::from_reader(reader).into_iter::<Request>(); // Turn a JSON deserializer into an iterator over values of type T

        /// Defines a macro named send_resp! that serializes a Rust data structure ($resp)
        /// into JSON format and sends it to a writer-like object.
        /// macro_rules! introduce a new macro definition
        /// send_resp! this is the name of macro
        /// ($resp: expr) defines a single argument named `$resp` that can be any expression.
        /// the curly braces {} enclose the code that will be executed when you call the macro.
        /// `let resp = $resp;` assigns the value of the argument(`$resp`) to a new variable named `resp`
        ///
        macro_rules! send_resp {
            ($resp: expr) => {{
                let resp = $resp;
                serde_json::to_writer(&mut writer, &resp)?;
                writer.flush()?; // must import Write trait, why
                debug!("Response sent to {}: {:?}", peer_addr, resp);
            };};
        }

        for req in req_reader {
            let req = req?;
            debug!("Receive request from {}: {:?}", peer_addr, req);

            match req {
                Request::Get { key } => send_resp!(match self.engine.get(key) {
                    Ok(value) => GetResponse::Ok(value),
                    Err(e) => GetResponse::Err(format!("{}", e)),
                }),

                Request::Set { key, value } => send_resp!(match self.engine.set(key, value) {
                    Ok(_) => SetResponse::Ok(()),
                    Err(e) => SetResponse::Err(format!("{}", e)),
                }),
                Request::Remove { key } => send_resp!(match self.engine.remove(key) {
                    Ok(_) => RemoveResponse::Ok(()),
                    Err(e) => RemoveResponse::Err(format!("{}", e)),
                }),
            }
        }

        Ok(())
    }
}

#[test]
fn test_tcp_peer_addr() {
    // let stream = TcpStream::connect("127.0.0.1:8080")
    //     .expect("Couldn't connect to the server...");
    //
    // assert_eq!(stream.peer_addr().unwrap(),
    //     SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)));
}

use std::fs::File;

#[test]
fn test_write_trait() -> Result<()> {
    let file = File::open("hello.txt")?;
    let mut buf_writer = BufWriter::new(file);

    // Method `write` not found in the current scope for struct `BufWriter<File>` [E0599]
    buf_writer.write("hello world".as_bytes())?;

    Ok(())
}

#[test]
fn test_serde_json_deserialize() -> Result<()> {
    let file = File::open("1.log")?;
    let reader = BufReader::new(file);
    let request_iter = Deserializer::from_reader(reader).into_iter::<Request>();

    for request in request_iter {
        let request = request?;

        println!("{:?}", request);
    }

    Ok(())
}
