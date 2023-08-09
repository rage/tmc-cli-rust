use crate::{
    client::Client,
    config::{LocalMoocExercise, TmcCliConfig},
    Io, PrintColor,
};
use std::{ops::Deref, path::Path, time::Duration};
use tmc_langs::{
    mooc::{ExerciseTaskSubmissionStatus, GradingProgress},
    Compression,
};

pub fn run(
    io: &mut Io,
    client: &mut Client,
    path: Option<&Path>,
    config: &TmcCliConfig,
) -> anyhow::Result<()> {
    let Some(exercise) = select_exercise(io, client, path, config)? else {
        return Ok(());
    };

    io.println("Packaging the submission...", PrintColor::Normal)?;
    let temp_file = tmc_langs::file_util::named_temp_file()?;
    tmc_langs::compress_project_to(
        &exercise.location,
        temp_file.path(),
        Compression::TarZstd,
        false,
        false,
    )?;

    io.println(
        "Sending the submission to the server...",
        PrintColor::Normal,
    )?;
    let submission = client.mooc_submit_exercise(
        exercise.exercise_id,
        exercise.slide_id,
        exercise.task_id,
        temp_file.path(),
    )?;

    io.println(
        "Waiting for the server to grade the submission...",
        PrintColor::Normal,
    )?;
    let mut seconds_waited = 0;
    loop {
        match client.mooc_get_submission_grading(submission.submission_id)? {
            ExerciseTaskSubmissionStatus::NoGradingYet => {} // continue waiting...
            ExerciseTaskSubmissionStatus::Grading {
                grading_progress,
                score_given,
                feedback_text,
                ..
            } => match grading_progress {
                GradingProgress::NotReady | GradingProgress::Pending => {} // continue waiting...
                GradingProgress::Failed => {
                    io.println("Failed to grade submission", PrintColor::Failed)?;
                    if let Some(feedback) = feedback_text {
                        io.println(&format!("Feedback: {feedback}"), PrintColor::Normal)?;
                    }
                    break;
                }
                GradingProgress::PendingManual => {
                    io.println(
                        "Finishing the grading process requires manual intervention",
                        PrintColor::Normal,
                    )?;
                    if let Some(feedback) = feedback_text {
                        io.println(&format!("Feedback: {feedback}"), PrintColor::Normal)?;
                    }
                    break;
                }
                GradingProgress::FullyGraded => {
                    io.println("Submission grading has finished", PrintColor::Success)?;
                    if let Some(score) = score_given {
                        io.println(&format!("Score: {score}"), PrintColor::Normal)?;
                    } else {
                        io.println("No score given by server", PrintColor::Failed)?;
                    }
                    if let Some(feedback) = feedback_text {
                        io.println(&format!("Feedback: {feedback}"), PrintColor::Normal)?;
                    }
                    break;
                }
            },
        }
        let waiting_period_seconds = 10;
        std::thread::sleep(Duration::from_secs(waiting_period_seconds));
        seconds_waited += waiting_period_seconds;
        if seconds_waited >= 10 {
            io.println("Still waiting...", PrintColor::Normal)?;
            seconds_waited = 0;
        }
    }
    Ok(())
}

fn select_exercise(
    io: &mut Io,
    client: &mut Client,
    path: Option<&Path>,

    config: &TmcCliConfig,
) -> anyhow::Result<Option<LocalMoocExercise>> {
    match path {
        Some(path) => {
            // try to find exercise details for path
            let canon_path = path.canonicalize()?;
            let exercises = config.get_mooc_exercises();
            let exercise = exercises
                .deref()
                .iter()
                .find(|e| e.location == canon_path)
                .ok_or_else(|| {
                    anyhow::anyhow!("Failed to find exercise for the path '{}'", path.display())
                })?;
            Ok(Some(exercise.clone()))
        }
        None => {
            let Some(course) = super::get_course_by_slug_or_selection(io, client, None)? else {
                return Ok(None);
            };
            let Some(exercise) = super::select_exercise(io, &course, config)? else {
                return Ok(None);
            };
            Ok(Some(exercise))
        }
    }
}
