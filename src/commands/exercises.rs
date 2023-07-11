use super::util::{self, choose_course, Client};
use crate::io::{Io, PrintColor};
use tmc_langs::tmc::response::CourseExercise;

/// Lists exercises for a given course
pub fn list_exercises(
    io: &mut Io,
    client: &mut dyn Client,
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
    use bytes::Bytes;
    use reqwest::Url;
    use std::{io::Cursor, path::Path};
    use termcolor::NoColor;
    use tmc_langs::{
        mooc::{self, ExerciseTaskSubmissionResult, ExerciseTaskSubmissionStatus},
        tmc::{
            response::{
                Course, CourseDetails, CourseExercise, ExercisesDetails, NewSubmission,
                Organization, SubmissionFinished, SubmissionStatus,
            },
            TestMyCodeClientError,
        },
        DownloadOrUpdateCourseExercisesResult, DownloadResult, LangsError, Language,
    };
    use uuid::Uuid;

    pub struct ClientTest;

    impl Client for ClientTest {
        fn paste(
            &self,
            _projects_dir: &Path,
            _course_slug: &str,
            _exercise_slug: &str,
            _paste_message: Option<String>,
            _locale: Option<Language>,
        ) -> Result<NewSubmission, String> {
            Err("not implemented".to_string())
        }
        fn is_test_mode(&mut self) -> bool {
            false
        }
        fn load_login(&mut self) -> anyhow::Result<()> {
            Ok(())
        }
        fn try_login(&mut self, _username: String, _password: String) -> anyhow::Result<String> {
            Ok("ok".to_string())
        }
        fn list_courses(&mut self) -> anyhow::Result<Vec<Course>> {
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
        fn get_organizations(&mut self) -> anyhow::Result<Vec<Organization>> {
            Ok(vec![])
        }
        fn logout(&mut self) -> anyhow::Result<()> {
            Ok(())
        }
        fn submit(
            &self,
            _projects_dir: &Path,
            _course_slug: &str,
            _exercise_slug: &str,
            _locale: Option<Language>,
        ) -> Result<NewSubmission, LangsError> {
            Ok(NewSubmission {
                show_submission_url: "".to_string(),
                paste_url: "".to_string(),
                submission_url: "".to_string(),
            })
        }
        fn wait_for_submission(
            &self,
            _submission_url: Url,
        ) -> Result<SubmissionFinished, TestMyCodeClientError> {
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
        fn get_course_exercises(&mut self, _course_id: u32) -> anyhow::Result<Vec<CourseExercise>> {
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
            _exercise_ids: Vec<u32>,
        ) -> Result<Vec<ExercisesDetails>, TestMyCodeClientError> {
            unimplemented!()
        }
        fn update_exercises(
            &mut self,
            _path: &Path,
        ) -> Result<DownloadOrUpdateCourseExercisesResult, LangsError> {
            unimplemented!()
        }
        fn download_or_update_exercises(
            &mut self,
            _download_params: &[u32],
            _path: &Path,
        ) -> Result<DownloadResult, LangsError> {
            Ok(DownloadResult::Success {
                downloaded: vec![],
                skipped: vec![],
            })
        }

        fn get_course_details(&self, _: u32) -> Result<CourseDetails, TestMyCodeClientError> {
            unimplemented!()
        }
        fn get_organization(&self, _: &str) -> Result<Organization, TestMyCodeClientError> {
            unimplemented!()
        }
        fn mooc_courses(&self) -> anyhow::Result<Vec<mooc::CourseInstance>> {
            unimplemented!()
        }
        fn mooc_course_exercises(
            &self,
            _course_instance_id: Uuid,
        ) -> anyhow::Result<Vec<mooc::TmcExerciseSlide>> {
            unimplemented!()
        }
        fn mooc_download_exercise(&self, _url: String) -> anyhow::Result<Bytes> {
            unimplemented!()
        }
        fn mooc_submit_exercise(
            &self,
            _exercise_id: Uuid,
            _slide_id: Uuid,
            _task_id: Uuid,
            _archive: &Path,
        ) -> anyhow::Result<ExerciseTaskSubmissionResult> {
            unimplemented!()
        }
        fn mooc_get_submission_grading(
            &self,
            _submission_id: Uuid,
        ) -> anyhow::Result<ExerciseTaskSubmissionStatus> {
            unimplemented!()
        }
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
        let mut output = NoColor::new(Vec::<u8>::new());
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut io = Io::new(&mut output, &mut input);

        let mut client = ClientTest;
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
