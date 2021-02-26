use super::command_util::*;
use crate::io_module::Io;

pub fn list_courses(io: &mut dyn Io, client: &mut dyn Client) {
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    }

    let courses_result = client.list_courses();

    match courses_result {
        Ok(course_list) => {
            for course in course_list {
                io.println(&course.name);
            }
        }
        Err(error) => io.println(&error),
    }
}
