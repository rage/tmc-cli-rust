use super::command_util::{Client, load_course_config};
use crate::io_module::Io;
use isolang::Language;
use reqwest::Url;
use std::path::{Path, PathBuf};
use std::env;

pub fn paste(
    io: &mut dyn Io,
    client: &mut dyn Client,
    exercise: Option<&str>,
    paste_message: Option<&str>,
) {
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };

    // This part pasted from submit_command
    // Assuming we are in course directory (not exercise)! TODO: Make work from other directories
    let mut pathbuf = env::current_dir().unwrap();
    pathbuf.push(".tmc.json"); // TODO: make .tmc.json into a constant


    let id = load_course_config(pathbuf.as_path()).unwrap().course.id;
    let course_details = client.get_course_details(id).unwrap();
    let submission_url = &course_details.exercises[0].return_url;
    let submission_url = Url::parse(&submission_url).unwrap();
    let path_str = exercise.unwrap();
    let paste_msg = match paste_message {
        Some(paste_mes) => Some(paste_mes.to_string()),
        None => Some("No paste message".to_string()),
    };

    // Send submission, handle errors and print link to paste
    let new_submission = client.paste(
        submission_url,
        Path::new(path_str),
        paste_msg,
        Some(Language::Eng),
    );

    if let Err(error) = new_submission.clone() {
        io.println(&error);
    }

    io.println(&format!(
        "Paste submitted to this address: {}",
        new_submission.unwrap().paste_url
    ));
}
