use assert_cmd::Command;
use predicates::prelude::*;
use std::collections::HashMap;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[test]
fn command_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut envs = HashMap::new();
    envs.insert("RUST_LOG", "debug".to_string());

    let mut cmd = command(&envs);
    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Client for downloading, testing and submitting exercises through the TestMyCode and MOOC.fi systems.",
    ));

    Ok(())
}

#[test]
fn command_wrong_argument_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut envs = HashMap::new();
    envs.insert("RUST_LOG", "debug".to_string());

    let mut cmd = command(&envs);
    cmd.arg("--wrong_argument");
    cmd.assert().failure().stderr(predicate::str::contains(
        "error: unexpected argument '--wrong_argument' found",
    ));

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

    let config_dir = tempfile::tempdir().unwrap();
    let server = mockito::Server::new();

    let mut envs = HashMap::new();
    envs.insert(
        "TMC_LANGS_CONFIG_DIR",
        config_dir.path().as_os_str().to_str().unwrap().to_string(),
    );
    envs.insert("TMC_LANGS_ROOT_URL", server.url());
    envs.insert("RUST_LOG", "debug".to_string());

    // logout
    let mut cmd = command(&envs);
    cmd.arg("--testmode").envs(&envs).arg("logout");
    cmd.assert().success();

    // download -c test-tmc-test-course -f folder_for_download
    cmd = command(&envs);
    cmd.arg("--testmode")
        .arg("download")
        .arg("-c")
        .arg("test-tmc-test-course");
    cmd.assert().success().stderr(predicate::str::contains(
        "No login found. Login to use this command with 'tmc login'",
    ));

    // courses
    cmd = command(&envs);
    cmd.arg("--testmode")
        .arg("courses")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "No login found. Login to use this command with 'tmc login'",
        ));

    // submit
    cmd = command(&envs);
    cmd.arg("--testmode")
        .arg("submit")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "No login found. Login to use this command with 'tmc login'",
        ));

    // paste
    cmd = command(&envs);
    cmd.arg("--testmode")
        .arg("paste")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "No login found. Login to use this command with 'tmc login'",
        ));

    // exercises
    cmd = command(&envs);
    cmd.arg("--testmode")
        .arg("exercises")
        .arg("test-tmc-test-course");
    cmd.assert().success().stderr(predicate::str::contains(
        "No login found. Login to use this command with 'tmc login'",
    ));

    // login
    // testusername testpassword imag
    cmd = command(&envs);
    cmd.arg("--testmode")
        .arg("login")
        .write_stdin("testusername\ntestpassword\nimag\n")
        .assert()
        .success()
        .stderr(predicate::str::contains("Logged in successfully!"));

    // organization -n
    // test
    let mut cmd = command(&envs);
    cmd.arg("--testmode")
        .arg("organization")
        .arg("-n")
        .write_stdin("test\n")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Selected test organization as organization.",
        ));

    // courses
    cmd = command(&envs);
    cmd.arg("--testmode").arg("courses");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("test-tmc-test-course"));

    // exercises test-tmc-test-course
    cmd = command(&envs);
    cmd.arg("--testmode")
        .arg("exercises")
        .arg("test-tmc-test-course");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Imaginary test exercise"));

    // download -c test-tmc-test-course -f folder_for_download
    cmd = command(&envs);
    cmd.arg("--testmode")
        .arg("download")
        .arg("-c")
        .arg("test-tmc-test-course");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Download was successful!"));

    // test folder/nonexistant_ex
    cmd = command(&envs);
    cmd.arg("--testmode")
        .arg("test")
        .arg("folder/nonexistant_ex");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Failed to load projects config"));

    // logout
    let mut cmd = command(&envs);
    cmd.arg("--testmode").arg("logout");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Logged out successfully"));

    // login
    // totallywrongname cantrememberpasswordeither imag
    cmd = command(&envs);
    cmd.arg("--testmode")
        .arg("login")
        .write_stdin("totallywrongname\ncantrememberpasswordeither\nimag\n")
        .assert()
        .success()
        .stderr(predicate::str::contains("Wrong username or password"));

    Ok(())
}

fn command(envs: &HashMap<&str, String>) -> Command {
    let mut command = Command::cargo_bin(PKG_NAME).unwrap();
    command.envs(envs);
    command
}
