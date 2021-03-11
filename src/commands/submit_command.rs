use super::command_util;
use super::command_util::CourseConfig;
use std::env;

pub fn submit() {
    // Assuming we are in exercise directory
    let mut pathbuf = env::current_dir().unwrap();
    pathbuf.pop(); // we go to the course directory
    pathbuf.push(".tmc.json"); // TODO: make .tmc.json into a constant
    match command_util::load_course_config(pathbuf.as_path()) {
        Ok(course_config) => submit_logic(course_config),
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn submit_logic(course_config: CourseConfig) {
    println!("Your username: {}", course_config.username);
    println!("Course name: {}", course_config.course.name);

    // Write submit_command logic here
    // ...
}
