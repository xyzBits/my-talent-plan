

// This line re-exporting the KvsError struct and Result type from
// the error module, making them accessible to other modules that
// import this module. This allows other modules to use these types
// without having to directly import the error module
pub use error::{KvsError, Result};


// this line importing the error module, which contains definitions
// for the KvsError struct and Result type
mod error;