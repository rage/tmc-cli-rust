use super::command_util;
use super::command_util::*;
use crate::interactive;
use crate::io_module::Io;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::cmp::min;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tmc_client::ClientError;
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
    let course_result = command_util::get_course_by_name(client, name_select);
    if course_result.is_none() {
        io.println("Could not find course with that name");
        return;
    }
    let course = course_result.unwrap();

    // check if download_folder was given as an argument, otherwise use course name
    /*let mut course_path = env::current_dir().unwrap();
    if let Some(download_folder) = download_folder_arg {
        course_path.push(download_folder);
    } else {
        course_path.push(name_select);
    }

    io.println("");

    let mut course_config_path = PathBuf::from(&course_path);
    course_config_path.push(course_config::COURSE_CONFIG_FILE_NAME);

    match course_config::load_course_config(&course_config_path.as_path()) {
        //if .tmc.json file exists, assume we're updating
        Ok(config) => {
            match client.get_course_exercises(course.id) {
                Ok(mut exercises) => {
                    // collect exercise id's
                    let mut exercise_ids = Vec::<usize>::new();
                    for exercise in &exercises {
                        // filter disabled and locked exercises
                        if !exercise.disabled && exercise.unlocked {
                            exercise_ids.push(exercise.id);
                        }
                    }
                    //get exercise details containing checksums
                    let exercises_details = match client.get_exercise_details(exercise_ids) {
                        Ok(details) => details,
                        Err(_) => {
                            println!("Failed to get exercise details from tmc_client");
                            return;
                        }
                    };

                    let mut exercises_id_to_download = Vec::<usize>::new();
                    for exercise_details in exercises_details {
                        let mut skip = false;
                        for local_exercise_details in &config.course.exercises {
                            // If an exercise with matching id AND matching checksum is found, skip it.
                            if exercise_details.id == local_exercise_details.id
                                && exercise_details.checksum == local_exercise_details.checksum
                            {
                                skip = true;
                            }
                        }
                        //either the exercise is new or the local version needs to be updated
                        if !skip {
                            exercises_id_to_download.push(exercise_details.id);
                        }
                    }

                    //compile result
                    exercises.retain(|exercise| {
                        let mut keep = false;
                        for exercise_id in &exercises_id_to_download {
                            if exercise_id == &exercise.id {
                                keep = true;
                            }
                        }
                        keep
                    });

                    io.println(&parse_download_result(client.download_or_update_exercises(
                        get_download_params(PathBuf::from(&course_path), exercises),
                    )));
                }
                Err(error) => io.println(&error),
            }
        }
        Err(_) => {*/
    //if .tmc.json is missing, assume it's the first download case for given course

    let pathbuf = if currentdir {
        std::env::current_dir().unwrap()
    } else {
        crate::config::get_tmc_dir(PLUGIN).unwrap()
    };

    match client.get_course_exercises(course.id) {
        Ok(exercises) => {
            let exercise_ids: Vec<usize> = exercises.iter().map(|t| t.id).collect();

            let mut manager = ProgressBarManager::new(exercise_ids.len() * 2 + 1);
            manager.start();

            let result = client.download_or_update_exercises(&exercise_ids, pathbuf.as_path());

            manager.join();

            io.println(&parse_download_result(result))
        }
        Err(error) => io.println(&error),
    }
    /*    }
    };*/

    // TODO: Integration tests skip creation of course folder, so we can't save course information there
    //if client.is_test_mode() {
    //    return;
    //}
    //save_course_config(client, PathBuf::from(&course_config_path), course.id);
}

/*fn save_course_config(client: &mut dyn Client, course_config_path: PathBuf, course_id: usize) {
    let course_details = client.get_course_details(course_id).unwrap();
    let organization = client
        .get_organization(&command_util::get_organization().unwrap())
        .unwrap();

    let course_config = CourseConfig {
        username: "My username".to_string(), // TODO: Find out where to get. from client?
        server_address: "Server addr".to_string(), // TODO: Find out where to get. from client?
        course: CourseDetailsWrapper::new(course_details),
        organization,
        local_completed_exercises: vec![],
        properties: HashMap::new(),
    };

    course_config::save_course_information(course_config, course_config_path.as_path());
}*/

fn parse_download_result(result: Result<(), ClientError>) -> String {
    match result {
        Ok(()) => "Download was successful!".to_string(),
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

struct ProgressBarManager {
    max_size: usize,
    percentage_progress: Arc<Mutex<f64>>,
    status_message: Arc<Mutex<String>>,
    is_finished: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl ProgressBarManager {
    fn new(total_updates: usize) -> ProgressBarManager {
        ProgressBarManager {
            max_size: total_updates,
            percentage_progress: Arc::new(Mutex::new(0.0)),
            status_message: Arc::new(Mutex::new("".to_string())),
            is_finished: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    fn start(&mut self) {
        let finished_cb = self.is_finished.clone();
        let percentage_cb = self.percentage_progress.clone();
        let message_cb = self.status_message.clone();
        let callback =
            move |status: tmc_langs_util::progress_reporter::StatusUpdate<ClientUpdateData>| {
                let mut percentage_guard = percentage_cb.lock().expect("Could not lock mutex");
                *percentage_guard = status.percent_done;
                drop(percentage_guard);

                let mut message_guard = message_cb.lock().expect("Could not lock mutex");
                *message_guard = status.message.to_string();
                drop(message_guard);

                if status.finished {
                    finished_cb.store(true, Ordering::Relaxed);
                }
            };

        let max_size = self.max_size;
        let message_t = self.status_message.clone();
        let percentage_t = self.percentage_progress.clone();
        let finished_t = self.is_finished.clone();
        let join_handle = std::thread::spawn(move || {
            ProgressBarManager::progress_thread(max_size, percentage_t, message_t, finished_t)
        });
        self.handle = Some(join_handle);

        tmc_langs_util::progress_reporter::subscribe(callback);
    }

    fn progress_thread(
        max_len: usize,
        percentage_progress: Arc<Mutex<f64>>,
        status_message: Arc<Mutex<String>>,
        is_finished: Arc<AtomicBool>,
    ) {
        let pb = ProgressBar::new(max_len as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{wide_msg} \n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})",
                )
                .progress_chars("#>-"),
        );

        loop {
            let guard = percentage_progress.lock().expect("Could not lock mutex");
            let progress = (*guard as f64) * max_len as f64;
            pb.set_position(min(progress as u64, max_len as u64));
            drop(guard);

            let message_guard = status_message.lock().expect("Could not lock mutex");
            pb.set_message(&*message_guard);
            drop(message_guard);

            if is_finished.load(Ordering::Relaxed) {
                pb.set_position(max_len as u64);
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(1000 / 15));
        }
        //let message_guard = status_message.lock().expect("Could not lock mutex");
        //pb.finish_with_message(&format!("{}",*message_guard));
        //drop(message_guard);
        pb.finish_with_message("Download finished!");
    }

    fn join(&mut self) {
        self.handle.take().map(JoinHandle::join);
    }
}
