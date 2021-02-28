use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

// Get the package name specified in Cargo.toml -> less to take care of in case the name needs to
// be changed
const PKG_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

#[test]
fn command_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--help").arg("--no-update");
    cmd.assert().success().stdout(predicate::str::contains(
        "Test My Code client written in Rust",
    ));

    Ok(())
}

#[test]
fn command_wrong_argument_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--wrong_argument").arg("--no-update");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error: Found argument '--wrong_argument' which wasn't expected, or isn't valid in this context"));

    Ok(())
}

#[test]
fn all_integration_tests() -> Result<(), Box<dyn std::error::Error>> {
    // Logout to make sure old login doesn't exist
    /*
        // tmc-cli-rust --testmode logout
        let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
        cmd.arg("--testmode")
        .arg("logout");


        // tmc-cli-rust --testmode login
        cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
        cmd.arg("--testmode")
        .arg("login")
        .with_stdin()
        .buffer("testusername\ntestpassword\ntest\n")
        .assert()
            .success()
            .stdout(predicate::str::contains("Imaginary test organization"));
    */
    // tmc-cli-rust --testmode organization
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("organization")
        .with_stdin()
        .buffer("test\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Selected test organization as organization.",
        ));

    // tmc-cli-rust --testmode courses
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode").arg("--no-update").arg("courses");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test-tmc-test-course"));

    // tmc-cli-rust --testmode exercises test-tmc-test-course
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("exercises")
        .arg("test-tmc-test-course");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Imaginary test exercise"));

    // tmc-cli-rust --testmode download test-tmc-test-course folder_for_download
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("download")
        .arg("test-tmc-test-course")
        .arg("folder_for_download");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Download was successful!"));

    Ok(())
}

/*
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
        .stdout(predicate::str::contains(
            String::from(PKG_NAME.unwrap()) + " test",
        ));

    Ok(())
}
*/
