// #[deny(missing_docs)]
//! A simple key/value store.

pub use error::{KvsError, Result}; // 在这里声明，其他地方才可以用
pub use kv::sorted_gen_list;
pub use kv::KvStore;

mod error;
mod kv;
