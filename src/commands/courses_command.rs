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

    /*-- START [Temporary fix] until slug is saved in config --*/
    io.print("(temporary) Choose organization by writing its slug: ");
    let mut slugs = io.read_line();
    let slug = slugs.trim();
    /*-- END [Temporary fix] --*/

    // Listing courses
    for course in client.list_courses(slug).unwrap() {
        io.println(&course.name);
    }
}
