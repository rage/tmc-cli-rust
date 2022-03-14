use std::path::PathBuf;
use std::process::Command;

use super::util;
use super::util::{get_organization, get_projects_dir, Client};
use crate::interactive;
use crate::io::{Io, PrintColor};
use crate::progress_reporting;
use crate::progress_reporting::ProgressBarManager;
use tmc_langs::ClientUpdateData;
use tmc_langs::Course;
use tmc_langs::DownloadResult;

// Downloads course exercises
// course_name as None will trigger interactive menu for selecting a course
// currentdir determines if course should be downloaded to current directory or central project directory
// Will run in privileged stage if needed on Windows.
pub fn download_or_update(
    io: &mut dyn Io,
    client: &mut dyn Client,
    course_name: Option<&str>,
    currentdir: bool,
) {
    if get_organization().is_none() {
        io.println(
            "No organization found. Run 'tmc organization' first.",
            PrintColor::Failed,
        );
        return;
    }

    io.println("Fetching courses...", PrintColor::Normal);
    let courses = client.list_courses();
    if courses.is_err() {
        io.println("Could not list courses.", PrintColor::Failed);
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
                io.println(&msg, PrintColor::Failed);
                return;
            }
        }
    };

    // Get course by name
    let course_result = match util::get_course_by_name(client, name_select) {
        Ok(result) => result,
        Err(msg) => {
            io.println(&msg, PrintColor::Failed);
            return;
        }
    };

    if course_result.is_none() {
        io.println("Could not find course with that name", PrintColor::Failed);
        return;
    }
    let course = course_result.unwrap();
    let pathbuf = if currentdir {
        std::env::current_dir().unwrap()
    } else {
        get_projects_dir()
    };

    let tmp_course = course.name.clone();
    let tmp_path = pathbuf.clone();
    let tmp_path = tmp_path.to_str().unwrap();
    match download_exercises(pathbuf, client, course) {
        Ok(msg) => io.println(&format!("\n{}", msg), PrintColor::Success),
        Err(msg) => {
            let os = std::env::consts::OS;
            if os == "windows" && msg.contains("Failed to create file") {
                io.println(
                    "Starting new cmd with administrator privileges...",
                    PrintColor::Normal,
                );
                let temp_file_path = get_projects_dir();
                let temp_file_path = temp_file_path.join("temp.txt");
                std::fs::write(temp_file_path, format!("{};{}", tmp_path, tmp_course)).unwrap();
                Command::new("cmd")
                    .args(&[
                        "/C",
                        "powershell",
                        "-Command",
                        "Start-Process",
                        "tmc.exe",
                        "elevateddownload",
                        "-Verb",
                        "RunAs",
                    ])
                    .spawn()
                    .expect("launch failure");
            } else {
                io.println(&msg, PrintColor::Failed)
            }
        }
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
            let exercise_ids: Vec<u32> = exercises
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
                            downloaded,
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

                            if !downloaded.is_empty() {
                                res.push_str(&format!(
                                    "\n\nSuccessful downloads saved to {}\\",
                                    pathbuf.to_str().unwrap()
                                ));
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

    Ok(format!(
        "Exercises downloaded successfully to {}\\",
        pathbuf.to_str().unwrap()
    ))
}
pub fn elevated_download(io: &mut dyn Io, client: &mut dyn Client) {
    use std::io::prelude::*;
    let temp_file_path = get_projects_dir();
    let temp_file_path = temp_file_path.join("temp.txt");
    let mut file = std::fs::File::open(temp_file_path.clone()).unwrap();
    let mut params = String::new();
    file.read_to_string(&mut params).unwrap();
    std::fs::remove_file(temp_file_path).unwrap();
    let split = params.split(';');
    let vec = split.collect::<Vec<&str>>();
    let path = PathBuf::from(vec[0]);
    let name_select = String::from(vec[1]);

    // Get course by name
    let course_result = match util::get_course_by_name(client, name_select) {
        Ok(result) => result,
        Err(msg) => {
            io.println(&msg, PrintColor::Failed);
            return;
        }
    };

    if course_result.is_none() {
        io.println("Could not find course with that name", PrintColor::Failed);
        return;
    }
    let course = course_result.unwrap();
    io.println("", PrintColor::Normal);
    match download_exercises(path, client, course) {
        Ok(msg) => io.println(&msg, PrintColor::Success),
        Err(msg) => io.println(&msg, PrintColor::Failed),
    }
    pause();
}
fn pause() {
    use std::io;
    use std::io::prelude::*;
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "Press any enter to continue...").unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
