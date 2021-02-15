use crate::config::Credentials;
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
