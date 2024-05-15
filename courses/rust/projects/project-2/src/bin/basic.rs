use std::fmt::Display;
use std::path::PathBuf;

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
}

fn open_path(path: impl Into<PathBuf>) {}
