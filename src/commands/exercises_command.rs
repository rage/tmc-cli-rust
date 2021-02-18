use super::command_util::*;
use crate::io_module::IO;
use tmc_client::TmcClient;

pub fn list_excercises(io: &mut IO, course_name: String) {
    // Login functinality
    let mut client = get_client();
    if !is_logged_in() {
        io.println("No login found. You need to be logged in to set organization.");
        return;
    }
    let credentials = get_credentials().unwrap();
    client.set_token(credentials.token()).unwrap();

    let slug = get_organization().unwrap();

    // Match course name to an id
    let mut course_id = 0;
    let mut found = false;
    let courses = client.list_courses(&slug).unwrap();
    for course in courses {
        if course.name == course_name {
            course_id = course.id;
            found = true;
            //break;
        }
    }
    if !found {
        io.println("Could not find course by name");
        return;
    }

    // Get and print exercises with course_id
    let exercises = client.get_course_exercises(course_id).unwrap();

    io.println("");
    io.print("Course name: ");
    io.println(course_name);

    let mut prev_deadline = "".to_string();
    for exercise in exercises {
        // Some duplicity
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

        let avail_points = exercise.available_points;
        let award_points = exercise.awarded_points;

        let mut completed = true;
        let mut attempted = false;
        for point in avail_points {
            if !award_points.contains(&point.name) {
                completed = false;
            } else {
                attempted = true;
            }
            /*io.print("        Point: ");
            io.print(&point.name);
            io.print(": ");
            io.println(award_points.contains(&point.name).to_string());*/
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
