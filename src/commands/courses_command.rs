use super::command_util::*;
use crate::io_module::IO;
use tmc_client::TmcClient;

pub fn list_courses(io: &mut IO) {
    // Login functinality
    let mut client = get_client();
    if !is_logged_in() {
        io.println("No login found. You need to be logged in to set organization.");
        return;
    }
    let credentials = get_credentials().unwrap();
    client.set_token(credentials.token()).unwrap();

    let slug = get_organization().unwrap();

    // Listing courses
    for course in client.list_courses(&slug).unwrap() {
        io.println(&course.name);
    }
}
