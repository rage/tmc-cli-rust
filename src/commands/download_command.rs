use crate::io_module::IO;
use tmc_client::TmcClient;

use std::path::PathBuf;

pub fn download_or_update(io: &mut IO) {
  // Pyydetään käyttäjältä Kurssin id ja tehtävien tallennus kansio
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



    //Väliaikainen viritelmä------

    io.print("username: ");
    let mut username = io.read_line();
    username = username.trim().to_string();

    io.print("password: ");
    let mut password = io.read_line();
    password = password.trim().to_string();

    client.authenticate("vscode_plugin", username, password).unwrap();

    //----------------------------

    //Rakennetaan vektori johon laitetaan tehtävä_id & tallennuslokaatio pareja
    let mut download_params = Vec::new();

    let exercises = client.get_course_exercises(course_id).unwrap();
    for exercise in exercises {
        let mut path = filepath.clone();
        path.push_str(&exercise.name);
        download_params.push((exercise.id, PathBuf::from(path)));
    }

    client.download_or_update_exercises(download_params);
}