use super::command_util;
use super::command_util::*;
use crate::config::course_config;
use crate::config::course_config::{CourseConfig, CourseDetailsWrapper};
use crate::interactive;
use crate::io_module::Io;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use tmc_client::{ClientError, CourseExercise};

pub fn download_or_update(
    io: &mut dyn Io,
    client: &mut dyn Client,
    course_name: Option<&str>,
    download_folder_arg: Option<&str>,
) {
    // Get a client that has credentials
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };

    let courses_result = client.list_courses();
    if courses_result.is_err() {
        io.println("Could not list courses.");
        return;
    }

    let name_select = if let Some(course) = course_name {
        course.to_string()
    } else {
        let courses = courses_result.unwrap();
        let mut course_details = vec![];
        // Course objects from list_courses() don't contain title, so we have to manually get it from CourseDetails
        for c in courses {
            let details = client.get_course_details(c.id);
            course_details.push(details.unwrap());
        }
        course_details.sort_by(|a, b| {
            a.course
                .title
                .to_lowercase()
                .cmp(&b.course.title.to_lowercase())
        });

        let result = interactive::interactive_list(
            "Select your course:",
            course_details
                .iter()
                .map(|course| course.course.title.clone())
                .collect(),
        );
        if result.is_none() {
            io.println("Course selection was interrupted.");
            return;
        }
        let course_title = result.unwrap();

        // find name of course with title
        let mut course_name = "".to_string();
        for c in course_details {
            if c.course.title.trim() == course_title.trim() {
                course_name = c.course.name
            }
        }
        if course_name.is_empty() {
            io.println("Could not find course by title.");
            return;
        }

        course_name
    };

    // Get course by name
    let course_result = command_util::get_course_by_name(client, name_select.clone());
    if course_result.is_none() {
        io.println("Could not find course with that name");
        return;
    }
    let course = course_result.unwrap();

    // check if download_folder was given as an argument, otherwise use course name
    let mut course_path = env::current_dir().unwrap();
    if let Some(download_folder) = download_folder_arg {
        course_path.push(download_folder);
    } else {
        course_path.push(name_select);
    }

    io.println("");

    let mut course_config_path = PathBuf::from(&course_path);
    course_config_path.push(course_config::COURSE_CONFIG_FILE_NAME);

    match course_config::load_course_config(&course_config_path.as_path()) {
        //if .tmc.json file exists, assume we're updating
        Ok(config) => {
            match client.get_course_exercises(course.id) {
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
                        get_download_params(PathBuf::from(&course_path), exercises),
                    )));
                }
                Err(error) => io.println(&error),
            }
        }
        Err(_) => {
            //if .tmc.json is missing, assume it's the first download case for given course
            match client.get_course_exercises(course.id) {
                Ok(exercises) => {
                    io.println(&parse_download_result(client.download_or_update_exercises(
                        get_download_params(PathBuf::from(&course_path), exercises),
                    )))
                }
                Err(error) => io.println(&error),
            }
        }
    };

    // TODO: Integration tests skip creation of course folder, so we can't save course information there
    if client.is_test_mode() {
        return;
    }
    save_course_config(client, PathBuf::from(&course_config_path), course.id);
}

fn save_course_config(client: &mut dyn Client, course_config_path: PathBuf, course_id: usize) {
    let course_details = client.get_course_details(course_id).unwrap();
    let organization = client
        .get_organization(&command_util::get_organization().unwrap())
        .unwrap();

    let course_config = CourseConfig {
        username: "My username".to_string(), // TODO: Find out where to get. from client?
        server_address: "Server addr".to_string(), // TODO: Find out where to get. from client?
        course: CourseDetailsWrapper::new(course_details),
        organization,
        local_completed_exercises: vec![],
        properties: HashMap::new(),
    };

    course_config::save_course_information(course_config, course_config_path.as_path());
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

fn get_download_params(
    course_path: PathBuf,
    exercises: Vec<CourseExercise>,
) -> Vec<(usize, PathBuf)> {
    let mut download_params = Vec::new();
    for exercise in exercises {
        if !exercise.disabled && exercise.unlocked {
            let mut path = PathBuf::from(&course_path);
            path.push(exercise.name);
            download_params.push((exercise.id, path));
        }
    }
    download_params
}
