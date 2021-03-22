use super::command_util::Client;
use crate::config::course_config;
use crate::io_module::Io;
use isolang::Language;
use reqwest::Url;
use std::env;
use std::path::Path;

pub fn paste(io: &mut dyn Io, client: &mut dyn Client, path: &str) {
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };

    let exercise_name;
    let mut pathbuf;
    let course_config;
    let mut exercise_dir;

    if path.is_empty() {
        // No exercise path given, so assuming we are in exercise directory.
        // TODO: Error message to say if we are not in exercise dir
        exercise_name = env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        pathbuf = env::current_dir().unwrap();
        pathbuf.pop(); // we go to the course directory
        pathbuf.push(course_config::COURSE_CONFIG_FILE_NAME);
        course_config = match course_config::load_course_config(pathbuf.as_path()) {
            Ok(conf) => conf,
            Err(_error) => {
                io.println("Could not load course config file. Check that you are an exercise folder inside a course folder.");
                return;
            }
        };
        exercise_dir = env::current_dir().unwrap();
    } else {
        // Path given, find out course part, exercise name, and full path
        // TODO: Error message when course config / exercise not found
        exercise_name = Path::new(path)
            .to_path_buf()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        println!("exercise_name {}", exercise_name);
        let mut part_path = Path::new(path).to_path_buf();
        part_path.pop();
        let mut course_config_path = env::current_dir().unwrap();
        course_config_path.push(part_path);
        course_config_path.push(course_config::COURSE_CONFIG_FILE_NAME);
        course_config = match course_config::load_course_config(course_config_path.as_path()) {
            Ok(conf) => conf,
            Err(_error) => {
                io.println("Could not load course config file. Check that exercise path leads to an exercise folder inside a course folder.");
                return;
            }
        };

        exercise_dir = env::current_dir().unwrap();
        exercise_dir.push(Path::new(path).to_path_buf());
    }

    let submission_url = match &course_config::get_exercise_by_name(&course_config, &exercise_name)
    {
        Some(result) => &result.return_url,
        None => {
            io.println(
                "Exercise not found. Check that exercise path leads to a valid exercise folder.",
            );
            return;
        }
    };
    let submission_url = Url::parse(&submission_url).unwrap();

    io.println("Write a paste message, enter sends it:");
    let paste_msg = io.read_line();
    io.println("");

    // Send submission, handle errors and print link to paste
    let new_submission = client.paste(
        submission_url,
        exercise_dir.as_path(),
        Some(paste_msg),
        Some(Language::Eng),
    );

    io.println(&format!(
        "Paste submitted to this address: {} \n",
        new_submission.unwrap().paste_url
    ));
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

    #[test]
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

        //let directory = env::current_dir().unwrap();
        //println!("The current directory is {}", directory.display());

        std::fs::create_dir("tmc_cli_test_course_dir/").unwrap();
        std::fs::create_dir("tmc_cli_test_course_dir/exercise_dir/").unwrap();

        let current_directory = env::current_dir().unwrap();
        //let pathbuf = env::current_dir().unwrap();

        env::set_current_dir("tmc_cli_test_course_dir/exercise_dir/").unwrap();
        paste(&mut io, &mut mock_client, path);

        env::set_current_dir(current_directory.to_str().unwrap().to_string()).unwrap();
        std::fs::remove_dir_all("tmc_cli_test_course_dir/").unwrap();

        assert_eq!(1, io.buffer_length());
        if io.buffer_length() == 1 {
            assert!(io
                .buffer_get(0)
                .to_string()
                .eq(&"Could not load course config file. Check that you are an exercise folder inside a course folder.".to_string()));
        }
    }
}
