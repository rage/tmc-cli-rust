use crate::config::{Config, Credentials};
use std::path::PathBuf;
use tmc_client::TmcClient;

pub const PLUGIN: &str = "vscode_plugin";

pub fn get_client() -> TmcClient {
    TmcClient::new(
        PathBuf::from("./config"),
        "https://tmc.mooc.fi".to_string(),
        PLUGIN.to_string(),
        "1.0.0".to_string(),
    )
    .unwrap()
}

pub fn get_logged_client() -> Option<TmcClient> {
    let mut client = get_client();

    if let Some(credentials) = get_credentials() {
        if let Err(_err) = client.set_token(credentials.token()) {
            println!("Could not got logged client");
            return None;
        }
        //client.set_token(credentials.token());
        return Some(client);
    }

    None
}

pub fn get_credentials() -> Option<Credentials> {
    // Load login credentials if they exist in the file
    Credentials::load(PLUGIN).unwrap()
}

pub fn is_logged_in() -> bool {
    get_credentials().is_some()
}

// Returns slug of organization as String (if successful)
#[allow(dead_code)]
pub fn get_organization() -> Option<String> {
    let config = Config::load(PLUGIN).unwrap();

    Some(config.get_value("organization").unwrap())
}

pub fn set_organization(org: &str) -> Result<(), &'static str> {
    let mut config = Config::new(PLUGIN);

    if let Err(_err) = config.change_value("organization", org) {
        return Err("Organization could not be changed");
    }

    if let Err(_err) = config.save() {
        return Err("Problem saving configurations");
    }
    Ok(())
}

pub fn get_course_id_by_name(client: &TmcClient, course_name: String) -> Option<usize> {
    let slug = get_organization().unwrap();

    match client.list_courses(&slug) {
        Ok(courses) => {
            for course in courses {
                if course.name == course_name {
                    return Some(course.id);
                }
            }
            None
        }
        //Err(ClientError::NotLoggedIn) => /* TODO: pass this information to caller */,
        _ => None,
    }
}
