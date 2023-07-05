use super::super::util::Client;
use crate::Io;
use std::path::{Path, PathBuf};
use tmc_langs::mooc::{ExerciseSlideSubmission, ExerciseTaskSubmission};
use uuid::Uuid;

pub fn run(_io: &mut dyn Io, client: &mut dyn Client, path: Option<&Path>) -> anyhow::Result<()> {
    let selected_path;
    let _path = match path {
        Some(path) => path,
        None => {
            selected_path = PathBuf::from("asd");
            &selected_path
        }
    };
    let exercise_id = Uuid::new_v4();
    let exercise_slide_submission = ExerciseSlideSubmission {
        exercise_slide_id: Uuid::new_v4(),
        exercise_task_submissions: vec![ExerciseTaskSubmission {
            exercise_task_id: Uuid::new_v4(),
            data_json: serde_json::Value::Null,
        }],
    };
    client.mooc_submit_exercise(exercise_id, &exercise_slide_submission)?;
    Ok(())
}
