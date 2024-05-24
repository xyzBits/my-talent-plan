use std::process::Command;
use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use tempfile::TempDir;

// `kvs-client` with no args should with a non-zero code
#[test]
fn client_cli_no_args() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("kvs-client").unwrap();
    cmd.current_dir(&temp_dir).assert().failure();

}
