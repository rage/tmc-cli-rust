use crate::{
    commands::util,
    config::TmcCliConfig,
    io::{Io, PrintColor},
};
use anyhow::Context;
use std::{cmp::Ordering, collections::HashMap, path::Path};
use tmc_langs::{RunResult, RunStatus};

/// Executes tmc tests for one exercise. If path not given, check if current folder is an exercise.
/// If not, asks exercise with an interactive menu.
#[tracing::instrument(skip_all)]
pub fn test(io: &mut Io, path: Option<&str>, config: &TmcCliConfig) -> anyhow::Result<()> {
    // todo: use context
    let exercise_path =
        util::exercise_pathfinder(path, config).context("Error finding exercise")?;
    test_exercise(io, &exercise_path, true)?;
    Ok(())
}

/// Executes tests for a single exercise, returns true if all tests passed (false if not).
fn test_exercise(io: &mut Io, path: &Path, print_progress: bool) -> anyhow::Result<bool> {
    // Get exercise folder name from last component in path
    let mut exercise_name = "";
    for component in path.components() {
        if let Some(component_string) = component.as_os_str().to_str() {
            exercise_name = component_string;
        }
    }

    let run_result = tmc_langs::run_tests(path)?;
    let res = print_result_test(io, run_result, exercise_name, print_progress)?;
    Ok(res)
}

/// Prints the result of running tests for a single exercise
fn print_result_test(
    io: &mut Io,
    run_result: RunResult,
    exercise_name: &str,
    print_progress: bool,
) -> anyhow::Result<bool> {
    io.println("", PrintColor::Normal)?;
    io.println(&format!("Testing: {exercise_name}"), PrintColor::Normal)?;

    let mut tests_passed = 0;
    let mut tests_total = 0;
    for test_result in run_result.test_results {
        tests_total += 1;
        if test_result.successful {
            tests_passed += 1;
        }

        if !test_result.successful {
            io.println(
                &format!("Failed '{}'", test_result.name),
                PrintColor::Failed,
            )?;
            io.println(&format!("\t{}", test_result.message), PrintColor::Normal)?;
            io.println("", PrintColor::Normal)?;
        }
    }

    io.println("", PrintColor::Normal)?;

    match run_result.status {
        RunStatus::Passed => {
            if tests_total == 0 {
                io.println(
                    "No tests found. Submit to server with 'tmc submit'",
                    PrintColor::Success,
                )?;
                if print_progress {
                    io.println(&util::get_progress_string(1, 1, 64), PrintColor::Normal)?;
                }
            } else {
                io.println(
                    &format!("Test results: {tests_passed}/{tests_total} tests passed"),
                    PrintColor::Success,
                )?;
                io.println(
                    "All tests passed! Submit to server with 'tmc submit'",
                    PrintColor::Success,
                )?;
                if print_progress {
                    io.println(
                        &util::get_progress_string(tests_passed, tests_total, 64),
                        PrintColor::Normal,
                    )?;
                }
            }
            return Ok(true);
        }
        RunStatus::TestsFailed => {
            io.println(
                &format!("Test results: {tests_passed}/{tests_total} tests passed"),
                PrintColor::Normal,
            )?;
            if print_progress {
                io.println(
                    &util::get_progress_string(tests_passed, tests_total, 64),
                    PrintColor::Normal,
                )?;
            }
        }
        RunStatus::CompileFailed => {
            print_logs(io, &run_result.logs)?;
            io.println(
                "Compilation failed, unable to run the tests. \
                The logs for the test process are printed above, if any.",
                PrintColor::Failed,
            )?;
        }
        RunStatus::TestrunInterrupted => {
            print_logs(io, &run_result.logs)?;
            io.println(
                "The test run was interrupted. \
                The logs for the test process are printed above, if any.",
                PrintColor::Failed,
            )?;
        }
        RunStatus::GenericError => {
            print_logs(io, &run_result.logs)?;
            io.println(
                "An unexpected error occurred. \
                The logs for the test process are printed above, if any.",
                PrintColor::Failed,
            )?;
        }
    }
    Ok(false)
}

fn print_logs(io: &mut Io, logs: &HashMap<String, String>) -> anyhow::Result<()> {
    let mut logs = logs
        .iter()
        // filter out empty log entries to reduce clutter
        .filter(|(_, logs)| !logs.is_empty())
        .collect::<Vec<_>>();
    // sort stdout first, then stderr, then everything else in alphabetical order
    logs.sort_by(|a, b| match (a.0.as_str(), b.0.as_str()) {
        ("stdout", _) => Ordering::Greater,
        ("stderr", _) => Ordering::Greater,
        (a, b) => a.cmp(b),
    });
    let logs = logs
        .into_iter()
        .map(|(key, logs)| format!("{key}:\n{logs}"))
        .collect::<Vec<_>>()
        .join("\n\n");
    io.println(
        &format!("Logs from the test process:\n{logs}"),
        PrintColor::Normal,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, io::Cursor};
    use termcolor::NoColor;
    use tmc_langs::{RunResult, RunStatus, TestResult};

    #[test]
    fn generate_progress_string_empty_test() {
        let progress_string = util::get_progress_string(0, 100, 64);
        let expected_string =
            String::from("   0%[░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░]");

        assert_eq!(progress_string, expected_string);
    }

    #[test]
    fn generate_progress_string_empty_2_test() {
        let progress_string = util::get_progress_string(0, 1, 64);
        let expected_string =
            String::from("   0%[░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░]");

        assert_eq!(progress_string, expected_string);
    }

    #[test]
    fn generate_progress_string_full_test() {
        let progress_string = util::get_progress_string(100, 100, 64);
        let expected_string =
            String::from(" 100%[████████████████████████████████████████████████████████████████]");

        assert_eq!(progress_string, expected_string);
    }

    #[test]
    fn generate_progress_string_full_2_test() {
        let progress_string = util::get_progress_string(1, 1, 64);
        let expected_string =
            String::from(" 100%[████████████████████████████████████████████████████████████████]");

        assert_eq!(progress_string, expected_string);
    }

    #[test]
    fn generate_progress_string_50_50_test() {
        let progress_string = util::get_progress_string(1, 2, 64);
        let expected_string =
            String::from("  50%[████████████████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░]");

        assert_eq!(progress_string, expected_string);
    }
    #[test]
    fn generate_progress_string_50_50_2_test() {
        let progress_string = util::get_progress_string(50, 100, 64);
        let expected_string =
            String::from("  50%[████████████████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░]");

        assert_eq!(progress_string, expected_string);
    }

    #[test]
    fn generate_progress_string_30_70_test() {
        let progress_string = util::get_progress_string(3, 10, 64);
        let expected_string =
            String::from("  30%[███████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░]");

        assert_eq!(progress_string, expected_string);
    }

    #[test]
    fn generate_progress_string_79_21_test() {
        let progress_string = util::get_progress_string(79, 100, 64);
        let expected_string =
            String::from("  79%[██████████████████████████████████████████████████░░░░░░░░░░░░░░]");

        assert_eq!(progress_string, expected_string);
    }

    fn assert_contains(line: &str, contains: &str) {
        assert!(
            line.contains(contains),
            "Line '{line}' did not contain '{contains}'"
        );
    }

    #[test]
    fn print_test_results_no_tests() {
        let mut output = NoColor::new(Vec::<u8>::new());
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut io = Io::new(&mut output, &mut input);

        let logs: HashMap<String, String> = HashMap::new();
        let run_result = RunResult::new(RunStatus::Passed, vec![], logs);
        let exercise_name = "my_test_exercise";

        let all_tests_passed = print_result_test(&mut io, run_result, exercise_name, true).unwrap();

        let output = String::from_utf8(output.into_inner()).unwrap();
        let output = output.lines().collect::<Vec<_>>();
        assert_eq!(output[0], "");

        assert_contains(output[1], "Testing");
        assert_contains(output[1], exercise_name);
        assert_eq!(output[2], "");
        assert_contains(output[3], "No tests found");
        assert_contains(output[4], "█");
        assert_contains(output[4], "100%");
        assert!(
            all_tests_passed,
            "print_result_test returned false, expected true"
        );
    }

    #[test]
    fn print_test_result_with_passed_tests_test() {
        let mut output = NoColor::new(Vec::<u8>::new());
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut io = Io::new(&mut output, &mut input);

        let test_result_completed = TestResult {
            name: "the_first_test".to_string(),
            successful: true,
            points: vec!["point1".to_string(), "point2".to_string()],
            message: "".to_string(),
            exception: vec![],
        };

        let logs: HashMap<String, String> = HashMap::new();
        let test_results = vec![test_result_completed];
        let run_result = RunResult::new(RunStatus::Passed, test_results, logs);
        let exercise_name = "my_test_exercise";

        let all_tests_passed = print_result_test(&mut io, run_result, exercise_name, true).unwrap();

        let output = String::from_utf8(output.into_inner()).unwrap();
        let output = output.lines().collect::<Vec<_>>();
        assert_eq!(output[0], "");

        assert_contains(output[1], "Testing");
        assert_contains(output[1], exercise_name);
        assert_eq!(output[2], "");
        assert_contains(output[3], "Test results");
        assert_contains(output[3], "1/1");

        assert_contains(output[4], "All tests passed");
        assert_contains(output[4], "tmc submit");

        assert_contains(output[5], "█");
        assert_contains(output[5], "100%");
        assert!(
            all_tests_passed,
            "print_result_test returned false, expected true"
        );
    }

    #[test]
    fn print_test_result_with_failed_tests_test() {
        let mut output = NoColor::new(Vec::<u8>::new());
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut io = Io::new(&mut output, &mut input);

        let test_result_message = "The test seems to have failed";
        let test_result_completed = TestResult {
            name: "the_first_test".to_string(),
            successful: false,
            points: vec!["point1".to_string()],
            message: test_result_message.to_string(),
            exception: vec![],
        };

        let logs: HashMap<String, String> = HashMap::new();
        let test_results = vec![test_result_completed];
        let run_result = RunResult::new(RunStatus::TestsFailed, test_results, logs);
        let exercise_name = "my_test_exercise";

        let all_tests_passed = print_result_test(&mut io, run_result, exercise_name, true).unwrap();

        let output = String::from_utf8(output.into_inner()).unwrap();
        let output = output.lines().collect::<Vec<_>>();
        assert_eq!(output[0], "");

        assert_contains(output[1], "Testing");
        assert_contains(output[1], exercise_name);
        assert_contains(output[2], "Failed");
        assert_contains(output[3], test_result_message);
        assert_contains(output[6], "Test results");
        assert_contains(output[6], "0/1");
        assert_contains(output[7], "░");
        assert_contains(output[7], " 0%");
        assert!(
            !all_tests_passed,
            "print_result_test returned true, expected false",
        );
    }
}
