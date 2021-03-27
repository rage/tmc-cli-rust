use super::command_util::*;
use crate::config::course_config;
use crate::io_module::Io;
use anyhow::{Context, Result};
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

    let course_config = course_config.unwrap();

    let submission_url;
    match course_config::get_exercise_by_name(&course_config, &exercise_name) {
        Some(exercise) => submission_url = into_url(&exercise.return_url).unwrap(),
        None => {
            io.println("Current directory does not contain any exercise");
            return;
        }
    }

    //file_util::lock!(submission_path);
    let new_submission = client.submit(submission_url, exercise_dir.as_path(), Some(locale));
    let submission_url = &new_submission.unwrap().show_submission_url;

    io.println(&format!(
        "Submitting... \nYou can find your submission here: {}",
        &submission_url
    ));

    match client.wait_for_submission(&submission_url) {
        Ok(_submission_finished) => io.println("Submission finished"),
        Err(_err) => io.println(""), //io.println(&format!("Submission failed with message {:#?}", err))
    }
}

fn into_locale(arg: &str) -> Result<Language> {
    Language::from_locale(arg)
        .or_else(|| Language::from_639_1(arg))
        .or_else(|| Language::from_639_3(arg))
        .with_context(|| format!("Invalid locale: {}", arg))
}
fn into_url(arg: &str) -> Result<Url> {
    Url::parse(arg).with_context(|| format!("Failed to parse url {}", arg))
}
