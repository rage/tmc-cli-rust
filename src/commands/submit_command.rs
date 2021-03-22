use super::command_util::*;
use crate::config::course_config;
use crate::config::course_config::CourseConfig;
use crate::io_module::Io;
use anyhow::{Context, Result};
use std::env;
use tmc_langs::Language;
use url::Url;

pub fn submit(io: &mut dyn Io, client: &mut dyn Client) {
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    }

    //file_util::lock!(submission_path);

    let mut pathbuf = env::current_dir().unwrap();
    pathbuf.pop();
    pathbuf.push(course_config::COURSE_CONFIG_FILE_NAME);

    if let Ok(config) = course_config::load_course_config(pathbuf.as_path()) {
        submit_logic(io, client, &config);
    } else {
        io.println("Current directory does not contain any exercise")
    }
}
fn submit_logic(io: &mut dyn Io, client: &mut dyn Client, course_config: &CourseConfig) {
    let locale = into_locale("fin").unwrap();
    let current_dir = env::current_dir().unwrap();

    let exercise_name = current_dir
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let submission_url;
    match course_config::get_exercise_by_name(course_config, &exercise_name) {
        Some(exercise) => submission_url = into_url(&exercise.return_url).unwrap(),
        None => {
            io.println("Current directory does not contain any exercise");
            return;
        }
    }

    let submission_path = current_dir.as_path();

    //file_util::lock!(submission_path);
    let new_submission = client.submit(submission_url, submission_path, Some(locale));
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
