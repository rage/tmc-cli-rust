use super::command_util::*;
use crate::io_module::Io;

use tmc_client::CourseExercise;

/// Lists exercises for a given course
pub fn list_exercises(io: &mut dyn Io, client: &mut dyn Client, course_name: String) {
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };

    // Get course by id
    let course_result = get_course_id_by_name(client, course_name.clone());
    if course_result.is_none() {
        io.println(&format!(
            "Could not find a course with name '{}'",
            course_name
        ));
        return;
    }
    let course_id = course_result.unwrap();

    match client.get_course_exercises(course_id) {
        Ok(exercises) => print_exercises(io, course_name, exercises),
        Err(error) => io.println(&error),
    }
}

/// Prints information about given exercises
fn print_exercises(io: &mut dyn Io, course_name: String, exercises: Vec<CourseExercise>) {
    io.println("");
    io.println(&format!("Course name: {}", course_name));

    let none = "none".to_string();
    let mut prev_deadline = "".to_string();
    let mut prev_soft_deadline = "".to_string();
    for exercise in exercises {
        // Skip locked and disabled exercises
        if exercise.disabled || !exercise.unlocked {
            continue;
        }

        // Print deadline if it exists
        if let Some(dl) = exercise.deadline {
            if prev_deadline != dl {
                io.println(&format!("Deadline: {}", &dl));
                prev_deadline = dl.clone();
            }
        } else if prev_deadline != none {
            io.println(&format!("Deadline: {}", &none));
            prev_deadline = none.clone();
        }

        // TODO: Do we need soft deadline?
        if let Some(dl) = exercise.soft_deadline {
            if prev_soft_deadline != dl {
                io.println(&format!("Soft deadline: {}", &dl));
                prev_soft_deadline = dl.clone();
            }
        } else if prev_soft_deadline != none {
            io.println(&format!("Soft deadline: {}", &none));
            prev_soft_deadline = none.clone();
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

        io.println(&format!("  {}: {}", completion_status, &exercise.name));
    }
}

#[cfg(test)]
mod tests {
    use isolang::Language;
    use reqwest::Url;
    use std::path::{Path, PathBuf};
    use tmc_client::Course;
    use tmc_client::Organization;
    use tmc_client::{
        ClientError, CourseExercise, NewSubmission, SubmissionFinished, SubmissionStatus,
    };

    use super::*;
    use std::slice::Iter;
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
    pub struct ClientTest {}

    #[cfg(test)]
    impl ClientTest {}

    #[cfg(test)]
    impl Client for ClientTest {
        fn paste(
            &self,
            submission_url: Url,
            submission_path: &Path,
            paste_message: Option<String>,
            locale: Option<Language>,
        ) -> Result<NewSubmission, String> {
            Err("not implemented".to_string())
        }
        fn is_test_mode(&mut self) -> bool {
            false
        }
        fn load_login(&mut self) -> Result<(), String> {
            Ok(())
        }
        fn try_login(&mut self, _username: String, _password: String) -> Result<String, String> {
            Ok("ok".to_string())
        }
        fn list_courses(&mut self) -> Result<Vec<Course>, String> {
            Ok(vec![
                Course {
                    id: 0,
                    name: "name".to_string(),
                    title: "".to_string(),
                    description: None,
                    details_url: "".to_string(),
                    unlock_url: "".to_string(),
                    reviews_url: "".to_string(),
                    comet_url: "".to_string(),
                    spyware_urls: vec![],
                },
                Course {
                    id: 88,
                    name: "course_name".to_string(),
                    title: "".to_string(),
                    description: None,
                    details_url: "".to_string(),
                    unlock_url: "".to_string(),
                    reviews_url: "".to_string(),
                    comet_url: "".to_string(),
                    spyware_urls: vec![],
                },
            ])
        }
        fn get_organizations(&mut self) -> Result<Vec<Organization>, String> {
            Ok(vec![])
        }
        fn logout(&mut self) {}
        fn submit(
            &self,
            submission_url: Url,
            submission_path: &Path,
            locale: Option<Language>,
        ) -> Result<NewSubmission, ClientError> {
            Ok(NewSubmission {
                show_submission_url: "".to_string(),
                paste_url: "".to_string(),
                submission_url: "".to_string(),
            })
        }
        fn wait_for_submission(
            &self,
            submission_url: &str,
        ) -> Result<SubmissionFinished, ClientError> {
            Ok(SubmissionFinished {
                api_version: 0,
                all_tests_passed: Some(true),
                user_id: 0,
                login: "".to_string(),
                course: "".to_string(),
                exercise_name: "".to_string(),
                status: SubmissionStatus::Ok,
                points: vec!["".to_string()],
                valgrind: Some("".to_string()),
                submission_url: "".to_string(),
                solution_url: Some("".to_string()),
                submitted_at: "".to_string(),
                processing_time: Some(0),
                reviewed: true,
                requests_review: true,
                paste_url: Some("".to_string()),
                message_for_paste: Some("".to_string()),
                missing_review_points: vec!["".to_string()],
                test_cases: None,
                feedback_questions: None,
                feedback_answer_url: Some("".to_string()),
                error: Some("".to_string()),
                validations: None,
            })
        }
        fn get_course_exercises(
            &mut self,
            _course_id: usize,
        ) -> Result<Vec<CourseExercise>, String> {
            /*TODO: ExercisePoint is in private module*/
            //let points = vec![];
            //let awarded_points = vec![/*"1.1".to_string()*/];

            let exercise1 = CourseExercise {
                id: 0,
                available_points: vec![],
                awarded_points: vec![],
                name: "part01-01_example_exercise".to_string(),
                publish_time: None,
                solution_visible_after: None,
                deadline: None,
                soft_deadline: None,
                disabled: false,
                unlocked: true,
            };
            let exercise2 = CourseExercise {
                id: 24,
                available_points: vec![],
                awarded_points: vec![],
                name: "part01-02_example_disabled".to_string(),
                publish_time: None,
                solution_visible_after: None,
                deadline: None,
                soft_deadline: None,
                disabled: true,
                unlocked: true,
            };
            let exercise3 = CourseExercise {
                id: 578,
                available_points: vec![],
                awarded_points: vec![],
                name: "part02-01_example_not_unlocked".to_string(),
                publish_time: None,
                solution_visible_after: None,
                deadline: None,
                soft_deadline: None,
                disabled: false,
                unlocked: false,
            };

            let exercise4 = CourseExercise {
                id: 578,
                available_points: vec![],
                awarded_points: vec![],
                name: "part02-02_example_disabled2".to_string(),
                publish_time: None,
                solution_visible_after: None,
                deadline: None,
                soft_deadline: None,
                disabled: true,
                unlocked: false,
            };

            let exercise5 = CourseExercise {
                id: 578,
                available_points: vec![],
                awarded_points: vec![],
                name: "part02-03_example_valid".to_string(),
                publish_time: None,
                solution_visible_after: None,
                deadline: None,
                soft_deadline: None,
                disabled: false,
                unlocked: true,
            };

            let exercises = vec![exercise1, exercise2, exercise3, exercise4, exercise5];
            Ok(exercises)
        }

        fn get_exercise_details(
            &mut self,
            exercise_ids: Vec<usize>,
        ) -> Result<Vec<tmc_client::ExercisesDetails>, String> {
            todo!()
        }

        fn download_or_update_exercises(
            &mut self,
            _download_params: Vec<(usize, PathBuf)>,
        ) -> Result<(), ClientError> {
            Ok(())
        }

        fn get_course_details(
            &self,
            _: usize,
        ) -> std::result::Result<tmc_client::CourseDetails, tmc_client::ClientError> {
            todo!()
        }
        fn get_organization(
            &self,
            _: &str,
        ) -> std::result::Result<Organization, tmc_client::ClientError> {
            todo!()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn list_exercises_test() {
            let mut v: Vec<String> = Vec::new();
            let input = vec![];
            let mut input = input.iter();

            let mut io = IoTest {
                list: &mut v,
                input: &mut input,
            };

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

            let exercises = vec![CourseExercise {
                id: 0,
                available_points: points,
                awarded_points: awarded_points,
                name: "part01-01_example_exercise".to_string(),
                publish_time: None,
                solution_visible_after: None,
                deadline: None,
                soft_deadline: None,
                disabled: false,
                unlocked: true,
            }];

            print_exercises(&mut io, "course_name".to_string(), exercises);
            assert!(io.list[0].eq(""), "first line should be empty");
            let course_string = "Course name: course_name";
            assert!(
                io.list[1].eq(course_string),
                "Expected '{}', got '{}'",
                course_string,
                io.list[1]
            );
            let deadline_string = "Deadline: none";
            let soft_deadline_string = "Soft deadline: none";
            assert!(
                io.list[2].eq(deadline_string),
                "Expected '{}', got '{}'",
                deadline_string,
                io.list[2]
            );
            assert!(
                io.list[3].eq(soft_deadline_string),
                "Expected '{}', got '{}'",
                soft_deadline_string,
                io.list[3]
            );

            let exercise_string = "  Completed: part01-01_example_exercise";
            assert!(
                io.list[4].eq(exercise_string),
                "Expected '{}', got '{}'",
                exercise_string,
                io.list[4]
            );
        }

        #[test]
        fn list_exercises_with_client_test() {
            let mut v: Vec<String> = Vec::new();
            let input = vec![];
            let mut input = input.iter();

            let mut io = IoTest {
                list: &mut v,
                input: &mut input,
            };
            let mut client = ClientTest {};
            list_exercises(&mut io, &mut client, "course_name".to_string());

            assert!(io.list[0].eq(""), "first line should be empty");
            let course_string = "Course name: course_name";
            assert!(
                io.list[1].eq(course_string),
                "Expected '{}', got '{}'",
                course_string,
                io.list[1]
            );

            let deadline_string = "Deadline: none";
            let soft_deadline_string = "Soft deadline: none";
            assert!(
                io.list[2].eq(deadline_string),
                "Expected '{}', got '{}'",
                deadline_string,
                io.list[2]
            );
            assert!(
                io.list[3].eq(soft_deadline_string),
                "Expected '{}', got '{}'",
                soft_deadline_string,
                io.list[3]
            );

            let exercise_string_1 = "  Completed: part01-01_example_exercise";
            assert!(
                io.list[4].eq(exercise_string_1),
                "Expected '{}', got '{}'",
                exercise_string_1,
                io.list[4]
            );

            let exercise_string_2 = "  Completed: part02-03_example_valid";
            assert!(
                io.list[5].eq(exercise_string_2),
                "Expected '{}', got '{}'",
                exercise_string_2,
                io.list[5]
            );

            let expected_size = 6;
            assert!(
                io.list.len().eq(&expected_size),
                "Expected size '{}', got {}",
                expected_size,
                io.list.len()
            );
        }
    }
}
