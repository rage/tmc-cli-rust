use super::command_util;
use super::command_util::{get_exercise_id_by_name, load_course_config, Client, CourseConfig};
use crate::io_module::Io;
use anyhow::{Context, Result};
use isolang::Language;
use std::env;
use url::Url;

pub fn submit(io: &mut dyn Io, client: &mut dyn Client) {
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };
    // Assuming we are in exercise directory
    let mut pathbuf = env::current_dir().unwrap();
    pathbuf.pop(); // we go to the course directory
    pathbuf.push(".tmc.json"); // TODO: make .tmc.json into a constant
    match command_util::load_course_config(pathbuf.as_path()) {
        Ok(course_config) => submit_logic(io, client, course_config),
        Err(err) => {
            println!("{}", err);
        }
    }
}
fn submit_logic(io: &mut dyn Io, client: &mut dyn Client, course_config: CourseConfig) {
    println!("Your username: {}", course_config.username);
    println!("Course name: {}", course_config.course.name);
    let locale = into_locale("fin").unwrap();
    let pathbuf = env::current_dir().unwrap();

    let course_id = course_config.course.id;
    let exercise_id = get_exercise_id_by_name(
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
    let generated_url = &course_config.course.exercises[exercise_id].return_url;
    let submission_url = Url::parse(&generated_url).unwrap();

    let submission_path = pathbuf.as_path();
    //file_util::lock!(submission_path);

    let new_submission = client.submit(submission_url, submission_path, Some(locale));
    io.println(&format!(
        "Submitting exercise {}",
        &course_config.course.name,
    ));
    io.println(&format!("{:?}", &new_submission));
    let submission_finished = client.wait_for_submission(&generated_url);
    io.println(&format!("{:?}", submission_finished));
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
