use crate::config::{Credentials, Config};
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

pub fn get_credentials() -> Option<Credentials> {
    // Load login credentials if they exist in the file
    Credentials::load(PLUGIN).unwrap()
}

pub fn is_logged_in() -> bool {
    get_credentials().is_some()
}

#[allow(dead_code)]
pub fn get_organization() -> Option<String> {
    let config = Config::load(PLUGIN).unwrap();
    
    Some(config.get_value("organization").unwrap())
}

pub fn set_organization(org: &str) -> Result<(), &'static str> {
    let mut config = Config::load(PLUGIN).unwrap();

    if let Err(_err) = config.change_value("organization", org) {
        return Err("Organization could not be changed");
    }

    if let Err(_err) = config.save() {
        return Err("Problem saving configurations");
    }
    Ok(())
}
