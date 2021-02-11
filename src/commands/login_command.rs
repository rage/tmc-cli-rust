use crate::io_module::IO;
use tmc_client::{oauth2::TokenResponse, ClientError, TmcClient};
use crate::config::credentials::Credentials;
use std::path::PathBuf;

pub fn login(io: &mut IO) {
    io.print("username: ");
    let mut username = io.read_line();
    username = username.trim().to_string();

    io.print("password: ");
    let mut password = io.read_line();
    password = password.trim().to_string();

    let mut client = TmcClient::new(
        PathBuf::from("./config"),
        "https://tmc.mooc.fi".to_string(),
        "vscode_plugin".to_string(),
        "1.0.0".to_string(),
    )
    .unwrap();

    let token;
    match client.authenticate("vscode_plugin", username, password) {
        Ok(x) => token = x,
        Err(x) => {
            io.println(explain_login_fail(x));
            return;
        }
    }

    println!("{:?}", token.access_token());
}

fn explain_login_fail(error: ClientError) -> String {
    format!("{:#?}", error)
}
