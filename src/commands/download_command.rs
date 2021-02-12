use crate::io_module::IO;
use tmc_client::TmcClient;
use crate::config::Credentials;
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

    // Lataa kirjautumistiedot olettaen että kirjautumistiedot sisältävä 
    // tiedosto on olemassa. Kaatuu jos ei ole!
    let mut credentials = Credentials::load("vscode_plugin").unwrap();
    if let Some(credentials) = credentials {
        client.set_token(credentials.token());
    } else {
        io.println("Not logged in!");
        return;
    }


    //Rakennetaan vektori johon laitetaan tehtävä_id & tallennuslokaatio pareja
    let mut download_params = Vec::new();

    let exercises = client.get_course_exercises(course_id).unwrap();
    for exercise in exercises {
        let mut path = filepath.clone();
        path.push_str(&exercise.name);
        download_params.push((exercise.id, PathBuf::from(path)));
    }

    let _ = client.download_or_update_exercises(download_params);
}
