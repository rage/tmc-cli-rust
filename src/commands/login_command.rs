use super::command_util::*;
use super::organization_command::set_organization;
use crate::config::Credentials;
use crate::io_module::Io;
use std::result::Result;
use std::string::String;
use tmc_client::ClientError;

pub fn login(io: &mut dyn Io) {
    if is_logged_in() {
        io.println("Already logged in!".to_string());
        return;
    }

    io.print("Email / username: ".to_string());
    let mut username = io.read_line();
    username = username.trim().to_string();

    if username.is_empty() {
        io.println("Username cannot be empty!".to_string());
        return;
    }

    io.print("Password: ".to_string());
    let mut password = io.read_password();
    password = password.trim().to_string();

    io.println("".to_string());

    match authenticate(username, password) {
        Ok(message) => {
            io.println(message);
            if let Err(_err) = set_organization(io) {
                io.println("Could not set organization".to_string());
            }
        }
        Err(message) => io.println(message),
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

#[cfg(test)]
pub struct IoTest<'a> {
    list: &'a mut Vec<String>,
}

#[cfg(test)]
impl IoTest<'_> {
    pub fn buffer_length(&mut self) -> usize {
        self.list.len()
    }

    pub fn buffer_get(&mut self, index: usize) -> String {
        self.list[index].to_string()
    }
}

#[cfg(test)]
impl Io for IoTest<'_> {
    fn read_line(&mut self) -> String {
        "".to_string()
    }

    fn print(&mut self, output: String) {
        print!("{}", output);
        self.list.push(output);
    }

    fn println(&mut self, output: String) {
        println!("{}", output);
        self.list.push(output);
    }

    fn read_password(&mut self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_username_test() {
        let mut v: Vec<String> = Vec::new();
        let mut io = IoTest { list: &mut v };

        assert!(io.read_line().eq(""));

        login(&mut io);

        assert_eq!(2, io.buffer_length());

        if io.buffer_length() == 2 {
            assert!(io
                .buffer_get(1)
                .to_string()
                .eq(&"Username cannot be empty!".to_string()));
        }
    }
}
