use std::path::PathBuf;

use super::command_util;
use super::command_util::*;
use crate::interactive;
use crate::io_module::Io;
use crate::progress_reporting;
use crate::progress_reporting::ProgressBarManager;
use tmc_client::Course;
use tmc_langs::ClientUpdateData;
use tmc_langs::DownloadResult;

// Downloads course exercises
// course_name as None will trigger interactive menu for selecting a course
// currentdir determines if course should be downloaded to current directory or central project directory
pub fn download_or_update(
    io: &mut dyn Io,
    client: &mut dyn Client,
    course_name: Option<&str>,
    currentdir: bool,
) {
    // Get a client that has credentials
    if let Err(error) = client.load_login() {
        io.println(&error);
        return;
    };

    io.println("Fetching courses...");
    let courses = client.list_courses();
    if courses.is_err() {
        io.println("Could not list courses.");
        return;
    }

    let mut courses = courses
        .unwrap()
        .iter()
        .map(|course| client.get_course_details(course.id).unwrap())
        .collect::<Vec<_>>();

    courses.sort_by(|a, b| {
        a.course
            .title
            .to_lowercase()
            .cmp(&b.course.title.to_lowercase())
    });

    let name_select = if let Some(course) = course_name {
        course.to_string()
    } else {
        match get_course_name(
            courses
                .iter()
                .map(|course| course.course.title.clone())
                .collect(),
        ) {
            Ok(course) => courses
                .iter()
                .find(|c| c.course.title == course)
                .unwrap()
                .course
                .name
                .clone(),
            Err(msg) => {
                io.println(&msg);
                return;
            }
        }
    };

    // Get course by name
    let course_result = match command_util::get_course_by_name(client, name_select) {
        Ok(result) => result,
        Err(msg) => {
            io.println(&msg);
            return;
        }
    };

    if course_result.is_none() {
        io.println("Could not find course with that name");
        return;
    }
    let course = course_result.unwrap();

    let pathbuf = if currentdir {
        std::env::current_dir().unwrap()
    } else {
        get_projects_dir()
    };

    match download_exercises(pathbuf, client, course) {
        Ok(msg) | Err(msg) => io.println(&format!("\n{}", msg)),
    }
}

pub fn get_course_name(courses: Vec<String>) -> Result<String, String> {
    let result = interactive::interactive_list("Select your course:", courses);

    match result {
        Some(course) => {
            if course.is_empty() {
                Err("Could not find a course by the given title".to_string())
            } else {
                Ok(course)
            }
        }
        None => Err("Course selection was interrupted".to_string()),
    }
}

pub fn download_exercises(
    pathbuf: PathBuf,
    client: &mut dyn Client,
    course: Course,
) -> Result<String, String> {
    match client.get_course_exercises(course.id) {
        Ok(exercises) => {
            let exercise_ids: Vec<usize> = exercises
                .iter()
                .filter(|t| !t.disabled && t.unlocked)
                .map(|t| t.id)
                .collect();

            if exercise_ids.is_empty() {
                return Err(format!(
                    "No valid exercises found for course '{}'",
                    course.title
                ));
            }

            // start manager for 1 event: tmc_langs::download_or_update_exercises
            let mut manager = ProgressBarManager::new(
                progress_reporting::get_default_style(),
                1,
                client.is_test_mode(),
            );
            manager.start::<ClientUpdateData>();

            let result = client.download_or_update_exercises(&exercise_ids, pathbuf.as_path());

            match result {
                Ok(download_result) => {
                    manager.join();
                    match download_result {
                        DownloadResult::Success {
                            downloaded: _,
                            skipped: _,
                        } => {
                            if client.is_test_mode() {
                                return Ok("Download was successful!".to_string());
                            }
                        }
                        DownloadResult::Failure {
                            downloaded: _,
                            skipped: _,
                            failed,
                        } => {
                            let mut res = String::from("");

                            for (id, messages) in failed {
                                res.push_str(&format!(
                                    "\nFailed to download exercise: '{}'",
                                    id.exercise_slug
                                ));
                                for message in messages {
                                    res.push_str(&format!("\n    with message: '{}'", message));
                                }
                            }

                            return Err(res);
                        }
                    }
                }
                Err(err) => {
                    manager.force_join();
                    return Err(format!("Error: {}", err));
                }
            }
        }
        Err(error) => return Err(format!("Error: {}", error)),
    }

    Ok("Exercises downloaded succesfully!".to_string())
}
