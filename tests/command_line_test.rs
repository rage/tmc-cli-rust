use assert_cmd::Command;
use predicates::prelude::*;

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

        All tests ran like this: tmc --testmode --no-update command [flags & args]

        logout
        download
        courses
        submit
        paste
        exercises
        login -n
            testusername
            testpassword
            imagorganization -n
            testcourses
        exercises test-tmc-test-course
        download -c test-tmc-test-course -f folder_for_download
        test folder/nonexistant_ex
        logout
        login
            totallywrongname
            cantrememberpasswordeither
            imag
    */

    // logout
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode").arg("--no-update").arg("logout");
    cmd.assert();

    // download -c test-tmc-test-course -f folder_for_download
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("download")
        .arg("-c")
        .arg("test-tmc-test-course");
    cmd.assert().success().stdout(predicate::str::contains(
        "No login found. Login to use this command with 'tmc login'",
    ));

    // courses
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("courses")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No login found. Login to use this command with 'tmc login'",
        ));

    // submit
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("submit")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No login found. Login to use this command with 'tmc login'",
        ));

    // paste
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("paste")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No login found. Login to use this command with 'tmc login'",
        ));

    // exercises
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("exercises")
        .arg("test-tmc-test-course");
    cmd.assert().success().stdout(predicate::str::contains(
        "No login found. Login to use this command with 'tmc login'",
    ));

    // login -n
    // testusername testpassword imag
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("login")
        .arg("-n")
        .write_stdin("testusername\ntestpassword\nimag\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Choose organization by writing its slug: ",
        ));

    // organization -n
    // test
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("organization")
        .arg("-n")
        .write_stdin("test\n")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Selected test organization as organization.",
        ));

    // courses
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode").arg("--no-update").arg("courses");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test-tmc-test-course"));

    // exercises test-tmc-test-course
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("exercises")
        .arg("test-tmc-test-course");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Imaginary test exercise"));

    // download -c test-tmc-test-course -f folder_for_download
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("download")
        .arg("-c")
        .arg("test-tmc-test-course");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Download was successful!"));

    // test folder/nonexistant_ex
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("test")
        .arg("folder/nonexistant_ex");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Invalid exercise path given"));

    // logout
    let mut cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode").arg("--no-update").arg("logout");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Logged out successfully"));

    // login
    // totallywrongname cantrememberpasswordeither imag
    cmd = Command::cargo_bin(PKG_NAME.unwrap())?;
    cmd.arg("--testmode")
        .arg("--no-update")
        .arg("login")
        .write_stdin("totallywrongname\ncantrememberpasswordeither\nimag\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Wrong username or password"));

    Ok(())
}
