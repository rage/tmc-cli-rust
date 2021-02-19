use super::command_util::*;
use crate::io_module::IO;
use std::path::PathBuf;
use tmc_client::{ClientError, CourseExercise};

pub fn download_or_update(io: &mut IO, course_name: String, download_folder: String) {
    // Get a client that has credentials
    let client_result = get_logged_client();
    if client_result.is_none() {
        io.println("Not logged in. Login before downloading exerises.");
        return;
    }
    let client = client_result.unwrap();

    // Get course by id
    let course_result = get_course_id_by_name(&client, course_name.clone());
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
    match client.get_course_exercises(course_id) {
        Ok(exercises) => parse_download_result(
            io,
            client.download_or_update_exercises(get_download_params(filepath, exercises)),
        ),
        Err(ClientError::NotLoggedIn) => {
            io.println("Login token is invalid. Please try logging in again.")
        }
        _ => io.println("Unknown error. Please try again."),
    }
}

fn parse_download_result(io: &mut IO, result: Result<(), ClientError>) {
    match result {
        Ok(()) => io.println("Download was successful!"),
        Err(ClientError::IncompleteDownloadResult {
            downloaded: successful,
            failed: fail,
        }) => {
            io.print("Incomplete download: [");
            io.print(successful.len().to_string());
            io.print(" / ");
            io.print((successful.len() + fail.len()).to_string());
            io.println("] exercises downloaded. (ie.Target folder already exists)");
        }
        _ => io.println("Some errors may have happened during the download."),
    }
}

fn get_download_params(filepath: String, exercises: Vec<CourseExercise>) -> Vec<(usize, PathBuf)> {
    let mut download_params = Vec::new();
    for exercise in exercises {
        let mut path = filepath.clone();
        path.push_str(&exercise.name);
        download_params.push((exercise.id, PathBuf::from(path)));
    }
    download_params
}
