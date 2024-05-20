use std::collections::BTreeMap;
use std::env::current_dir;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::{fs, io};

// use tempfile::TempDir;

fn hello(input: impl Into<String> + Display) {
    println!("input = {}", input);
}

#[test]
fn test_hello() {
    let input = "hello world";
    hello(input);
}

#[test]
fn test_into_path_buf() {
    let mut path_buf = PathBuf::new();
    path_buf.push("..");

    // let temp_dir = TempDir::new().expect("unable to create temporary working directory");

    // let path = temp_dir.path();
}

fn open_path(path: impl Into<PathBuf>) {}

/// Creates an iterator that works like map, but flattens nested structure.
///
/// The `map` adapter is very useful, but only when the closure argument produces values.
/// If it produces an iterator instead, there's an extra layer of indirection.
/// flat_map() will remove this extra layer on its own.
///
/// You can think of flat_map as the semantic equivalent of map ping, and then flattening as in map(f).flatten().
///
/// Another way of thinking about flat_map(): map's closure return one item for each element, and flat_map()'s closure
/// returns an iterator for each element.
#[test]
fn test_flat_map() {
    let nested_numbers = vec![vec![1, 2, 3], vec![4, 5]];

    // Using map (doesn't flatten)
    let mapped_numbers: Vec<_> = nested_numbers
        .iter()
        .map(|inner_list| {
            inner_list.iter().map(|num| *num * 2) // Double each number in the inner list
        })
        .collect(); // Collects as a vector of vectors

    // Using flat_map (flattens and transforms)
    let flat_numbers = nested_numbers
        .iter()
        .flat_map(|inner_list| inner_list.iter().map(|num| num * 2))
        .collect::<Vec<i32>>(); // Collects as a flat vector of numbers

    println!("Mapped (not flattened): {:?}", mapped_numbers);
    println!("Flattened: {:?}", flat_numbers);
}

#[test]
fn test_as_ref() {
    let hello = "hello";
    // convert this into shared reference type
    let hello_ref: &OsStr = hello.as_ref();
}

#[test]
fn test_seek_trait() -> io::Result<()> {
    let mut file = File::open("../Cargo.toml")?;

    // mov ethe cursor 42 bytes from the start of the file
    let result = file.seek(SeekFrom::Start(42))?;

    println!("{}", result);

    Ok(())
}

#[test]
fn test_path_join() -> io::Result<()> {
    let path_buf = current_dir()?;

    let buf = path_buf.join("/hello");

    println!();

    Ok(())
}

#[test]
fn test_buffer_reader() -> std::io::Result<()> {
    let file = File::open("../Cargo.toml")?;

    let mut reader = BufReader::new(file);

    let mut line = String::new();

    // read all byte until a new line
    let len = reader.read_line(&mut line)?;
    println!("{}", len);

    Ok(())
}

#[test]
fn test_vec_last_unwrap_or() {
    let v: Vec<i32> = vec![];

    let last = v.last().unwrap_or(&-1);

    println!("{}", last);
}

#[test]
fn test_open_option() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("../Cargo.toml")?;

    Ok(())
}

#[test]
fn test_i32_into() {
    let x = 34_i16;

    let x1: i32 = x.into();
}

#[test]
fn test_btree_map() {
    let mut map = BTreeMap::new();
    map.insert(3, "hello".to_string());
    map.insert(2, "world".to_string());

    for value in map.values_mut() {
        // iterate order is the order of key
        println!("{}", value);
        value.push_str("!");
    }
}

#[test]
fn test_seek_and_take() -> std::io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open("hello.log")?;

    let mut reader = BufReader::new(file);

    let len = reader.seek(SeekFrom::Current(6))?;

    let mut stream = reader.take(5);

    let mut hello = String::new();
    let len = stream.read_line(&mut hello)?;

    println!("{}", hello);

    Ok(())
}

#[test]
fn test_range() {
    let range = (1..4);
    let sum: i32 = range.sum();
    println!("{:?}", sum);
}

// test all the api used in this crate

#[test]
fn test_dir_path_use_for() -> std::io::Result<()> {
    let path_buf = current_dir()?;
    let read_dir = fs::read_dir(&path_buf)?;

    for entry in read_dir {
        let entry = entry?;
        let path_buf = entry.path();
        if path_buf.is_file() {
            println!(
                "this is a file {}",
                path_buf.file_name().unwrap().to_str().unwrap()
            );
        } else if path_buf.is_dir() {
            println!(
                "this is a dir {}",
                path_buf.file_name().unwrap().to_str().unwrap()
            );
        }
    }

    Ok(())
}

use kvs::{KvsError, Result};

#[test]
fn test_read_dir_result() -> io::Result<()> {
    let path_buf = current_dir()?;
    let read_dir = fs::read_dir(&path_buf)?;

    for entry in read_dir {
        let res: Result<PathBuf> = Ok(entry?.path());
    }

    Ok(())
}

#[test]
fn test_closure_specify_return_type() {
    /// In Rust, -> syntax in closures is used to specify the return type.
    /// When defining a closure, you can explicitly declare the input and output types
    /// using the |parameter| -> return_type {body} syntax.
    ///
    /// For example, in closure |x: i32| -> i32 { x + 1},
    /// the -> i32 specifies that the closure will return an i32 type.
    /// This can be helpful for explicitly stating the expected return type
    /// of the closure, especially in cases where it my not be inferred by the compiler.
    ///
    /// However, in many cases, Rust's type inference system can automatically determine the return type
    /// of the closure based on the body of the closure, so explicitly specifying the return type is
    /// not always necessary
    let f1 = |x: i32| -> String { x.to_string() };
}

#[test]
fn test_map() {
    let data = vec![1, 2, 3, 4];
    let result = data.iter().map(|item| *item * 2).collect::<Vec<_>>();
}


#[test]
fn test_string_slice_as_ref() {
    let hello = "hello";
    // as_ref: convert this type into a shared reference of the (usually inferred) input type

    // The as_ref function in Rust is a method that is used to convert a value into
    // a reference of a different type.
    let hello_as_ref: &OsStr = hello.as_ref();
    //  ---- type must be known at this point
    println!("{:?}", hello_as_ref);
}


#[test]
fn test_drop() {
    let vec = vec![1, 2, 3, 4];

    drop(vec);// set random value

    println!();
}