use super::command_util;
use super::command_util::CourseConfig;
use super::command_util::*;
use crate::io_module::Io;
use std::env;
use std::path::PathBuf;
use tmc_client::{ClientError, CourseExercise};

pub fn download_or_update(
    io: &mut dyn Io,
    client: &mut dyn Client,
    course_name: String,
    download_folder: String,
) {
    // Get a client that has credentials

    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };

    // Get course by id
    let course_result = get_course_id_by_name(client, course_name);
    if course_result.is_none() {
        io.println("Could not find course by name");
        return;
    }
    let course_id = course_result.unwrap();

    let mut filepath = download_folder.trim().to_string();
    filepath = if filepath.ends_with('/') {
        filepath
    } else {
        format!("{}/", filepath)
    };

    io.println("");

    match client.get_course_exercises(course_id) {
        Ok(exercises) => io.println(&parse_download_result(
            client.download_or_update_exercises(get_download_params(filepath, exercises)),
        )),
        Err(error) => io.println(&error),
    }

    // TODO: Integration tests skip creation of course folder, so we can't save course information there
    if client.is_test_mode() {
        return;
    }

    let course_details = client.get_course_details(course_id).unwrap();
    let organization = client
        .get_organization(&command_util::get_organization().unwrap())
        .unwrap();
    //Generate path for config
    let mut pathbuf = env::current_dir().unwrap();
    pathbuf.push(download_folder);
    pathbuf.push(".tmc.json"); // Make into a constant, also used in submit command

    let course_config = CourseConfig {
        username: "My username".to_string(),       // TODO
        server_address: "Server addr".to_string(), // TODO
        course: CourseDetailsWrapper::new(course_details),
        organization,
        local_completed_exercises: vec![], // TODO
        properties: vec![],                // TODO
    };

    command_util::save_course_information(course_config, pathbuf);
}

fn parse_download_result(result: Result<(), ClientError>) -> String {
    match result {
        Ok(()) => "Download was successful!".to_string(),
        Err(ClientError::IncompleteDownloadResult {
            downloaded: successful,
            failed: fail,
        }) => {
            let done = successful.len().to_string();
            let total = (successful.len() + fail.len()).to_string();
            format!("[{} / {}] exercises downloaded.", done, total)
        }
        _ => "Received an unexpected result from downloading exercises.".to_string(),
    }
}

fn get_download_params(filepath: String, exercises: Vec<CourseExercise>) -> Vec<(usize, PathBuf)> {
    let mut download_params = Vec::new();
    for exercise in exercises {
        if !exercise.disabled && exercise.unlocked {
            let mut path = filepath.clone();
            path.push_str(&exercise.name);
            download_params.push((exercise.id, PathBuf::from(path)));
        }
    }
    download_params
}
