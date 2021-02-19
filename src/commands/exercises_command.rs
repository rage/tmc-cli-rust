use super::command_util::*;
use crate::io_module::IO;

use tmc_client::{ClientError, CourseExercise};

pub fn list_excercises(io: &mut IO, course_name: String) {
    // Get a client that has credentials
    let client_result = get_logged_client();
    if client_result.is_none() {
        io.println("No login found. You need to be logged in to list exercises.");
        return;
    }
    let client = client_result.unwrap();

    // Get course by id
    let course_result = get_course_id_by_name(&client, course_name.clone());
    if course_result.is_none() {
        io.println("Could not find course by name");
        return;
    }
    let course_id = course_result.unwrap();

    match client.get_course_exercises(course_id) {
        Ok(exercises) => print_exercises(io, course_name, exercises),
        Err(ClientError::NotLoggedIn) => {
            io.println("Login token is invalid. Please try logging in again.")
        }
        _ => io.println("Unknown error. Please try again."),
    }
}

fn print_exercises(io: &mut IO, course_name: String, exercises: Vec<CourseExercise>) {
    // Print exercises
    io.println("");
    io.print("Course name: ");
    io.println(course_name);

    let mut prev_deadline = "".to_string();
    for exercise in exercises {
        // Print deadline if it exists
        if let Some(dl) = exercise.deadline {
            if prev_deadline != dl {
                io.print("Deadline: ");
                io.println(&dl);
                prev_deadline = dl;
            }
        } else if let Some(dl) = exercise.soft_deadline {
            if prev_deadline != dl {
                io.print("Soft deadline: ");
                io.println(&dl);
                prev_deadline = dl;
            }
        }

        // Print the status of an exercise
        let mut completed = true;
        let mut attempted = false;
        for point in exercise.available_points {
            if !exercise.awarded_points.contains(&point.name) {
                completed = false;
            } else {
                attempted = true;
            }
        }

        io.print("  ");
        if completed {
            io.print("Completed: ");
        } else if attempted {
            io.print("Attempted: ");
        } else {
            io.print("Not completed: ");
        }
        io.println(&exercise.name);
    }
}
