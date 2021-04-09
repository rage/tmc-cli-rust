use super::command_util;
use super::command_util::*;
use crate::io_module::Io;
use crate::progress_reporting::ProgressBarManager;
use anyhow::{Context, Result};
use indicatif::ProgressStyle;
use tmc_langs::ClientError;
use tmc_langs::ClientUpdateData;
use tmc_langs::Language;
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
        io.println(&error);
        return;
    }

    //file_util::lock!(submission_path);
    submit_logic(io, client, path);
}

fn submit_logic(io: &mut dyn Io, client: &mut dyn Client, path: &str) {
    let locale = into_locale("fin").unwrap();

    let mut exercise_name = "".to_string();
    let mut course_config = None;
    let mut exercise_dir = std::path::PathBuf::new();

    match find_submit_or_paste_config(
        &mut exercise_name,
        &mut course_config,
        &mut exercise_dir,
        path,
    ) {
        Ok(_) => (),
        Err(msg) => {
            io.println(&msg);
            return;
        }
    }

    if course_config.is_none() {
        io.println("could not find course config");
        return;
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
            io.println(&err);
            return;
        }
    }

    // Make style for progress bar:
    let progress_style = ProgressStyle::default_bar()
    .template(
        "{wide_msg} \n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})",
    )
    .progress_chars("#>-");

    // start manager for 2 events TmcClient::Submit, TmcClient::WaitForSubmission
    let mut manager = ProgressBarManager::new(progress_style, 2, client.is_test_mode());
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
                io.println(&format!(
                    "\nGot error '{}' \n    while submitting exercise to address {}",
                    error, url
                ));
            }
            _ => {
                io.println("Error during submission");
            }
        }
        return;
    }

    let new_submission = new_submission_result.unwrap();

    let wait_status = client.wait_for_submission(&new_submission.submission_url);
    match wait_status {
        Ok(_submission_finished) => {
            manager.join();

            io.println(&format!(
                "Submission finished.\nYou can find your submission here: {}",
                &new_submission.show_submission_url
            ));
        }
        Err(err) => {
            manager.force_join();
            io.println(&format!("Failed while waiting for server to process submission.\n You can still check your submission manually here: {}.", &new_submission.show_submission_url));
            io.println(&format!("Error message: {:#?}", err));
        }
    }
}

fn into_locale(arg: &str) -> Result<Language> {
    Language::from_locale(arg)
        .or_else(|| Language::from_639_1(arg))
        .or_else(|| Language::from_639_3(arg))
        .with_context(|| format!("Invalid locale: {}", arg))
}

/*fn into_url(arg: &str) -> Result<Url> {
    Url::parse(arg).with_context(|| format!("Failed to parse url {}", arg))
}*/

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
    #[test]
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
