use super::util::{self, choose_course, Client};
use crate::io::{Io, PrintColor};
use tmc_langs::tmc::response::CourseExercise;

/// Lists exercises for a given course
pub fn list_exercises(
    io: &mut Io,
    client: &mut Client,
    course_name: Option<&str>,
) -> anyhow::Result<()> {
    let fetched_course_name;
    let name_select = if let Some(course_name) = course_name {
        course_name
    } else {
        fetched_course_name = choose_course(io, client)?;
        &fetched_course_name
    };
    let course = util::get_course_by_name(client, name_select)?
        .ok_or_else(|| anyhow::anyhow!("Could not find a course with name '{}'", name_select))?;

    let mut exercises = client.get_course_exercises(course.id)?;
    exercises.sort_unstable_by(|l, r| l.name.cmp(&r.name));
    print_exercises(io, name_select, &exercises)?;
    Ok(())
}

/// Prints information about given exercises
fn print_exercises(
    io: &mut Io,
    course_name: &str,
    exercises: &[CourseExercise],
) -> anyhow::Result<()> {
    io.println("", PrintColor::Normal)?;
    io.println(&format!("Course name: {course_name}"), PrintColor::Normal)?;

    let none = "none";
    let mut prev_deadline = "";
    let mut prev_soft_deadline = "";
    for exercise in exercises {
        // Skip locked and disabled exercises
        if exercise.disabled || !exercise.unlocked {
            continue;
        }

        // Print deadline if it exists
        if let Some(dl) = &exercise.deadline {
            if prev_deadline != dl {
                io.println(&format!("Deadline: {dl}"), PrintColor::Normal)?;
                prev_deadline = dl;
            }
        } else if prev_deadline != none {
            io.println(&format!("Deadline: {none}"), PrintColor::Normal)?;
            prev_deadline = none;
        }

        // TODO: Do we need soft deadline?
        if let Some(dl) = &exercise.soft_deadline {
            if prev_soft_deadline != dl {
                io.println(&format!("Soft deadline: {dl}"), PrintColor::Normal)?;
                prev_soft_deadline = dl;
            }
        } else if prev_soft_deadline != none {
            io.println(&format!("Soft deadline: {none}"), PrintColor::Normal)?;
            prev_soft_deadline = none;
        }

        let mut completed = true;
        let mut attempted = false;

        for point in &exercise.available_points {
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

        io.println(
            &format!("  {}: {}", completion_status, &exercise.name),
            PrintColor::Normal,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{Matcher, Mock, Server, ServerGuard};
    use std::io::Cursor;
    use termcolor::NoColor;
    use tmc_langs::tmc::response::CourseExercise;

    fn logging() {
        let _ = flexi_logger::Logger::try_with_env()
            .unwrap()
            .log_to_stdout()
            .start();
    }

    fn mock_server() -> (ServerGuard, Vec<Mock>) {
        let mut server = Server::new();
        let mut mocks = Vec::new();
        mocks.push(
            server
                .mock("GET", "/api/v8/core/org/test/courses")
                .match_query(Matcher::Any)
                .with_body(
                    serde_json::json!([
                        {
                            "id": 1,
                            "name": "course_name",
                            "title": "title",
                            "details_url": "example.com",
                            "unlock_url": "example.com",
                            "reviews_url": "example.com",
                            "comet_url": "example.com",
                            "spyware_urls": ["example.com"],
                        },
                    ])
                    .to_string(),
                )
                .create(),
        );
        mocks.push(
            server
                .mock("GET", "/api/v8/courses/1/exercises")
                .match_query(Matcher::Any)
                .with_body(
                    serde_json::json!([
                        {
                            "id": 1,
                            "available_points": [],
                            "awarded_points": [],
                            "name": "part01-01_example_exercise",
                            "disabled": false,
                            "unlocked": true,
                        },
                        {
                            "id": 2,
                            "available_points": [],
                            "awarded_points": [],
                            "name": "part02-03_example_valid",
                            "disabled": false,
                            "unlocked": true,
                        },
                    ])
                    .to_string(),
                )
                .create(),
        );
        (server, mocks)
    }

    #[test]
    fn list_exercises_test() {
        let mut output = NoColor::new(Vec::<u8>::new());
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut io = Io::new(&mut output, &mut input);

        let points = vec![
            //TODO: ExercisePoint is in private module
            /*ExercisePoint {
                id: 0,
                exercise_id: 0,
                name: "1.1".to_string(),
                requires_review: true,
            }*/
        ];
        let awarded_points = vec![/*"1.1".to_string()*/];

        let exercises = [CourseExercise {
            id: 0,
            available_points: points,
            awarded_points,
            name: "part01-01_example_exercise".to_string(),
            publish_time: None,
            solution_visible_after: None,
            deadline: None,
            soft_deadline: None,
            disabled: false,
            unlocked: true,
        }];

        print_exercises(&mut io, "course_name", &exercises).unwrap();
        let output = String::from_utf8(output.into_inner()).unwrap();
        let output = output.lines().collect::<Vec<_>>();
        assert!(output[0].eq(""), "first line should be empty");
        let course_string = "Course name: course_name";
        assert!(
            output[1].eq(course_string),
            "Expected '{}', got '{}'",
            course_string,
            output[1]
        );
        let deadline_string = "Deadline: none";
        let soft_deadline_string = "Soft deadline: none";
        assert!(
            output[2].eq(deadline_string),
            "Expected '{}', got '{}'",
            deadline_string,
            output[2]
        );
        assert!(
            output[3].eq(soft_deadline_string),
            "Expected '{}', got '{}'",
            soft_deadline_string,
            output[3]
        );

        let exercise_string = "  Completed: part01-01_example_exercise";
        assert!(
            output[4].eq(exercise_string),
            "Expected '{}', got '{}'",
            exercise_string,
            output[4]
        );
    }

    #[test]
    fn list_exercises_with_client_test() {
        logging();
        let (server, _mocks) = mock_server();

        let mut output = NoColor::new(Vec::<u8>::new());
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut io = Io::new(&mut output, &mut input);

        let mut client = Client::new(server.url().parse().unwrap(), "".to_string(), false).unwrap();
        list_exercises(&mut io, &mut client, Some("course_name")).unwrap();

        let output = String::from_utf8(output.into_inner()).unwrap();
        let output = output.lines().collect::<Vec<_>>();
        assert!(output[0].eq(""), "first line should be empty");
        let course_string = "Course name: course_name";
        assert!(
            output[1].eq(course_string),
            "Expected '{}', got '{}'",
            course_string,
            output[1]
        );

        let deadline_string = "Deadline: none";
        let soft_deadline_string = "Soft deadline: none";
        assert!(
            output[2].eq(deadline_string),
            "Expected '{}', got '{}'",
            deadline_string,
            output[2]
        );
        assert!(
            output[3].eq(soft_deadline_string),
            "Expected '{}', got '{}'",
            soft_deadline_string,
            output[3]
        );

        let exercise_string_1 = "  Completed: part01-01_example_exercise";
        assert!(
            output[4].eq(exercise_string_1),
            "Expected '{}', got '{}'",
            exercise_string_1,
            output[4]
        );

        let exercise_string_2 = "  Completed: part02-03_example_valid";
        assert!(
            output[5].eq(exercise_string_2),
            "Expected '{}', got '{}'",
            exercise_string_2,
            output[5]
        );

        let expected_size = 6;
        assert!(
            output.len().eq(&expected_size),
            "Expected size '{}', got {}",
            expected_size,
            output.len()
        );
    }
}
