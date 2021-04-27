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
    /*
        Basic test "pipeline" to run all commands to quickly test if anything obvious has broken.
        Update this comment if you change/add tests in this function.
        Write also what input command is given. Run logout first to remove possible old test-login.

        tmc-cli-rust --testmode --no-update logout
        tmc-cli-rust --testmode --no-update login -n
            testusername
            testpassword
            imag
        tmc-cli-rust --testmode --no-update organization -n
            test
        tmc-cli-rust --testmode --no-update courses
        tmc-cli-rust --testmode --no-update exercises test-tmc-test-course
        tmc-cli-rust --testmode --no-update download -c test-tmc-test-course -f folder_for_download
        tmc-cli-rust --testmode --no-update test folder/nonexistant_ex
        tmc-cli-rust --testmode --no-update logout
        tmc-cli-rust --testmode --no-update login
            totallywrongname
            cantrememberpasswordeither
            imag
    */

    // tmc-cli-rust --testmode --no-update logout
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode").arg("--no-update").arg("logout");
    cmd.assert();

    // tmc-cli-rust --testmode --no-update login -n
    // testusername testpassword imag
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("login")
        .arg("-n")
        .with_stdin()
        .buffer("testusername\ntestpassword\nimag\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Choose organization by writing its slug: ",
        ));

    // tmc-cli-rust --testmode --no-update organization -n
    // test
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("organization")
        .arg("-n")
        .with_stdin()
        .buffer("test\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Selected test organization as organization.",
        ));

    // tmc-cli-rust --testmode --no-update courses
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode").arg("--no-update").arg("courses");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test-tmc-test-course"));

    // tmc-cli-rust --testmode --no-update exercises test-tmc-test-course
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("exercises")
        .arg("test-tmc-test-course");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Imaginary test exercise"));

    // tmc-cli-rust --testmode --no-update download -c test-tmc-test-course -f folder_for_download
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("download")
        .arg("-c")
        .arg("test-tmc-test-course");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Download was successful!"));

    // tmc-cli-rust --testmode --no-update test folder/nonexistant_ex
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("test")
        .arg("folder/nonexistant_ex");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Invalid exercise path given"));

    // tmc-cli-rust --testmode --no-update logout
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode").arg("--no-update").arg("logout");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Logged out successfully"));

    // tmc-cli-rust --testmode --no-update login
    // totallywrongname cantrememberpasswordeither imag
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("login")
        .with_stdin()
        .buffer("totallywrongname\ncantrememberpasswordeither\nimag\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Wrong username or password"));

    Ok(())
}
