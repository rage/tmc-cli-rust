use super::util;
use crate::{
    client::Client,
    config::TmcCliConfig,
    io::{Io, PrintColor},
    progress_reporting,
    progress_reporting::ProgressBarManager,
};
use anyhow::Context;
use tmc_langs::{tmc::ClientUpdateData, Language};

/// Sends the course exercise submission with paste message to the server.
/// Path to the exercise can be given as a parameter or
/// the user can run the command in the exercise folder.
///
/// # Errors
/// Returns an error if no exercise found on given path or current folder.
/// Returns an error if user is not logged in.
pub fn paste(
    io: &mut Io,
    client: &mut Client,
    path: Option<&str>,
    config: &TmcCliConfig,
) -> anyhow::Result<()> {
    // todo: use context
    let exercise_path =
        util::exercise_pathfinder(path, config).context("Error finding exercise")?;

    let (project_config, course_slug, exercise_slug) = util::parse_exercise_dir(exercise_path)?;

    io.println("Write a paste message, enter sends it:", PrintColor::Normal)?;
    let paste_msg = io.read_line()?;
    io.println("", PrintColor::Normal)?;

    // start manager for 1 events TmcClient::paste
    let mut manager = ProgressBarManager::new(
        progress_reporting::get_default_style(),
        1,
        client.is_test_mode(),
    );
    manager.start::<ClientUpdateData>();

    // Send submission, handle errors and print link to paste
    let new_submission = client.paste(
        &project_config,
        &course_slug,
        &exercise_slug,
        Some(paste_msg),
        Some(Language::Eng),
    );

    match new_submission {
        Ok(_submission) => {
            manager.join();
        }
        Err(err) => {
            manager.force_join();
            io.println(&format!("Error: {err} "), PrintColor::Failed)?;
        }
    }
    Ok(())
}
