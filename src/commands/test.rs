use crate::{
    commands::util,
    io::{Io, PrintColor},
};
use anyhow::Context;
use std::path::Path;
use tmc_langs::RunResult;

/// Executes tmc tests for one exercise. If path not given, check if current folder is an exercise.
/// If not, asks exercise with an interactive menu.
pub fn test(io: &mut dyn Io, path: Option<&str>) -> anyhow::Result<()> {
    // todo: use context
    let exercise_path = util::exercise_pathfinder(path).context("Error finding exercise")?;
    test_exercise_path(io, &exercise_path)?;
    Ok(())
}

/// Wrapper around test_exercise funtion to get uniform Result type
fn test_exercise_path(io: &mut dyn Io, path: &Path) -> anyhow::Result<()> {
    test_exercise(io, path, true)?;
    Ok(())
}

/// Executes tests for a single exercise, returns true if all tests passed (false if not).
fn test_exercise(io: &mut dyn Io, path: &Path, print_progress: bool) -> anyhow::Result<bool> {
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
    io: &mut dyn Io,
    run_result: RunResult,
    exercise_name: &str,
    print_progress: bool,
) -> anyhow::Result<bool> {
    io.println("", PrintColor::Normal)?;
    io.println(&format!("Testing: {}", exercise_name), PrintColor::Normal)?;

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
    io.println(
        &format!(
            "Test results: {}/{} tests passed",
            tests_passed, tests_total
        ),
        PrintColor::Normal,
    )?;
    if tests_passed == tests_total {
        io.println(
            "All tests passed! Submit to server with 'tmc submit'",
            PrintColor::Success,
        )?;
    }
    if print_progress {
        io.println(
            &util::get_progress_string(tests_passed, tests_total, 64),
            PrintColor::Normal,
        )?;
    }
    Ok(tests_passed == tests_total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, slice::Iter};
    use tmc_langs::{RunResult, RunStatus, TestResult};

    pub struct IoTest<'a> {
        list: &'a mut Vec<String>,
        input: &'a mut Iter<'a, &'a str>,
    }

    impl Io for IoTest<'_> {
        fn read_line(&mut self) -> anyhow::Result<String> {
            let res = match self.input.next() {
                Some(string) => string,
                None => "",
            };
            Ok(res.to_string())
        }

        fn print(&mut self, output: &str, _font_color: PrintColor) -> anyhow::Result<()> {
            print!("{}", output);
            self.list.push(output.to_string());
            Ok(())
        }

        fn println(&mut self, output: &str, _font_color: PrintColor) -> anyhow::Result<()> {
            println!("{}", output);
            self.list.push(output.to_string());
            Ok(())
        }

        fn read_password(&mut self) -> anyhow::Result<String> {
            self.read_line()
        }
    }

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

    #[test]
    fn print_test_results_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec![];
        let mut input = input.iter();

        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };

        let logs: HashMap<String, String> = HashMap::new();
        let run_result = RunResult::new(RunStatus::Passed, vec![], logs);
        let exercise_name = "my_test_exercise";

        let all_tests_passed = print_result_test(&mut io, run_result, exercise_name, true).unwrap();

        assert_eq!(io.list[0], "");

        assert!(
            io.list[1].contains("Testing"),
            "line does not contain 'Testing'"
        );
        assert!(
            io.list[1].contains(exercise_name),
            "line does not contain exercise name"
        );
        assert_eq!(io.list[2], "");
        assert!(
            io.list[3].contains("Test results"),
            "line does not contain 'Test results'"
        );
        assert!(
            io.list[3].contains("0/0"),
            "line does not contain completed tests, should be '0/0' "
        );

        assert!(
            io.list[4].contains("All tests passed"),
            "line does not contain 'All tests passed'"
        );
        assert!(
            io.list[4].contains("tmc submit"),
            "line does not contain hint 'tmc submit'"
        );

        assert!(
            io.list[5].contains("█"),
            "line does not contain progress bar char '█'"
        );
        assert!(
            io.list[5].contains("100%"),
            "line does not contain progress '100%'"
        );
        assert!(
            all_tests_passed,
            "print_result_test returned false, expected true"
        );
    }

    #[test]
    fn print_test_result_with_passed_tests_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec![];
        let mut input = input.iter();

        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };

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

        assert_eq!(io.list[0], "");

        assert!(
            io.list[1].contains("Testing"),
            "line does not contain 'Testing'"
        );
        assert!(
            io.list[1].contains(exercise_name),
            "line does not contain exercise name"
        );
        assert_eq!(io.list[2], "");
        assert!(
            io.list[3].contains("Test results"),
            "line does not contain 'Test results'"
        );
        assert!(
            io.list[3].contains("1/1"),
            "line does not contain completed tests, should be '1/1' "
        );

        assert!(
            io.list[4].contains("All tests passed"),
            "line does not contain 'All tests passed'"
        );
        assert!(
            io.list[4].contains("tmc submit"),
            "line does not contain hint 'tmc submit'"
        );

        assert!(
            io.list[5].contains("█"),
            "line does not contain progress bar char '█'"
        );
        assert!(
            io.list[5].contains("100%"),
            "line does not contain progress '100%'"
        );
        assert!(
            all_tests_passed,
            "print_result_test returned false, expected true"
        );
    }

    #[test]
    fn print_test_result_with_failed_tests_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec![];
        let mut input = input.iter();

        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };

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
        let run_result = RunResult::new(RunStatus::Passed, test_results, logs);
        let exercise_name = "my_test_exercise";

        let all_tests_passed = print_result_test(&mut io, run_result, exercise_name, true).unwrap();

        assert_eq!(io.list[0], "");

        assert!(
            io.list[1].contains("Testing"),
            "line does not contain 'Testing'"
        );
        assert!(
            io.list[1].contains(exercise_name),
            "line does not contain exercise name"
        );
        assert!(
            io.list[2].contains("Failed"),
            "line does not contain 'Failed'"
        );
        assert!(
            io.list[3].contains(test_result_message),
            "line does not contain message from test_result"
        );

        assert!(
            io.list[6].contains("Test results"),
            "line does not contain 'Test results'"
        );
        assert!(
            io.list[6].contains("0/1"),
            "line does not contain completed tests, should be '0/1' "
        );
        assert!(
            io.list[7].contains("░"),
            "line does not contain progress bar char '█'"
        );
        assert!(
            io.list[7].contains(" 0%"),
            "line does not contain progress ' 0%'"
        );
        assert!(
            !all_tests_passed,
            "print_result_test returned true, expected false"
        );
    }
}
