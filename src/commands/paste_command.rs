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
        course_config = course_config::load_course_config(pathbuf.as_path()).unwrap();
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
        let mut part_path = Path::new(path).to_path_buf();
        part_path.pop();
        let mut course_config_path = env::current_dir().unwrap();
        course_config_path.push(part_path);
        course_config_path.push(course_config::COURSE_CONFIG_FILE_NAME);
        course_config = course_config::load_course_config(course_config_path.as_path()).unwrap();
        exercise_dir = env::current_dir().unwrap();
        exercise_dir.push(Path::new(path).to_path_buf());
    }

    let submission_url = &course_config::get_exercise_by_name(&course_config, &exercise_name)
        .unwrap()
        .return_url;
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
