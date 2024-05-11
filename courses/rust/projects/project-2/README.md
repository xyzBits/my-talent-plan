# PNA Rust Project 2: Log-structured fil I/O

**Task** Create a _persistent_ key/value store that _can be accessed from the command line._

**Goals**:

- Handle and report errors robustly
- Use serde for serialization
- Write data to disk as log using a standard file APIss
- Read the state of the key/value store from disk 
- Map in-memory key-indexes to on-disk values
- Periodically compact the log to remove stale data

**Topics**: log-structured file I/O, bitcask, the `failure` crate,
`Read` `Write` traits, the `serde` crate.


## Introduction

In this project you will crate a simple on-disk key/value store that can be
modified and queried from the command line. It will use a simplification of the
storage algorithm used by [bitcask], chosen for its combination of simplicity
and effectiveness. You will start by maintaining a _log_ (sometimes called a
["write-ahead log"][wall] or "WAL") on disk of previous write commands that is evaluated on startup to re-create the state of the database in memory. Then you
will extend that by storing only the keys in memory, along with offsets into the
on-disk log. Finally, you will introduce log compaction so that it does not grow
indefinitely. At the end of this project you will have built a simple, but
well-architected database using Rust file APIs.

[wal]: https://en.wikipedia.org/wiki/Write-ahead_logging
[bitcask]: https://github.com/basho/bitcask
