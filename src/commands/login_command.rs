use crate::config::Credentials;
use crate::io_module::IO;
use std::path::PathBuf;
use tmc_client::{ClientError, TmcClient};

const PLUGIN: &str = "vscode_plugin";

pub fn login(io: &mut IO) {
    if is_logged_in() {
        io.println("Already logged in!");
        return;
    }

    io.print("email / username: ");
    let mut username = io.read_line();
    username = username.trim().to_string();

    if username.is_empty() {
        io.println("Username cannot be empty!");
        return;
    }

    io.print("password: ");
    let mut password = io.read_password();
    password = password.trim().to_string();

    io.println("");

    io.println(authenticate(username, password));
}

fn authenticate(username: String, password: String) -> &'static str {
    let mut client = get_client();

    let token;

    match client.authenticate(PLUGIN, username, password) {
        Ok(x) => token = x,
        Err(x) => return explain_login_fail(x),
    }

    if Credentials::save(PLUGIN, token).is_ok() {
        return "Succesfully logged in!";
    };

    "Something funny happened"
}

fn get_client() -> TmcClient {
    TmcClient::new(
        PathBuf::from("./config"),
        "https://tmc.mooc.fi".to_string(),
        PLUGIN.to_string(),
        "1.0.0".to_string(),
    )
    .unwrap()
}

fn get_credentials() -> Option<Credentials> {
    // Load login credentials if they exist in the file
    Credentials::load(PLUGIN).unwrap()
}

fn is_logged_in() -> bool {
    get_credentials().is_some()
}

fn explain_login_fail(error: ClientError) -> &'static str {
    let res = format!("{:?}", error);

    if res.contains("The provided authorization grant is invalid, expired, revoked, does not match the redirection URI used in the authorization request, or was issued to another client.") {
        return "Invalid username or password";
    }

    "Something funny happened"
}
