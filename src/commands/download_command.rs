use super::command_util;
use super::command_util::*;
use crate::config::course_config;
use crate::config::course_config::{CourseConfig, CourseDetailsWrapper};
use crate::io_module::Io;
use std::collections::HashMap;
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

    //io.print("Destination Folder: ");
    //let mut filepath = io.read_line();
    let mut filepath = download_folder.trim().to_string();
    filepath = if filepath.ends_with('/') {
        filepath
    } else {
        format!("{}/", filepath)
    };

    io.println("");

    let mut course_config_path = filepath.clone();
    course_config_path.push_str(".tmc.json");
    match course_config::load_course_config(&PathBuf::from(course_config_path)) {
        //if .tmc.json file exists, assume we're updating
        Ok(config) => {
            match client.get_course_exercises(course_id) {
                Ok(mut exercises) => {
                    // collect exercise id's
                    let mut exercise_ids = Vec::<usize>::new();
                    for exercise in &exercises {
                        // filter disabled and locked exercises
                        if !exercise.disabled && exercise.unlocked {
                            exercise_ids.push(exercise.id);
                        }
                    }
                    //get exercise details containing checksums
                    let exercises_details = match client.get_exercise_details(exercise_ids) {
                        Ok(details) => details,
                        Err(_) => {
                            println!("Failed to get exercise details from tmc_client");
                            return;
                        }
                    };

                    let mut exercises_id_to_download = Vec::<usize>::new();
                    for exercise_details in exercises_details {
                        let mut skip = false;
                        for local_exercise_details in &config.course.exercises {
                            // If an exercise with matching id AND matching checksum is found, skip it.
                            if exercise_details.id == local_exercise_details.id
                                && exercise_details.checksum == local_exercise_details.checksum
                            {
                                skip = true;
                            }
                        }
                        //either the exercise is new or the local version needs to be updated
                        if !skip {
                            exercises_id_to_download.push(exercise_details.id);
                        }
                    }

                    //compile result
                    exercises.retain(|exercise| {
                        let mut keep = false;
                        for exercise_id in &exercises_id_to_download {
                            if exercise_id == &exercise.id {
                                keep = true;
                            }
                        }
                        keep
                    });

                    io.println(&parse_download_result(client.download_or_update_exercises(
                        get_download_params(filepath, exercises),
                    )));
                }
                Err(error) => io.println(&error),
            }
        }
        Err(_) => {
            //if .tmc.json is missing, assume it's the first download case for given course
            match client.get_course_exercises(course_id) {
                Ok(exercises) => io.println(&parse_download_result(
                    client.download_or_update_exercises(get_download_params(filepath, exercises)),
                )),
                Err(error) => io.println(&error),
            }
        }
    };

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
    pathbuf.push(course_config::COURSE_CONFIG_FILE_NAME);

    let course_config = CourseConfig {
        username: "My username".to_string(), // TODO: Find out where to get. from client?
        server_address: "Server addr".to_string(), // TODO: Find out where to get. from client?
        course: CourseDetailsWrapper::new(course_details),
        organization,
        local_completed_exercises: vec![],
        properties: HashMap::new(),
    };

    course_config::save_course_information(course_config, pathbuf);
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
