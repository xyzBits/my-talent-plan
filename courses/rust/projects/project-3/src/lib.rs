// This line re-exporting the KvsError struct and Result type from
// the error module, making them accessible to other modules that
// import this module. This allows other modules to use these types
// without having to directly import the error module
pub use client::KvsClient;
pub use engines::{KvStore, KvsEngine, SledKvsEngine};
pub use error::{KvsError, Result};
pub use server::KvsServer;

// this line importing the error module, which contains definitions
// for the KvsError struct and Result type
mod error;

mod common;

mod engines;

mod client;

mod server;
