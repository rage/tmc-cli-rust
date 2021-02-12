use crate::config::Credentials;
use crate::io_module::IO;
use std::path::PathBuf;
use tmc_client::TmcClient;

pub fn logout(io: &mut IO) {
    let _client = TmcClient::new(
        PathBuf::from("./config"),
        "https://tmc.mooc.fi".to_string(),
        "vscode_plugin".to_string(),
        "1.0.0".to_string(),
    )
    .unwrap();
    let credentials = Credentials::load("vscode_plugin");

    match credentials {
        Ok(ok) => {
            if let Some(credentials) = ok {
                credentials.remove().unwrap();
                io.println("Logged out successfully.");
            } else {
                // No credentials file or no login in it
                io.println("No login was found.");
            }
        }
        Err(_) => {
            io.println("Error with credentials file");
        }
    }
}
