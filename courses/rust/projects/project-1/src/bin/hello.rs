use std::process::Command;
use assert_cmd::prelude::*;

fn main() {
    println!("hello world");

    let x = env!("CARGO_PKG_AUTHORS");
    println!("{}", x);


}