use crate::io_module::Io;
use std::env;
use std::path::{Path, PathBuf};
use tmc_langs_util::task_executor::{
    find_exercise_directories, is_exercise_root_directory, run_tests,
};
use tmc_langs_util::RunStatus;

pub fn test(io: &mut dyn Io, exercise: Option<&str>) -> Result<(), String> {
    match env::current_dir() {
        Ok(mut pathbuf) => match exercise {
            Some(ex) => {
                pathbuf.push(ex);
                // If a specific exercise was given as an argument, we only do tests for it.
                if is_exercise_root_directory(pathbuf.as_path()) {
                    test_exercise(io, pathbuf.as_path())
                } else {
                    Err("Specified folder is not an exercise".to_string())
                }
            }
            None => {
                // If current directory is an excercise, its tests will be done.
                if is_exercise_root_directory(pathbuf.as_path()) {
                    test_exercise(io, pathbuf.as_path())
                } else {
                    // Otherwise we will find exercises under this directory recursively.
                    match find_exercise_directories(pathbuf.as_path()) {
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
    }
}

fn test_exercises(io: &mut dyn Io, paths: Vec<PathBuf>) -> Result<(), String> {
    for exercise_path in paths {
        io.println("");
        io.println("-----");
        if let Err(err) = test_exercise(io, exercise_path.as_path()) {
            // TODO: what to do with results?
            // Exiting on first fail might not be the best idea
            return Err(err);
        }
    }

    Ok(())
}

fn test_exercise(io: &mut dyn Io, path: &Path) -> Result<(), String> {
    // TODO: A more specific layout to print the results in
    io.println("");
    //let errors = &mut vec![];
    match run_tests(path) {
        Ok(run_result) => {
            match run_result.status {
                RunStatus::Passed => {
                    io.println("[RunStatus] passed");
                }
                RunStatus::TestsFailed => {
                    io.println("[RunStatus] TestsFailed");
                }
                RunStatus::CompileFailed => {
                    io.println("[RunStatus] CompileFailed");
                }
                RunStatus::TestrunInterrupted => {
                    io.println("[RunStatus] TestrunInterrupted");
                }
                RunStatus::GenericError => {
                    io.println("[RunStatus] GenericError");
                }
            }
            io.println("");

            for test_result in run_result.test_results {
                io.println(&format!(
                    "[TestResult] {} {}",
                    test_result.name,
                    if !test_result.successful {
                        "failed"
                    } else {
                        "ran successfully"
                    }
                ));
                io.println(&test_result.message);

                io.print("Points: ");
                for point in test_result.points {
                    io.print(&format!("{}, ", point));
                }
                io.println("");
                io.println("");
            }
        }
        Err(error) => return Err(error.to_string()), // For example "Error in plugin" on a python exercise, if python wasn't installed on the system
    }

    Ok(())
}
