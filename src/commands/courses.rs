use super::util::Client;
use crate::io::{Io, PrintColor};

/// Lists available courses from clients organization
pub fn list_courses(io: &mut Io, client: &mut dyn Client) -> anyhow::Result<()> {
    let mut course_list = client.list_courses()?;
    course_list.sort_unstable_by(|l, r| l.name.cmp(&r.name));
    io.println("", PrintColor::Normal)?;
    for course in course_list {
        io.println(&course.name, PrintColor::Normal)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use reqwest::Url;
    use std::{io::Cursor, path::Path, slice::Iter};
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
                    name: "mooc-tutustumiskurssi".to_string(),
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
        fn update_exercises(
            &mut self,
            _path: &Path,
        ) -> Result<DownloadOrUpdateCourseExercisesResult, LangsError> {
            unimplemented!()
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
            Ok(vec![])
        }

        fn get_exercise_details(
            &mut self,
            _exercise_ids: Vec<u32>,
        ) -> Result<Vec<ExercisesDetails>, TestMyCodeClientError> {
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

        fn get_course_details(
            &self,
            _: u32,
        ) -> std::result::Result<CourseDetails, TestMyCodeClientError> {
            unimplemented!()
        }
        fn get_organization(
            &self,
            _: &str,
        ) -> std::result::Result<Organization, TestMyCodeClientError> {
            unimplemented!()
        }
        fn mooc_courses(&self) -> anyhow::Result<Vec<mooc::CourseInstance>> {
            unimplemented!()
        }
        fn mooc_course_exercises(
            &self,
            course_instance_id: Uuid,
        ) -> anyhow::Result<Vec<mooc::TmcExerciseSlide>> {
            unimplemented!()
        }
        fn mooc_download_exercise(&self, url: String) -> anyhow::Result<Bytes> {
            unimplemented!()
        }
        fn mooc_submit_exercise(
            &self,
            exercise_id: Uuid,
            slide_id: Uuid,
            task_id: Uuid,
            archive: &Path,
        ) -> anyhow::Result<ExerciseTaskSubmissionResult> {
            unimplemented!()
        }
        fn mooc_get_submission_grading(
            &self,
            submission_id: Uuid,
        ) -> anyhow::Result<ExerciseTaskSubmissionStatus> {
            unimplemented!()
        }
    }

    #[test]
    fn list_courses_with_client_test() {
        let mut output = NoColor::new(Vec::<u8>::new());
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut io = Io::new(&mut output, &mut input);

        let mut client = ClientTest;
        list_courses(&mut io, &mut client).unwrap();

        let output = String::from_utf8(output.into_inner()).unwrap();
        let output = output.lines().collect::<Vec<_>>();
        assert!(output[0].eq(""), "first line should be empty");
        assert!(
            output[1].eq("mooc-tutustumiskurssi"),
            "Expected 'mooc-tutustumiskurssi', got {}",
            output[1]
        );
        assert!(output[2].eq("name"), "Expected 'name', got '{}'", output[2]);
    }
}
