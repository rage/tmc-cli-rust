use super::command_util::*;
use crate::io_module::IO;
use tmc_client::ClientError;

pub fn list_courses(io: &mut IO) {
    // Get a client that has credentials
    let client_result = get_logged_client();
    if client_result.is_none() {
        io.println("No login found. You need to be logged in to list courses.");
        return;
    }
    let client = client_result.unwrap();

    // Listing courses
    match client.list_courses(&get_organization().unwrap()) {
        Ok(courses) => {
            for course in courses {
                io.println(&course.name);
            }
        }
        Err(ClientError::NotLoggedIn) => {
            io.println("Login token is invalid. Please try logging in again.")
        }
        _ => io.println("Unknown error. Please try again."),
    }
}
