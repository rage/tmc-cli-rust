use super::command_util;
use super::command_util::{ask_exercise_interactive, find_course_config_for_exercise, Client};
use crate::io_module::{Io, PrintColor};
use crate::progress_reporting;
use crate::progress_reporting::ProgressBarManager;
use isolang::Language;
use reqwest::Url;
use tmc_langs::ClientUpdateData;

/// Sends the course exercise submission with paste message to the server.
/// Path to the exercise can be given as a parameter or
/// the user can run the command in the exercise folder.
///
/// # Errors
/// Returns an error if no exercise found on given path or current folder.
/// Returns an error if user is not logged in.
pub fn paste(io: &mut dyn Io, client: &mut dyn Client, path: &str) {
    let mut exercise_name = "".to_string();
    let mut course_config = None;
    let mut exercise_dir = std::path::PathBuf::new();

    if let Err(error) = find_course_config_for_exercise(
        &mut exercise_name,
        &mut course_config,
        &mut exercise_dir,
        path,
    ) {
        io.println(&error, PrintColor::Failed);
        return;
    }

    if course_config.is_none() {
        if client.is_test_mode() {
            io.println("Could not load course config file. Check that exercise path leads to an exercise folder inside a course folder.", PrintColor::Failed);
            return;
        }
        // Did not find course config, use interactive selection if possible
        match ask_exercise_interactive(&mut exercise_name, &mut exercise_dir, &mut course_config) {
            Ok(()) => (),
            Err(msg) => {
                io.println(&msg, PrintColor::Failed);
                return;
            }
        }
    }

    let exercise_id_result =
        command_util::get_exercise_id_from_config(&course_config.unwrap(), &exercise_name);
    let return_url: Url;
    match exercise_id_result {
        Ok(exercise_id) => {
            return_url = Url::parse(&command_util::generate_return_url(exercise_id)).unwrap();
        }
        Err(err) => {
            io.println(&err, PrintColor::Failed);
            return;
        }
    }

    io.println("Write a paste message, enter sends it:", PrintColor::Normal);
    let paste_msg = io.read_line();
    io.println("", PrintColor::Normal);

    // start manager for 1 events TmcClient::paste
    let mut manager = ProgressBarManager::new(
        progress_reporting::get_default_style(),
        1,
        client.is_test_mode(),
    );
    manager.start::<ClientUpdateData>();

    // Send submission, handle errors and print link to paste
    let new_submission = client.paste(
        return_url,
        exercise_dir.as_path(),
        Some(paste_msg),
        Some(Language::Eng),
    );

    match new_submission {
        Ok(_submission) => {
            manager.join();
        }
        Err(err) => {
            manager.force_join();
            io.println(&format!("Error: {} ", err), PrintColor::Failed);
        }
    }
}
