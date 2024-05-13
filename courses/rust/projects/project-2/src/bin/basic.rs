use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use kvs::{KvStore, Result};

fn main() {

}

#[test]
fn test_path_buf() -> Result<()>  {

    // An owned, mutable path
    // This type provides methods like push and set_extension that mutate the path in place
    // It also implements Deref to Path, meaning that all methods on Path slices are availabe on
    // PathBuf values as well
    let mut path = PathBuf::new();

    path.push(r"C:\");
    path.push("windows");
    path.push("system32");

    let temp_dir = TempDir::new().expect("unable to create temporary working directory");


    let path = temp_dir.path();
    ///Users/dongfang/work/talent/tmp/hello
    let mut path = PathBuf::new();
    path.push("Users/dongfang/work/talent/tmp/hello");

    let store = KvStore::open(path)?;


    // fs::create_dir_all("/Users/dongfang/work/talent/tmp/hello")?;

    Ok(())

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[test]
    fn test_sorted_gen_list() {
        // Create a temporary directory for the test files
        let temp_dir = tempfile::tempdir().unwrap();

        // Create a temporary log file inside the temporary directory
        let test_file1 = NamedTempFile::new_in(&temp_dir).unwrap();
        let test_file2 = NamedTempFile::new_in(&temp_dir).unwrap();
        let test_file3 = NamedTempFile::new_in(&temp_dir).unwrap();

        // Get the path of the temporary directory
        let test_dir = temp_dir.path();

        // Rename the temporary log files to have .log extension
        let test_file_path1 = test_file1.path().with_extension("log");
        test_file1.persist(test_file_path1).unwrap();

        let test_file_path2 = test_file2.path().with_extension("log");
        test_file2.persist(test_file_path2).unwrap();

        let test_file_path3 = test_file3.path().with_extension("log");
        test_file3.persist(test_file_path3).unwrap();

        // Run the test on the temporary directory
        let result = kvs::sorted_gen_list(test_dir).unwrap();
        let expected_result = vec![100, 200, 300]; // Assuming the test files directory contains files named 100.log, 200.log, and 300.log

        assert_eq!(result, expected_result);
    }
}