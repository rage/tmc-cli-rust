use crate::io_module::Io;
use std::env;
use std::path::{Path, PathBuf};
use tmc_langs_framework::domain::RunResult;
use tmc_langs_util::task_executor::{
    find_exercise_directories, is_exercise_root_directory, run_tests,
};

/// Executes tmc tests for exercise(s)
pub fn test(io: &mut dyn Io, exercise_folder: Option<&str>) {
    let status = match env::current_dir() {
        Ok(mut pathbuf) => match exercise_folder {
            Some(exercise) => {
                // Specific exercise folder was given as an argument, so we only do tests for it.
                pathbuf.push(exercise);
                let path = pathbuf.as_path();
                if is_exercise_root_directory(path) {
                    test_exercise_path(io, path)
                } else {
                    Err("Specified folder is not an exercise".to_string())
                }
            }
            None => {
                let path = pathbuf.as_path();
                // If current directory is an excercise, its tests will be done.
                if is_exercise_root_directory(path) {
                    test_exercise_path(io, path)
                } else {
                    // Otherwise we will find exercises under this directory recursively.
                    match find_exercise_directories(path) {
                        Ok(exercises) => test_exercises(io, exercises),
                        Err(error) => Err(error.to_string()),
                    }
                }
            }
        },
        Err(error) => Err(format!(
            "Invalid directory / Insufficient permissions: {}",
            error
        )),
    };

    if let Err(err) = status {
        io.println(&err);
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

/// Wrapper around test_exercise funtion to get uniform Result type
fn test_exercise_path(io: &mut dyn Io, path: &Path) -> Result<(), String> {
    if let Err(err) = test_exercise(io, path, true) {
        Err(err)
    } else {
        Ok(())
    }
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

    match run_tests(path) {
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

/// Prints the result of running tests for a single test
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

/// Returns a progress bar of size 'length' based on percentage of 'passed' / 'total'
fn get_progress_string(completed: usize, total: usize, length: usize) -> String {
    let completed_proportion = completed as f32 / total as f32;
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
