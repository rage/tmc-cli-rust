use super::command_util::Client;
use crate::io_module::Io;
use isolang::Language;
use reqwest::Url;
use std::path::Path;

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
    let course_details = client.get_course_details(600).unwrap();
    let submission_url = &course_details.exercises[0].return_url;
    let submission_url = Url::parse(&submission_url).unwrap();
    let path_str = exercise.unwrap_or("");

    let paste_msg = match paste_message {
        Some(paste_mes) => Some(paste_mes.to_string()),
        None => Some("No paste message".to_string()),
    };

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
