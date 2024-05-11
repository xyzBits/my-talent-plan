use std::env;

fn main() {
    println!("hello world");

    let x = env!("CARGO_PKG_AUTHORS");
    println!("{}", x);

    for (key, value) in env::vars() {
        if key.starts_with("CARGO") {
            println!("key={key}, value={value}");

        }
    }


}