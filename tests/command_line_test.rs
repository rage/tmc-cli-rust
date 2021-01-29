use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn command_hello() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tmc-cli-rust")?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello"));

    Ok(())
}

#[test]
fn command_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tmc-cli-rust")?;
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Test My Code client written in Rust",
    ));

    Ok(())
}

#[test]
fn command_wrong_argument_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tmc-cli-rust")?;
    cmd.arg("--wrong_argument");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: Found argument '--wrong_argument' which wasn't expected, or isn't valid in this context"));

    Ok(())
}

#[test]
fn command_version() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tmc-cli-rust")?;
    cmd.arg("--version");
    cmd.assert().success().stdout(predicate::str::contains(
        "Test My Code client written in Rust 0.1.0",
    ));

    Ok(())
}

#[test]
fn command_test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tmc-cli-rust")?;
    cmd.arg("test").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("tmc-cli-rust test"));

    Ok(())
}
