use crate::io_module::IO;
use std::path::PathBuf;
use tmc_client::TmcClient;
use crate::config::Credentials;

pub fn logout(io: &mut IO) {
    let mut client = TmcClient::new(
        PathBuf::from("./config"),
        "https://tmc.mooc.fi".to_string(),
        "vscode_plugin".to_string(),
        "1.0.0".to_string(),
    )
    .unwrap();
    let mut credentials = Credentials::load("vscode_plugin");

    match credentials {
        Ok(ok) => if let Some(credentials) = ok {
            credentials.remove();
            io.println("Logged out successfully.");
        } else {
            // No credentials file or no login in it
            io.println("No login was found.");
        },
        Err(error) => {
            io.println("Error with credentials file");
        }
    }
}
