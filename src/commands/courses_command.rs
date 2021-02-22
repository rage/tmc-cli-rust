use super::command_util::*;
use crate::io_module::Io;
use tmc_client::ClientError;

pub fn list_courses(io: &mut dyn Io) {
    // Get a client that has credentials
    let client_result = get_logged_client();
    if client_result.is_none() {
        io.println("No login found. You need to be logged in to list courses.".to_string());
        return;
    }
    let client = client_result.unwrap();

    // Listing courses
    match client.list_courses(&get_organization().unwrap()) {
        Ok(courses) => {
            for course in courses {
                io.println(course.name.to_string());
            }
        }
        Err(ClientError::NotLoggedIn) => {
            io.println("Login token is invalid. Please try logging in again.".to_string())
        }
        _ => io.println("Unknown error. Please try again.".to_string()),
    }
}
