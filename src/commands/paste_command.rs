use super::command_util;
use super::command_util::{ask_exercise_interactive, find_submit_or_paste_config, Client};
use crate::io_module::{Io, PrintColor};
use crate::progress_reporting;
use crate::progress_reporting::ProgressBarManager;
use isolang::Language;
use reqwest::Url;
use tmc_langs::ClientUpdateData;

/// Sends the course exercise submission with paste message to the server.
/// Path to the exercise can be given as a parameter or
/// the user can run the command in the exercise folder.
///
/// # Errors
/// Returns an error if no exercise found on given path or current folder.
/// Returns an error if user is not logged in.
pub fn paste(io: &mut dyn Io, client: &mut dyn Client, path: &str) {
    if let Err(error) = client.load_login() {
        io.println(&error, PrintColor::Normal);
        return;
    };

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

    let exercise_id_result =
        command_util::get_exercise_id_from_config(&course_config.unwrap(), &exercise_name);
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

    io.println("Write a paste message, enter sends it:", PrintColor::Normal);
    let paste_msg = io.read_line();
    io.println("", PrintColor::Normal);

    // start manager for 1 events TmcClient::paste
    let mut manager = ProgressBarManager::new(
        progress_reporting::get_default_style(),
        1,
        client.is_test_mode(),
    );
    manager.start::<ClientUpdateData>();

    // Send submission, handle errors and print link to paste
    let new_submission = client.paste(
        return_url,
        exercise_dir.as_path(),
        Some(paste_msg),
        Some(Language::Eng),
    );

    match new_submission {
        Ok(_submission) => {
            manager.join();
        }
        Err(err) => {
            manager.force_join();
            io.println(&format!("Error: {} \n", err), PrintColor::Normal);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::command_util::*;
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
    fn paste_command_when_not_logged_in_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec![];
        let mut input = input.iter();
        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };

        let mut mock_client = MockClient::new();
        mock_client
            .expect_load_login()
            .returning(|| Err("Not logged in message.".to_string()));

        let path = "";

        paste(&mut io, &mut mock_client, path);

        assert_eq!(1, io.buffer_length());
        if io.buffer_length() == 1 {
            assert!(io
                .buffer_get(0)
                .to_string()
                .eq(&"Not logged in message.".to_string()));
        }
    }

    //#[test]
    fn paste_command_when_path_is_empty_and_config_file_not_exists_test() {
        let mut v: Vec<String> = Vec::new();
        let input = vec![];
        let mut input = input.iter();
        let mut io = IoTest {
            list: &mut v,
            input: &mut input,
        };

        let mut mock_client = MockClient::new();
        mock_client.expect_load_login().returning(|| Ok(()));

        let path = "";

        std::fs::create_dir("tmc_cli_test_course_dir/").unwrap();
        std::fs::create_dir("tmc_cli_test_course_dir/exercise_dir/").unwrap();

        let current_directory = std::env::current_dir().unwrap();

        std::env::set_current_dir("tmc_cli_test_course_dir/exercise_dir/").unwrap();
        paste(&mut io, &mut mock_client, path);

        std::env::set_current_dir(current_directory.to_str().unwrap().to_string()).unwrap();
        std::fs::remove_dir_all("tmc_cli_test_course_dir/").unwrap();

        assert_eq!(1, io.buffer_length());
        if io.buffer_length() == 1 {
            assert!(io
                .buffer_get(0)
                .to_string()
                .eq(&"Could not load course config file. Check that exercise path leads to an exercise folder inside a course folder.".to_string()));
        }
    }
}
