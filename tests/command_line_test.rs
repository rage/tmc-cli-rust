use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Get the package name specified in Cargo.toml -> less to take care of in case the name needs to
// be changed
const PKG_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

#[test]
fn command_hello() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello"));

    Ok(())
}

#[test]
fn command_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Test My Code client written in Rust",
    ));

    Ok(())
}

#[test]
fn command_wrong_argument_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--wrong_argument");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: Found argument '--wrong_argument' which wasn't expected, or isn't valid in this context"));

    Ok(())
}

#[test]
fn command_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--version");
    cmd.assert().success().stdout(predicate::str::contains(
        "Test My Code client written in Rust 0.1.0",
    ));

    Ok(())
}

#[test]
fn command_test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("test").arg("--help");
    cmd.assert()
        .success()
        // check that the help info for command 'test' contains the words "tmc-cli-rust test"
        .stdout(predicate::str::contains("tmc-cli-rust test"));

    Ok(())
}
