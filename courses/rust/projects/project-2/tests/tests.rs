use tempfile::TempDir;
use kvs::{KvStore, Result};

// `kvs` with no args should exit with a non-zero code.
#[test]
fn cli_no_args() {

}

// `kvs -V` should print the version
#[test]
fn cli_version() {

}



// Should get previously stored value.
#[test]
fn get_stored_value() -> Result<()> {
    let temp_dir = TempDir::new().expect("unable to create temporary working directory");
    let mut store = KvStore::open(temp_dir.path())?;

    store.set("key1".to_owned(), "value1".to_owned())?;
    store.set("key2".to_owned(), "value2".to_owned())?;
    store.set("hello".to_owned(), "world".to_owned())?;




    Ok(())




}