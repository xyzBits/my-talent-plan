# PNA Rust Project 4: Concurrency and parallelism

**Task**: Create a _multi-thread_, persistent key/value store server and client 
with synchronous networking over a custom protocol.

**Goals**:

- Write a simple thread pool
- Use channels for cross-thread communication
- Share data structures with lock
- Perform reader operations with locks 
- Benchmark single-threaded vs multithreaded

**Topics**: thread pools, channels, locks, lock-free data structures,


## Introduction

In this project you will create a simple key/value server and client that
communicate over a custom protocol. The server will use synchronous networking,
and will respond to multiple requests using increasingly sophisticated
concurrent implementations. The in-memory index will become a concurrent
data structure, shared by all threads, and compaction will be done on a
dedicated thread, to reduce latency of individual requests. 