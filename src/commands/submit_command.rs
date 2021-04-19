use super::command_util;
use super::command_util::*;
use crate::io_module::{Io, PrintColor};
use crate::progress_reporting;
use crate::progress_reporting::ProgressBarManager;
use anyhow::{Context, Result};
use tmc_langs::ClientError;
use tmc_langs::ClientUpdateData;
use tmc_langs::Language;
use tmc_langs::NewSubmission;
use tmc_langs::SubmissionFinished;
use url::Url;
/// Sends the course exercise submission to the server.
/// Path to the exercise can be given as a parameter or
/// the user can run the command in the exercise folder.
///
/// # Errors
/// Returns an error if no exercise was found on given path or current folder.
/// Returns an error if user is not logged in.
pub fn submit(io: &mut dyn Io, client: &mut dyn Client, path: &str) {
    if let Err(error) = client.load_login() {
        io.println(&error, PrintColor::Normal);
        return;
    }

    submit_logic(io, client, path);
}

fn submit_logic(io: &mut dyn Io, client: &mut dyn Client, path: &str) {
    let locale = into_locale("fin").unwrap();

    let mut exercise_name = "".to_string();
    let mut course_config = None;
    let mut exercise_dir = std::path::PathBuf::new();

    find_submit_or_paste_config(
        &mut exercise_name,
        &mut course_config,
        &mut exercise_dir,
        path,
    )
    .unwrap();

    if course_config.is_none() {
        if client.is_test_mode() {
            io.println("Could not load course config file. Check that exercise path leads to an exercise folder inside a course folder.", PrintColor::Normal);
            return;
        }
        // Did not find course config, use interactive selection if possible
        match ask_exercise_interactive(&mut exercise_name, &mut exercise_dir, &mut course_config) {
            Ok(()) => (),
            Err(msg) => {
                io.println(&msg, PrintColor::Normal);
                return;
            }
        }
    }

    let course_config = course_config.unwrap();
    let exercise_id_result =
        command_util::get_exercise_id_from_config(&course_config, &exercise_name);
    let return_url: Url;
    match exercise_id_result {
        Ok(exercise_id) => {
            return_url = Url::parse(&command_util::generate_return_url(exercise_id)).unwrap();
        }
        Err(err) => {
            io.println(&err, PrintColor::Normal);
            return;
        }
    }

    io.println("\n", PrintColor::Normal);
    // start manager for 2 events TmcClient::submit, TmcClient::wait_for_submission
    let mut manager = ProgressBarManager::new(
        progress_reporting::get_default_style(),
        2,
        client.is_test_mode(),
    );
    manager.start::<ClientUpdateData>();

    // Send submission
    let new_submission_result = client.submit(return_url, exercise_dir.as_path(), Some(locale));
    if let Err(err) = new_submission_result {
        manager.force_join();

        match err {
            ClientError::HttpError {
                url,
                status: _,
                error,
                obsolete_client: _,
            } => {
                io.println(
                    &format!(
                        "\nGot error '{}' \n    while submitting exercise to address {}",
                        error, url
                    ),
                    PrintColor::Normal,
                );
            }
            _ => {
                io.println("Error during submission", PrintColor::Normal);
            }
        }
        return;
    }

    let new_submission: NewSubmission = new_submission_result.unwrap();
    manager.println(format!(
        "You can view your submission at: {}",
        new_submission.show_submission_url
    ));

    let wait_status: Result<SubmissionFinished, ClientError> =
        client.wait_for_submission(&new_submission.submission_url);
    match wait_status {
        Ok(submission_finished) => {
            manager.join();

            print_wait_for_submission_results(io, submission_finished);
        }
        Err(err) => {
            manager.force_join();
            io.println(&format!("Failed while waiting for server to process submission.\n You can still check your submission manually here: {}.", &new_submission.show_submission_url), PrintColor::Normal);
            io.println(&format!("Error message: {:#?}", err), PrintColor::Normal);
        }
    }
}

fn print_wait_for_submission_results(io: &mut dyn Io, submission_finished: SubmissionFinished) {
    let mut all_passed = false;
    if let Some(all_tests_passed) = submission_finished.all_tests_passed {
        all_passed = all_tests_passed;
        if all_tests_passed {
            io.println("All tests passed on server!", PrintColor::Normal);
        }
    }
    if !submission_finished.points.is_empty() {
        io.print("Points permanently awarded: [", PrintColor::Normal);
        for i in 0..submission_finished.points.len() {
            io.print(
                &submission_finished.points[i].to_string(),
                PrintColor::Normal,
            );
            if i < submission_finished.points.len() - 1 {
                io.print(", ", PrintColor::Normal);
            }
        }
        io.println("]", PrintColor::Normal);
    } else {
        io.println("No new points awarded.", PrintColor::Normal);
    }

    if all_passed {
        if let Some(solution_url) = submission_finished.solution_url {
            io.println(
                &format!("Model solution: {}", solution_url),
                PrintColor::Normal,
            );
        }
    } else {
        if let Some(error) = submission_finished.error {
            io.println(&format!("Error: {}", error), PrintColor::Normal);
        }

        if let Some(test_cases) = submission_finished.test_cases {
            let mut completed = 0;
            let mut total = 0;
            for case in test_cases {
                if case.successful {
                    io.println(&format!("Failed: {}", case.name), PrintColor::Normal);
                    if let Some(message) = case.message {
                        io.println(&format!("    Message: {}", message), PrintColor::Normal);
                    }
                    if let Some(detailed_message) = case.detailed_message {
                        io.println(
                            &format!("    Detailed message: {}", detailed_message),
                            PrintColor::Normal,
                        );
                    }
                    if let Some(exceptions) = case.exception {
                        for exception in exceptions {
                            io.println(
                                &format!("        Exception: {}", exception),
                                PrintColor::Normal,
                            );
                        }
                    }
                    completed += 1;
                }
                total += 1;
            }
            io.println(
                &format!("\nTest results: {}/{} tests passed", completed, total),
                PrintColor::Normal,
            );

            io.println(
                &command_util::get_progress_string(completed, total, 64),
                PrintColor::Normal,
            );
        }
    }
}

fn into_locale(arg: &str) -> Result<Language> {
    Language::from_locale(arg)
        .or_else(|| Language::from_639_1(arg))
        .or_else(|| Language::from_639_3(arg))
        .with_context(|| format!("Invalid locale: {}", arg))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::slice::Iter;

    pub struct IoTest<'a> {
        list: &'a mut Vec<String>,
        input: &'a mut Iter<'a, &'a str>,
    }

    impl IoTest<'_> {
        pub fn buffer_length(&mut self) -> usize {
            self.list.len()
        }

        pub fn buffer_get(&mut self, index: usize) -> String {
            self.list[index].to_string()
        }
    }

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

    #[test]
    fn submit_not_logged_in_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec![];
        let mut input = input.iter();
        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };

        let mut mock = MockClient::new();
        mock.expect_load_login()
            .returning(|| Err("Not logged in.".to_string()));

        let path = "";

        submit(&mut io, &mut mock, path);

        assert_eq!(1, io.buffer_length());
        if io.buffer_length() == 1 {
            assert!(io
                .buffer_get(0)
                .to_string()
                .eq(&"Not logged in.".to_string()));
        }
    }

    //#[test]
    fn submit_with_proper_login_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec![];
        let mut input = input.iter();
        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };

        let mut mock = MockClient::new();
        mock.expect_load_login().returning(|| Ok(()));

        let path = "";

        submit(&mut io, &mut mock, path);

        assert_eq!(1, io.buffer_length());
        assert!(io
            .buffer_get(0)
            .to_string()
            .eq(&"Could not load course config file. Check that exercise path leads to an exercise folder inside a course folder.".to_string()));
    }
}
