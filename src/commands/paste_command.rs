use super::command_util;
use super::command_util::Client;
use crate::config::course_config;
use crate::io_module::Io;
use isolang::Language;
use reqwest::Url;
use std::env;

pub fn paste(io: &mut dyn Io, client: &mut dyn Client) {
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };

    // Pasted from submit. Assuming we are in exercise directory
    let mut pathbuf = env::current_dir().unwrap();
    let current_dir = env::current_dir().unwrap();
    pathbuf.pop(); // we go to the course directory
    pathbuf.push(course_config::COURSE_CONFIG_FILE_NAME);

    let course_id = course_config::load_course_config(pathbuf.as_path())
        .unwrap()
        .course
        .id;
    let course_details = client.get_course_details(course_id).unwrap();

    // find out exercise id
    let exercise_id = command_util::get_exercise_id_by_name(
        client,
        course_id,
        env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    )
    .unwrap();

    let submission_url = &course_details.exercises[exercise_id].return_url;
    let submission_url = Url::parse(&submission_url).unwrap();
    io.println("Write a paste message, enter sends it:");
    let paste_msg = io.read_line();
    io.println("");

    // Send submission, handle errors and print link to paste
    let new_submission = client.paste(
        submission_url,
        current_dir.as_path(),
        Some(paste_msg),
        Some(Language::Eng),
    );

    io.println(&format!(
        "Paste submitted to this address: {} \n",
        new_submission.unwrap().paste_url
    ));
}
