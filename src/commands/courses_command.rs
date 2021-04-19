use super::command_util::*;
use crate::io_module::{Io,PrintColor};
use tmc_client::Course;

/// Lists available courses from clients organization
pub fn list_courses(io: &mut dyn Io, client: &mut dyn Client) {
    if let Err(error) = client.load_login() {
        io.println(&error, PrintColor::Normal);
        return;
    }

    let courses_result = client.list_courses();

    match courses_result {
        Ok(course_list) => print_courses(io, course_list),
        Err(error) => io.println(&error, PrintColor::Normal),
    }
}

/// Prints course names
fn print_courses(io: &mut dyn Io, course_list: Vec<Course>) {
    io.println("", PrintColor::Normal);
    for course in course_list {
        io.println(&course.name, PrintColor::Normal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use isolang::Language;
    use reqwest::Url;
    use std::path::Path;
    use std::slice::Iter;
    use tmc_client::{ClientError, CourseExercise, NewSubmission, SubmissionStatus};
    use tmc_client::{Organization, SubmissionFinished};
    use tmc_langs::DownloadResult;
    use tmc_langs::LangsError;
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

        fn print(&mut self, output: &str, _font_color: PrintColor) {
            print!("{}", output);
            self.list.push(output.to_string());
        }

        fn println(&mut self, output: &str, _font_color: PrintColor) {
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
            _submission_url: Url,
            _submission_path: &Path,
            _paste_message: Option<String>,
            _locale: Option<Language>,
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
        fn get_organizations(&mut self) -> Result<Vec<Organization>, String> {
            Ok(vec![])
        }
        fn logout(&mut self) {}
        fn submit(
            &self,
            _submission_url: Url,
            _submission_path: &Path,
            _locale: Option<Language>,
        ) -> Result<NewSubmission, ClientError> {
            Ok(NewSubmission {
                show_submission_url: "".to_string(),
                paste_url: "".to_string(),
                submission_url: "".to_string(),
            })
        }
        fn wait_for_submission(
            &self,
            _submission_url: &str,
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
            Ok(vec![])
        }

        fn get_exercise_details(
            &mut self,
            _exercise_ids: Vec<usize>,
        ) -> Result<Vec<tmc_client::ExercisesDetails>, String> {
            todo!()
        }

        fn download_or_update_exercises(
            &mut self,
            _download_params: &[usize],
            _path: &Path,
        ) -> Result<DownloadResult, LangsError> {
            Ok(DownloadResult::Success {
                downloaded: vec![],
                skipped: vec![],
            })
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
        fn list_courses_test() {
            let mut v: Vec<String> = Vec::new();
            let input = vec![];
            let mut input = input.iter();

            let mut io = IoTest {
                list: &mut v,
                input: &mut input,
            };

            let courses = vec![
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
                    id: 10,
                    name: "course of sorts".to_string(),
                    title: "".to_string(),
                    description: None,
                    details_url: "".to_string(),
                    unlock_url: "".to_string(),
                    reviews_url: "".to_string(),
                    comet_url: "".to_string(),
                    spyware_urls: vec![],
                },
            ];
            print_courses(&mut io, courses);

            assert!(io.list[0].eq(""));
            assert!(io.list[1].eq("name"), "Expected 'name', got {}", io.list[1]);
            assert!(
                io.list[2].eq("course of sorts"),
                "Expected 'course of sorts', got {}",
                io.list[2]
            );
        }

        #[test]
        fn list_courses_with_client_test() {
            let mut v: Vec<String> = Vec::new();
            let input = vec![];
            let mut input = input.iter();

            let mut io = IoTest {
                list: &mut v,
                input: &mut input,
            };

            let mut client = ClientTest {};
            list_courses(&mut io, &mut client);

            assert!(io.list[0].eq(""), "first line should be empty");
            assert!(io.list[1].eq("name"), "Expected 'name', got {}", io.list[1]);
            assert!(
                io.list[2].eq("mooc-tutustumiskurssi"),
                "Expected 'mooc-tutustumiskurssi', got '{}'",
                io.list[2]
            );
        }
    }
}
