use super::command_util;
use super::command_util::*;
use crate::interactive;
use crate::io_module::Io;
use crate::progress_reporting::progress_manager::ProgressBarManager;
use indicatif::ProgressStyle;
use tmc_langs::ClientError;
use tmc_langs::ClientUpdateData;

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

    let courses_result = client.list_courses();
    if courses_result.is_err() {
        io.println("Could not list courses.");
        return;
    }

    let name_select = if let Some(course) = course_name {
        course.to_string()
    } else {
        let courses = courses_result.unwrap();
        let mut course_details = vec![];
        // Course objects from list_courses() don't contain title, so we have to manually get it from CourseDetails
        for c in courses {
            let details = client.get_course_details(c.id);
            course_details.push(details.unwrap());
        }
        course_details.sort_by(|a, b| {
            a.course
                .title
                .to_lowercase()
                .cmp(&b.course.title.to_lowercase())
        });

        let result = interactive::interactive_list(
            "Select your course:",
            course_details
                .iter()
                .map(|course| course.course.title.clone())
                .collect(),
        );
        if result.is_none() {
            io.println("Course selection was interrupted.");
            return;
        }
        let course_title = result.unwrap();

        // find name of course with title
        let mut course_name = "".to_string();
        for c in course_details {
            if c.course.title.trim() == course_title.trim() {
                course_name = c.course.name
            }
        }
        if course_name.is_empty() {
            io.println("Could not find course by title.");
            return;
        }

        course_name
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

    match client.get_course_exercises(course.id) {
        Ok(exercises) => {
            let exercise_ids: Vec<usize> = exercises
                .iter()
                .filter(|t| !t.disabled && t.unlocked)
                .map(|t| t.id)
                .collect();

            let progress_style = ProgressStyle::default_bar()
                .template(
                    "{wide_msg} \n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})",
                )
                .progress_chars("#>-");

            // start manager for 1 event: tmc_langs::download_or_update_exercises
            let mut manager = ProgressBarManager::new(progress_style, 1, client.is_test_mode());
            manager.start::<ClientUpdateData>();

            let result = client.download_or_update_exercises(&exercise_ids, pathbuf.as_path());

            manager.join();

            io.println(&parse_download_result(result))
        }
        Err(error) => io.println(&error),
    }
}

fn parse_download_result(result: Result<String, ClientError>) -> String {
    match result {
        Ok(path) => format!("{}/\nDownload was successful!", path),
        Err(ClientError::IncompleteDownloadResult {
            downloaded: successful,
            failed: fail,
        }) => {
            let done = successful.len().to_string();
            let total = (successful.len() + fail.len()).to_string();
            format!("[{} / {}] exercises downloaded.", done, total)
        }
        _ => "Received an unexpected result from downloading exercises.".to_string(),
    }
}
