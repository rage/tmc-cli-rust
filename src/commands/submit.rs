use super::{util, util::Client};
use crate::{
    io::{Io, PrintColor},
    progress_reporting,
    progress_reporting::ProgressBarManager,
};
use anyhow::{Context, Result};
use reqwest::Url;
use tmc_langs::{
    tmc::{response::SubmissionFinished, ClientUpdateData},
    Language,
};

/// Sends the course exercise submission to the server.
/// Path to the exercise can be given as a parameter or
/// the user can run the command in the exercise folder.
///
/// # Errors
/// Returns an error if no exercise was found on given path or current folder.
/// Returns an error if user is not logged in.
pub fn submit(io: &mut Io, client: &mut Client, path: Option<&str>) -> anyhow::Result<()> {
    let locale = into_locale("fin").expect("The locale should always be valid.");

    // todo: use context
    let exercise_path = util::exercise_pathfinder(path).context("Error finding exercise")?;

    let (project_config, course_slug, exercise_slug) = util::parse_exercise_dir(exercise_path)?;

    io.println("\n", PrintColor::Normal)?;

    // start manager for 2 events TmcClient::submit, TmcClient::wait_for_submission
    let mut manager = ProgressBarManager::new(
        progress_reporting::get_default_style(),
        2,
        client.is_test_mode(),
    );
    manager.start::<ClientUpdateData>();

    // Send submission
    let new_submission =
        match client.submit(&project_config, &course_slug, &exercise_slug, Some(locale)) {
            Ok(sub) => sub,
            err @ Err(_) => {
                manager.force_join();

                return err.map(|_| ()).context("Error during submission");
            }
        };

    manager.println(format!(
        "You can view your submission at: {}",
        new_submission.show_submission_url
    ));

    let submission_url = Url::parse(&new_submission.submission_url)?;
    match client.wait_for_submission(submission_url) {
        Ok(submission_finished) => {
            manager.join();

            print_wait_for_submission_results(io, submission_finished)?;
        }
        Err(err) => {
            manager.force_join();

            io.println(&format!("Failed while waiting for server to process submission.\n You can still check your submission manually here: {}.", &new_submission.show_submission_url), PrintColor::Normal)?;
            io.println(&format!("Error message: {err:#?}"), PrintColor::Normal)?;
        }
    }
    Ok(())
}

fn print_wait_for_submission_results(
    io: &mut Io,
    submission_finished: SubmissionFinished,
) -> anyhow::Result<()> {
    let mut all_passed = false;
    if let Some(all_tests_passed) = submission_finished.all_tests_passed {
        all_passed = all_tests_passed;
        if all_tests_passed {
            io.println("All tests passed on server!", PrintColor::Success)?;
        }
    }
    if !submission_finished.points.is_empty() {
        io.print("Points permanently awarded: [", PrintColor::Normal)?;
        for i in 0..submission_finished.points.len() {
            io.print(
                &submission_finished.points[i].to_string(),
                PrintColor::Normal,
            )?;
            if i < submission_finished.points.len() - 1 {
                io.print(", ", PrintColor::Normal)?;
            }
        }
        io.println("]", PrintColor::Normal)?;
    } else {
        io.println("No new points awarded.", PrintColor::Normal)?;
    }

    if all_passed {
        if let Some(solution_url) = submission_finished.solution_url {
            io.println(
                &format!("Model solution: {solution_url}"),
                PrintColor::Normal,
            )?;
        }
    } else {
        if let Some(error) = submission_finished.error {
            io.println(&format!("Error: {error}"), PrintColor::Failed)?;
        }

        if let Some(test_cases) = submission_finished.test_cases {
            let mut completed = 0;
            let mut total = 0;
            for case in test_cases {
                if case.successful {
                    completed += 1;
                } else {
                    io.println(&format!("Failed: {}", case.name), PrintColor::Failed)?;
                    if let Some(message) = case.message {
                        let formatted = message.replace('\n', "\n        ");
                        io.println(&format!("        {formatted}"), PrintColor::Normal)?;
                    }
                    io.println("", PrintColor::Normal)?;
                }
                total += 1;
            }
            io.println(
                &format!("\nTest results: {completed}/{total} tests passed"),
                PrintColor::Normal,
            )?;

            io.println(
                &util::get_progress_string(completed, total, 64),
                PrintColor::Normal,
            )?;
        }
    }
    Ok(())
}

fn into_locale(arg: &str) -> Result<Language> {
    Language::from_locale(arg)
        .or_else(|| Language::from_639_1(arg))
        .or_else(|| Language::from_639_3(arg))
        .with_context(|| format!("Invalid locale: {arg}"))
}
