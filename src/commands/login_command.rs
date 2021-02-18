use super::command_util::*;
use super::organization_command::set_organization;
use crate::config::Credentials;
use crate::io_module::IO;
use std::path::PathBuf;
use std::result::Result;
use std::string::String;
use tmc_client::{ClientError, TmcClient};

pub fn login(io: &mut IO) {
    if is_logged_in() {
        io.println("Already logged in!");
        return;
    }

    io.print("Email / username: ");
    let mut username = io.read_line();
    username = username.trim().to_string();

    if username.is_empty() {
        io.println("Username cannot be empty!");
        return;
    }

    io.print("Password: ");
    let mut password = io.read_password();
    password = password.trim().to_string();

    io.println("");

    match authenticate(username, password) {
        Ok(message) => {
            io.println(message);
            set_organization(io);
        },
        Err(message) => io.println(message)
    }

}

fn authenticate(username: String, password: String) -> Result<String, String> {
    let mut client = get_client();

    let token;

    match client.authenticate(PLUGIN, username, password) {
        Ok(x) => token = x,
        Err(x) => return Err(explain_login_fail(x).to_string()),
    }

    if Credentials::save(PLUGIN, token).is_ok() {
        return Ok("Succesfully logged in!".to_string());
    };

    Err("Something funny happened".to_string())
}

fn explain_login_fail(error: ClientError) -> &'static str {
    let res = format!("{:?}", error);

    if res.contains("The provided authorization grant is invalid, expired, revoked, does not match the redirection URI used in the authorization request, or was issued to another client.") {
        return "Invalid username or password";
    }

    "Something funny happened"
}
