use std::path::PathBuf;

fn main() {

}

#[test]
fn test_path_buf() {

    // An owned, mutable path
    // This type provides methods like push and set_extension that mutate the path in place
    // It also implements Deref to Path, meaning that all methods on Path slices are availabe on
    // PathBuf values as well
    let mut path = PathBuf::new();

    path.push(r"C:\");
    path.push("windows");
    path.push("system32");



}