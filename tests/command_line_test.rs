use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn hello() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tmc-cli-rust")?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello"));

    Ok(())
}

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tmc-cli-rust")?;
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Test My Code client written in Rust",
    ));

    Ok(())
}

#[test]
fn wrong_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tmc-cli-rust")?;
    cmd.arg("--wrong_argument");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: Found argument '--wrong_argument' which wasn't expected, or isn't valid in this context"));

    Ok(())
}
