use crate::config::Credentials;
use crate::io_module::IO;
use std::path::PathBuf;
use tmc_client::TmcClient;

pub fn download_or_update(io: &mut IO) {
    // Ask user for course id and destination folder for exercises
    io.print("Course id: ");
    let course_id = io.read_line();
    let course_id: usize = course_id.trim().parse().unwrap();

    io.print("Destination Folder: ");
    let mut filepath = io.read_line();
    filepath = filepath.trim().to_string();
    filepath = if filepath.ends_with('/') {
        filepath
    } else {
        format!("./{}/", filepath)
    };

    let mut client = TmcClient::new(
        PathBuf::from("./config"),
        "https://tmc.mooc.fi".to_string(),
        "vscode_plugin".to_string(),
        "1.0.0".to_string(),
    )
    .unwrap();

    // Load login credentials if they exist in the file
    let credentials = Credentials::load("vscode_plugin").unwrap();
    if let Some(credentials) = credentials {
        client.set_token(credentials.token()).unwrap();
    } else {
        io.println("Not logged in!");
        return;
    }

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
