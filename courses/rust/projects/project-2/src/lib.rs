// #[deny(missing_docs)]
//! A simple key/value store.


pub use kv::KvStore;
pub use error::{Result, KvsError};// 在这里声明，其他地方才可以用
pub use kv::sorted_gen_list;

mod kv;
mod error;