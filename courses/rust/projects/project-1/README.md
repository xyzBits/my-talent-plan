# PNA Rust Project 1: The Rust toolbox 

**Task**: Create an in-memory key/value store that passes simple tests and responds to command-cline arguments

**Goals** 
- Install the Rust compiler and tools 
- Learn the project structure used through this course 
- Use `Cargo init` / `run` / `test` / `clippy` / `fmt`
- Learn how to find and import crates from crates.io
- Define an appropriate data type for a key-value store

**Topics**: testing, the `clap` crate, `CARGO_VERSION` etc., the `clippy` and `rustfmt` tools.

**Extensions**: the `structopt` crate.

- [Introduction](#user-content-introduction)

## Introduction

In this project you will create a smiple in-memory key/value store that maps strings to strings, and that passes some tests and responds to command line arguments. The focus of this project is on the tooling and setup that goes into a typical Rust project.

If this sounds basic to you, please to the project anyway as it discuss some general patterns that will be used throughout the course.


## Project spec

The cargo project, `kvs`, builds a command-line key-value store client called `kvs`, which in turns calls into a library called `kvs`.

The `kvs` executable supports the following command line arguments:

- `kvs set <KET> <VALUE>`

    Set the vlaue of a string key to a string

- `kvs get <KEY>`

    Get the string value of a given string key

- `kvs rm <KEY>`

    remove a given key

- `kvs -V`

    Print the version 

The `kvs` library contains a type, `KvStore`, that support the following methods:

- `KvStore::set(&mut self, key: String, value: String)`

    Set the value of a string key to a strin g

- `KvStore::get(&self, key: String) -> Option<String>`

    Get the string value of a string key. If the key does not exist, return `None`.

- `KvStore::remove(&mut self, key: String)`

    Remove a give key.

The `KvStore` type stores values in-memory, and thus the command-line client can do litlle more than print the version. The `get` / `set` / `rm` commands will return an "unimplemented" error when run from the command line. Future projects will store values on disk and have a working command line interface.

## Installation

At this point in your Rust programming experience you should know how to install Rust via [rustup].

[rustup]: https://www.rustup.rs

If you haven't already, do so now by running
```
curl https://sh.rustup.rs -sSf | sh
```

## Project setup

You will do the work for this project in your own git repository, with your own Cargo project, You will import the test cases for the project from the [source repository for this course][course]

[course]: https://github.com/pingcap/talent-plan

Note that within that repository, all content related to this course is within the `rest` subdirectory. You may ignore any other directories.

The projects in this course contain both libraries and excutables. They are executables because we are developing an application that can be run. They are libraries because the supplied test cases must link to them.

We'll use the same setup for each project in this course.

The directory layout we will use is:

```
├── Cargo.toml
├── src
│   ├── bin
│   │   └── kvs.rs
│   └── lib.rs
└── tests
    └── tests.rs
```

The `Cargo.toml`, `lib.rs` and `kvs.rs` files looks as follows:

`Cargo.toml`:

```toml
[package]
name = "kvs"
version = "0.1.0"
authors = ["A certain talent <talent@gmail.com>"]
description = "A key-value store"
edition = "2021"
```

`lib.rs`:

```rust
// just leave it empty for now
```

`kvs.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

The author should be yourself, but the name needs to be `kvs` in order for the test cases to work. that's because the project name is also the name of library it contains. Likewise the name of the library (the command line application) needs to be `kvs`. In the above setup it will be `kvs` implicityly based on the file name, but you could name the file whatever you wanted by putting the appropriate information in the manifest (`Cargo.toml`).

You may set up this project with `cargo new --lib`, `cargo init --lib` (int a clean directory), or manually. You'll probably also want be initialize a git repository in the same directory.

Finally, the `tests` directory is copied from the course materials. In this case, copy from the course repository the file `rust/projects/project-1-tests` into your own repository, as `test`.

At this point you should be able to run the program with `cargo run`.

_Try it now._

You are set up for this project and read to start hacking.


## Part 1: Make the tests compile

You've been provided with a suite of unit tests in `tests/tests.rs`. Open it up and take a look.

_Try to run the tests with `cargo test`._ What happens?

Your first task for this project is to make the tests _compile_. Fun!

If you project is like mine you probably saw a huge spew of build errors. Look at the first few. In general, when you see a bunch of errors, the first are the most important &mdash; `rustc` will keep trying to compile even after hiting errors, so errors can cascade, the later ones being pretty meaningless. Your first few eror probably look like:

```
error[E0433]: failed to resolve: use of undeclared crate or module `assert_cmd`
 --> tests/tests.rs:1:5
  |
1 | use assert_cmd::prelude::*;
  |     ^^^^^^^^^^ use of undeclared crate or module `assert_cmd`

error[E0433]: failed to resolve: use of undeclared crate or module `predicates`
 --> tests/tests.rs:3:5
  |
3 | use predicates::str::contains;
```
(If you are seeing something else, plese file an issue).

These two errors are quite hard to diagnose to a new Rust programmer so I'll 
just tell you what's going on there: you are missing  [dev-dependency] crates
in your manifest.

[dev-dependency]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies

For this project your `Cargo.toml` needs to contain these lines:

```toml 
[dev-dependencies]
assert_cmd = "0.11.0"
predicates = "1.0.0"
```

The details of these dependencies are not important to yu completing the 
project, but you might want to investigate them on you own. We didn't tell you
about the need for dev-deps earlier just so you would experience these errors
yourself. In future projects, the setup text will tell you the dev-deps you need.


One quick note: how can you figure out that these error due to missing
dependencies in your manifest and not due to error in your source code? Here's 
one big clue, from the error shown previously: 

```
1 | use assert_cmd::prelude::*;
  |     ^^^^^^^^^^ use of undeclared crate or module `assert_cmd`
```

In `use` statements the first path element is always the name of crate. The
exception to this is when the first path element references a name that was
previously brought into scope with _another_ `use` statement. In other words, if
there had been another `use` statement in this file like `use foo::assert_cmd`,
then use `assert_cmd::prelude::*` would refer to _that_ `assert_cmd`. There is
more that could be said about this but we shouldn't go deeper into the nuances
of path resolution here. Just know that, in general, in a `use` statement, if
the first element in the path isn't found (i.e. cannot be resolved), the problem
is probably that the crate hasn't been name in the manifest.

