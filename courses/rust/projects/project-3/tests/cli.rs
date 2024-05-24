use std::fmt::Debug;
use std::fs::File;
use std::process::Command;
use std::{fs, thread};
use std::time::Duration;

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use predicates::str::contains;
use tempfile::TempDir;

// `kvs-client` with no args should with a non-zero code
#[test]
fn client_cli_no_args() {
    let temp_dir = TempDir::new().unwrap();
    // cargo_bin Create a command to run a specific binary of the current crate
    let mut cmd = Command::cargo_bin("kvs-client").unwrap();

    // Sets the working directory for the child process
    cmd.current_dir(&temp_dir)

        // wrap with an interface for that provides assertions on the output
        .assert()
        // ensure the command failed
        .failure();
}

#[test]
fn test_ls() {
    Command::new("ls")
        .current_dir("/bin")
        .spawn()
        .expect("xxxx");
}


#[test]
fn client_cli_invalid_get() {
    let temp_dir = TempDir::new().unwrap();
    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["get"])
        .current_dir(&temp_dir)
        .assert()
        .failure();


    let res = Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["get"])
        // Executing the command as a child process and returning a handle to it
        .spawn();


    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["get", "extra", "field"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["get", "key", "--addr", "invalid-addr"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["get", "key", "--addr", "--unknown-flag"])
        .current_dir(&temp_dir)
        .assert()
        .failure();
}


#[test]
fn client_cli_invalid_set() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["set"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["set", "missing_field"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["set", "key", "value", "--addr", "invalid-addr"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["set", "key", "value", "extra_field"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["set", "key", "--unknown-flag"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

}

#[test]
fn client_cli_invalid_rm() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["rm"])
        .current_dir(&temp_dir)
        .assert()
        .failure();


    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["rm", "extra", "field"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["rm", "key", "--unknown-flag"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["rm", "key", "--addr", "invalid-addr"])
        .current_dir(&temp_dir)
        .assert()
        .failure();
}


#[test]
fn client_cli_invalid_subcommand() {
    let temp_dir = TempDir::new().unwrap();

    Command::cargo_bin("kvs-client")
        .unwrap()
        .args(&["unknown"])
        .current_dir(&temp_dir)
        .assert()
        .failure();

}

// `kvs-client -V` should print the version
#[test]
fn client_cli_version() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("kvs-client").unwrap();

    cmd.args(&["-V"])
        .current_dir(&temp_dir)
        .assert()
        // Ensure the command wrote the expected data to stdout
        .stdout(contains(env!("CARGO_PKG_VERSION")));

}


#[test]
fn cli_log_configuration() {
    let temp_dir = TempDir::new().unwrap();

    let stderr_path = temp_dir.path().join("stderr");

    let mut cmd = Command::cargo_bin("kvs-server").unwrap();

    let mut child = cmd
        .args(&["--engine", "kvs", "--addr", "127.0.0.1:4001"])
        .current_dir(&temp_dir)
        .stderr(File::create(&stderr_path).unwrap())
        .spawn()
        .unwrap();

    thread::sleep(Duration::from_secs(1));
    child.kill().expect("server exited before killed");

    let content = fs::read_to_string(&stderr_path).expect("unable to read from stderr file");

    println!("{}", content);

    assert!(content.contains(env!("CARGO_PKG_VERSION")));
    assert!(content.contains("kvs"));
    assert!(content.contains("127.0.0.1:4001"));

}
