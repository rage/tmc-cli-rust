use super::command_util::*;
use crate::io_module::Io;

use tmc_client::{ClientError, CourseExercise};

pub fn list_exercises(io: &mut dyn Io, client: &mut Client, course_name: String) {

    if let Err(error) = client.load_login() {
        io.println(error);
        return;
    };

    // Get course by id
    let course_result = get_course_id_by_name(client, course_name.clone());
    if course_result.is_none() {
        io.println("Could not find course by name".to_string());
        return;
    }
    let course_id = course_result.unwrap();

    match client.get_course_exercises(course_id) {
        Ok(exercises) => print_exercises(io, course_name, exercises),
        // TODO: Get a more detailed error from get_course_exercises and print it
        _ => io.println("Failed to download course exercises".to_string()),
    }
}

fn print_exercises(io: &mut dyn Io, course_name: String, exercises: Vec<CourseExercise>) {
    // Print exercises
    io.println("".to_string());
    io.print("Course name: ".to_string());
    io.println(course_name);

    let mut prev_deadline = "".to_string();
    for exercise in exercises {
        // Print deadline if it exists
        if let Some(dl) = exercise.deadline {
            if prev_deadline != dl {
                io.println(format!("Deadline: {}", &dl));
                prev_deadline = dl;
            }
        } else if let Some(dl) = exercise.soft_deadline {
            if prev_deadline != dl {
                io.println(format!("Soft deadline: {}", &dl));
                prev_deadline = dl;
            }
        }

        let mut completed = true;
        let mut attempted = false;

        for point in exercise.available_points {
            if !exercise.awarded_points.contains(&point.name) {
                completed = false;
            } else {
                attempted = true;
            }
        }

        let completion_status = if completed {
            "Completed"
        } else if attempted {
            "Attempted"
        } else {
            "Not completed"
        };

        io.println(format!("  {}: {}", completion_status, &exercise.name));
    }
}
