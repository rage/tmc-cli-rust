use super::command_util::*;
use crate::config::Credentials;
use crate::io_module::IO;
use std::path::PathBuf;
use tmc_client::TmcClient;

pub fn download_or_update(io: &mut IO, course_name: String, download_folder: String) {
    let mut client = get_client();
    // Login functinality
    if !is_logged_in() {
        io.println("Not logged in. Login before downloading exerises");
        return;
    }

    // Load login credentials if they exist in the file
    let credentials = get_credentials().unwrap();
    client.set_token(credentials.token()).unwrap();

    let slug = get_organization().unwrap();

    // Match course name to an id
    let mut course_id = 0;
    let mut found = false;
    let courses = client.list_courses(&slug).unwrap();
    for course in courses {
        if course.name == course_name {
            course_id = course.id;
            found = true;
            //break;
        }
    }
    if !found {
        io.println("Could not find course by name");
        return;
    }

    //io.print("Destination Folder: ");
    //let mut filepath = io.read_line();
    let mut filepath = download_folder.trim().to_string();
    filepath = if filepath.ends_with('/') {
        filepath
    } else {
        format!("{}/", filepath)
    };

    // Build a vector for exercise id and saving location pairs
    let mut download_params = Vec::new();

    let exercises = client.get_course_exercises(course_id).unwrap();
    for exercise in exercises {
        let mut path = filepath.clone();
        path.push_str(&exercise.name);
        download_params.push((exercise.id, PathBuf::from(path)));
    }

    let _ = client.download_or_update_exercises(download_params);
}
