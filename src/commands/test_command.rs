use crate::io_module::Io;
use std::env;
use std::path::{Path, PathBuf};
use tmc_langs::RunResult;

/// Executes tmc tests for exercise(s)
pub fn test(io: &mut dyn Io, exercise_folder: Option<&str>) {
    let status = match env::current_dir() {
        Ok(mut pathbuf) => {
            if let Some(exercise) = exercise_folder {
                pathbuf.push(exercise);
            }

            match tmc_langs::find_exercise_directories(pathbuf.as_path()) {
                Ok(exercises) => match exercises.len() {
                    0 => Err("No exercises found.".to_string()),
                    1 => test_exercise_path(io, exercises[0].as_path()),
                    _ => test_exercises(io, exercises),
                },
                Err(error) => Err(format!("No exercises found: {}", error)),
            }
        }
        Err(error) => Err(format!(
            "Invalid directory / Insufficient permissions: {}",
            error
        )),
    };

    if let Err(err) = status {
        io.println(&err);
    }
}

/// Wrapper around test_exercise funtion to get uniform Result type
fn test_exercise_path(io: &mut dyn Io, path: &Path) -> Result<(), String> {
    if let Err(err) = test_exercise(io, path, true) {
        Err(err)
    } else {
        Ok(())
    }
}
/// Executes tmc tests for multiple exercises
fn test_exercises(io: &mut dyn Io, paths: Vec<PathBuf>) -> Result<(), String> {
    let mut exercises_completed = 0_usize;
    let mut exercises_total = 0_usize;
    for exercise_path in paths {
        match test_exercise(io, exercise_path.as_path(), false) {
            Ok(passed) => {
                exercises_total += 1;
                if passed {
                    exercises_completed += 1;
                }
            }
            Err(err) => return Err(err), // Stops iteration on first error
        }
    }

    print_result_tests(io, exercises_completed, exercises_total);
    Ok(())
}

/// Executes tests for a single exercise, returns true if all tests passed (false if not).
fn test_exercise(io: &mut dyn Io, path: &Path, print_progress: bool) -> Result<bool, String> {
    // Get exercise folder name from last component in path
    let mut exercise_name = "";
    for component in path.components() {
        if let Some(component_string) = component.as_os_str().to_str() {
            exercise_name = component_string;
        }
    }

    match tmc_langs::run_tests(path) {
        Ok(run_result) => Ok(print_result_test(
            io,
            run_result,
            exercise_name,
            print_progress,
        )),
        Err(error) => Err(error.to_string()), // For example "Error in plugin" on a python exercise, if python wasn't installed on the system
    }
}

/// Prints the end result of running multiple tests
fn print_result_tests(io: &mut dyn Io, exercises_completed: usize, exercises_total: usize) {
    io.println("");
    io.println(&format!(
        "Total results: {}/{} exercises passed",
        exercises_completed, exercises_total
    ));
    io.println(&get_progress_string(
        exercises_completed,
        exercises_total,
        64,
    ));
}

/// Prints the result of running tests for a single exercise
fn print_result_test(
    io: &mut dyn Io,
    run_result: RunResult,
    exercise_name: &str,
    print_progress: bool,
) -> bool {
    io.println("");
    io.println(&format!("Testing: {}", exercise_name));

    let mut tests_passed = 0;
    let mut tests_total = 0;
    for test_result in run_result.test_results {
        tests_total += 1;
        if test_result.successful {
            tests_passed += 1;
        }

        if !test_result.successful {
            io.println(&format!("Failed '{}'", test_result.name));
            io.println(&format!("\t{}", test_result.message));
            io.println("");
        }
    }

    io.println("");
    io.println(&format!(
        "Test results: {}/{} tests passed",
        tests_passed, tests_total
    ));
    if tests_passed == tests_total {
        io.println("All tests passed! Submit to server with 'tmc submit'");
    }
    if print_progress {
        io.println(&get_progress_string(tests_passed, tests_total, 64));
    }
    tests_passed == tests_total
}

/// Returns a progress bar of size 'length' based on percentage of 'completed' / 'total'
fn get_progress_string(completed: usize, total: usize, length: usize) -> String {
    let completed_proportion = if total == 0 {
        1_f32
    } else {
        completed as f32 / total as f32
    };
    let completed_percentage_readable = (completed_proportion * 100_f32).floor() as usize;
    let progress_done = (completed_proportion * length as f32).floor() as usize;

    let mut progress_string = String::with_capacity(length);
    for _ in 0..progress_done {
        progress_string.push('█');
    }
    for _ in progress_done..length {
        progress_string.push('░');
    }

    let spaces = if completed_percentage_readable < 10 {
        "  "
    } else if completed_percentage_readable < 100 {
        " "
    } else {
        ""
    };
    format!(
        "{}{}%[{}]",
        spaces, completed_percentage_readable, progress_string
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::slice::Iter;
    use tmc_langs::RunResult;

    pub struct IoTest<'a> {
        list: &'a mut Vec<String>,
        input: &'a mut Iter<'a, &'a str>,
    }

    #[cfg(test)]
    impl IoTest<'_> {}

    #[cfg(test)]
    impl Io for IoTest<'_> {
        fn read_line(&mut self) -> String {
            match self.input.next() {
                Some(string) => string,
                None => "",
            }
            .to_string()
        }

        fn print(&mut self, output: &str) {
            print!("{}", output);
            self.list.push(output.to_string());
        }

        fn println(&mut self, output: &str) {
            println!("{}", output);
            self.list.push(output.to_string());
        }

        fn read_password(&mut self) -> String {
            self.read_line()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::HashMap;
        use tmc_langs_framework::RunStatus;
        use tmc_langs_framework::TestResult;

        #[test]
        fn generate_progress_string_empty_test() {
            let progress_string = get_progress_string(0, 100, 64);
            let expected_string = String::from(
                "  0%[░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░]",
            );

            assert_eq!(progress_string, expected_string);
        }

        #[test]
        fn generate_progress_string_empty_2_test() {
            let progress_string = get_progress_string(0, 1, 64);
            let expected_string = String::from(
                "  0%[░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░]",
            );

            assert_eq!(progress_string, expected_string);
        }

        #[test]
        fn generate_progress_string_full_test() {
            let progress_string = get_progress_string(100, 100, 64);
            let expected_string = String::from(
                "100%[████████████████████████████████████████████████████████████████]",
            );

            assert_eq!(progress_string, expected_string);
        }

        #[test]
        fn generate_progress_string_full_2_test() {
            let progress_string = get_progress_string(1, 1, 64);
            let expected_string = String::from(
                "100%[████████████████████████████████████████████████████████████████]",
            );

            assert_eq!(progress_string, expected_string);
        }

        #[test]
        fn generate_progress_string_50_50_test() {
            let progress_string = get_progress_string(1, 2, 64);
            let expected_string = String::from(
                " 50%[████████████████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░]",
            );

            assert_eq!(progress_string, expected_string);
        }
        #[test]
        fn generate_progress_string_50_50_2_test() {
            let progress_string = get_progress_string(50, 100, 64);
            let expected_string = String::from(
                " 50%[████████████████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░]",
            );

            assert_eq!(progress_string, expected_string);
        }

        #[test]
        fn generate_progress_string_30_70_test() {
            let progress_string = get_progress_string(3, 10, 64);
            let expected_string = String::from(
                " 30%[███████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░]",
            );

            assert_eq!(progress_string, expected_string);
        }

        #[test]
        fn generate_progress_string_79_21_test() {
            let progress_string = get_progress_string(79, 100, 64);
            let expected_string = String::from(
                " 79%[██████████████████████████████████████████████████░░░░░░░░░░░░░░]",
            );

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

            let all_tests_passed = print_result_test(&mut io, run_result, exercise_name, true);

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

            let all_tests_passed = print_result_test(&mut io, run_result, exercise_name, true);

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

            let all_tests_passed = print_result_test(&mut io, run_result, exercise_name, true);

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

        #[test]
        fn print_multiple_completed_tests_results_test() {
            let mut v: Vec<String> = Vec::new();
            let input = vec![];
            let mut input = input.iter();

            let mut io = IoTest {
                list: &mut v,
                input: &mut input,
            };

            print_result_tests(&mut io, 10, 10);

            assert_eq!(io.list[0], "");

            assert!(
                io.list[1].contains("Total results"),
                "line does not contain 'Total results'"
            );
            assert!(
                io.list[1].contains("10/10"),
                "line does not contain total completed exercises '10/10'"
            );
            assert!(
                io.list[2].contains("█"),
                "line does not contain progress char '█'"
            );
            assert!(
                !io.list[2].contains("░"),
                "line contains progress char '░', which should not appear at 100% completed"
            );
        }

        #[test]
        fn print_multiple_failed_tests_results_test() {
            let mut v: Vec<String> = Vec::new();
            let input = vec![];
            let mut input = input.iter();

            let mut io = IoTest {
                list: &mut v,
                input: &mut input,
            };

            print_result_tests(&mut io, 5, 10);

            assert_eq!(io.list[0], "");

            assert!(
                io.list[1].contains("Total results"),
                "line does not contain 'Total results'"
            );
            assert!(
                io.list[1].contains("5/10"),
                "line does not contain total completed exercises '10/10'"
            );
            assert!(
                io.list[2].contains("█"),
                "line does not contain progress char '█'"
            );
            assert!(
                io.list[2].contains("░"),
                "line does not contain progress char '░'"
            );
        }
    }
}
