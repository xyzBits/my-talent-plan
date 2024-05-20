# PNA Rust Project 3: Synchronous client-server networking

**Task**: Create a _single-threaded_, persistent key/value store _server
and client with synchronous networking over a custom protocol_.

**Goals**:

- Create a client-server application 
- Write a custom protocol with `std` networking APIs
- Introduce logging to the server 
- Implement pluggable backends with traits 
- Benchmark the hand-written backend against `sled`

**Topics**: `std::net`, logging, traits, benchmarking.

- [Introduction](#introduction)
- [Project spec](#project-spec)
- [Project setup](#project-setup)
- [Part 1: Command line parsing](#part-1-command-line-parsing)
- [Part 2: Logging](#part-2-logging)
- [Part 3: Client-server networking setup](#part-3-client-server-networking-setup)
- [Part 4: Implementing commands across the network](#part-4-implementing-commands-across-the-network)
- [Part 5: Pluggable storage engines](#part-5-pluggable-storage-engines)
- [Part 6: Benchmarking](#part-6-benchmarking)

## Introduction 

In this project you will create a simple key/value server and client. They will
communicate with a custom networking protocol of your design. You will emit logs
using standard logging crates, and handle errors correctly across the network
boundary. Once you have a working client-server architecture,
then you will abstract the storage engine behind traits, and compare
the performance of yours to the `sled` engine.

## Project spec

The cargo project, `kvs`, builds a command-cline key-value store client called
`kvs-client`, and a key-value store server called `kvs-server`, both of which in
turn call into a library called `kvs`. The client speaks to the server over
a custom protocol.

The `kvs-server` executable supports the following command line arguments:

- `kvs-server [--addr IP-PORT] [--engine ENGINE-NAME]`

    Start the server and begin listening for incoming connections. `--addr`
    accepts an IP address, either v4 or v6, and a port number, with the format
    `IP:PORT`. If `--addr` is not specified then listen on `127.0.0.1:4000`.

    If `--engine` is specified, then `ENGINE-NAME` must be either "kvs", in which
    case the built-in engine is used, or "sled", in which case sled is used. If
    this is the first run (there is no data previously persisted) then the default
    value is "kvs": if there is previously persisted data then the default is the
    engine already in use. If data was previously persisted with a different
    engine than selected, print an error and exit with a non-zero exit code.

## Project setup

## Part 1: Command line parsing


## Part 2: Logging


## Part 3: Client-server networking setup


## Part 4: Implementing commands across the network

## Part 5: Pluggable storage engines


## Part 6: Benchmarking

