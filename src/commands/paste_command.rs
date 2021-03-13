use super::command_util::{Client, load_course_config};
use crate::io_module::Io;
use isolang::Language;
use reqwest::Url;
use std::path::{Path, PathBuf};
use std::env;

pub fn paste(
    io: &mut dyn Io,
    client: &mut dyn Client
) {
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };

    // Pasted from submit. Assuming we are in exercise directory
    let mut pathbuf = env::current_dir().unwrap();
    pathbuf.pop(); // we go to the course directory
    pathbuf.push(".tmc.json"); // TODO: make .tmc.json into a constant


    

    let id = load_course_config(pathbuf.as_path()).unwrap().course.id;
    let course_details = client.get_course_details(id).unwrap();
    let submission_url = &course_details.exercises[0].return_url;
    let submission_url = Url::parse(&submission_url).unwrap();
    io.println("Write a paste message, enter sends it:");
    let paste_msg = io.read_line();
    io.println("");


    // Send submission, handle errors and print link to paste
    let new_submission = client.paste(
        submission_url,
        Path::new("./"),
        Some(paste_msg),
        Some(Language::Eng),
    );

    if let Err(error) = new_submission.clone() {
        io.println(&error);
    }

    io.println(&format!(
        "Paste submitted to this address: {} \n",
        new_submission.unwrap().paste_url
    ));
}
