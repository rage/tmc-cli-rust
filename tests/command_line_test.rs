use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn hello() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tmc-cli-ohtuprojekti")?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello"));

    Ok(())
}

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tmc-cli-ohtuprojekti")?;
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Test My Code client written in Rust"));

    Ok(())
}
