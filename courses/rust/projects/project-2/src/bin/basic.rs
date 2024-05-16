use std::collections::{BTreeMap, HashMap};
use std::env::current_dir;
use std::ffi::OsStr;
use std::fmt::Display;
use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::Command;

use serde_json::Deserializer;
use tempfile::TempDir;

use kvs::{ Result};

fn main() {}

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
    path_buf.push(".");

    let temp_dir = TempDir::new().expect("unable to create temporary working directory");

    let path = temp_dir.path();
}

fn open_path(path: impl Into<PathBuf>) {}





#[test]
fn test_flat_map() {
    let nested_numbers = vec![vec![1, 2, 3], vec![4, 5]];

    // Using map (doesn't flatten)
    let mapped_numbers: Vec<_> = nested_numbers
        .iter()
        .map(|inner_list| {
            inner_list.iter().map(|num| *num * 2) // Double each number in the inner list
        }).collect();  // Collects as a vector of vectors

// Using flat_map (flattens and transforms)
    let flat_numbers = nested_numbers.iter()
        .flat_map(|inner_list| inner_list.iter().map(|num| num * 2))
        .collect::<Vec<i32>>();  // Collects as a flat vector of numbers

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
    let mut file = File::open("Cargo.toml")?;

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
fn test_buffer_reader() -> std::io::Result<()>{
    let file = File::open("Cargo.toml")?;

    let mut reader = BufReader::new(file);

    let mut line = String::new();

    // read all byte until a new line
    let len = reader.read_line(&mut line)?;
    println!("{}", len);

    Ok(())

}















