use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tmc-cli-ohtuprojekti")?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello"));

    Ok(())
}
