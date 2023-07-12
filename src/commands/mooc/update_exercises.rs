use crate::{
    commands::util::Client,
    config::{LocalMoocExercise, TmcCliConfig},
    Io, PrintColor,
};
use std::{collections::HashMap, ops::DerefMut};
use tmc_langs::mooc::{PublicSpec, TmcExerciseTask};
use uuid::Uuid;

pub fn run(io: &mut Io, client: &mut Client, course: Option<&str>) -> anyhow::Result<()> {
    let Some(course) = super::get_course_by_slug_or_selection(io, client, course)? else {
        return Ok(());
    };

    let mooc_exercises = client
        .mooc_course_exercises(course.id)?
        .into_iter()
        .flat_map(|e| {
            e.tasks
                .into_iter()
                .map(move |t| ((e.exercise_id, e.slide_id, t.task_id), t))
        })
        .collect::<HashMap<_, _>>();
    update_exercises(io, client, mooc_exercises)?;

    Ok(())
}

fn update_exercises(
    io: &mut Io,
    client: &mut Client,
    mooc_exercises: HashMap<(Uuid, Uuid, Uuid), TmcExerciseTask>,
) -> anyhow::Result<()> {
    let config = TmcCliConfig::load()?;

    for local_exercise in config.get_mooc_exercises().deref_mut() {
        if let Err(err) = update_exercise(io, client, local_exercise, &mooc_exercises) {
            // ignore (highly unlikely) logging errors, we'll want to save the config either way
            let _ = io.println(
                &format!(
                    "Failed to update exercise '{}': {err}",
                    local_exercise.exercise_name
                ),
                PrintColor::Failed,
            );
        }
    }
    config.save()?;

    Ok(())
}

// handles a single exercise update
// this way one update erroring out doesn't end the entire update process
fn update_exercise(
    io: &mut Io,
    client: &mut Client,
    local_exercise: &mut LocalMoocExercise,
    mooc_exercises: &HashMap<(Uuid, Uuid, Uuid), TmcExerciseTask>,
) -> anyhow::Result<()> {
    match mooc_exercises.get(&(
        local_exercise.exercise_id,
        local_exercise.slide_id,
        local_exercise.task_id,
    )) {
        Some(TmcExerciseTask {
            public_spec:
                Some(PublicSpec::Editor {
                    archive_name: _,
                    ref archive_download_url,
                    ref checksum,
                }),
            ..
        }) => {
            if checksum != &local_exercise.checksum || !local_exercise.location.exists() {
                io.println(
                    &format!(
                        "Downloading an update to exercise '{}'",
                        local_exercise.exercise_name
                    ),
                    PrintColor::Normal,
                )?;
                super::download_and_extract_exercise(
                    client,
                    archive_download_url.clone(),
                    &local_exercise.location,
                )?;
                local_exercise.checksum = checksum.clone();
                io.println(
                    &format!("Updated exercise '{}'!", local_exercise.exercise_name,),
                    PrintColor::Success,
                )?;
            } else {
                io.println(
                    &format!("Exercise '{}' is up to date", local_exercise.exercise_name),
                    PrintColor::Normal,
                )?;
            }
        }
        Some(_non_editor_task) => {
            // this case should be pretty unusual...
            io.println(
            &format!(
                "Exercise '{}' (located at '{}') has been changed into a browser exercise, intended to be completed in the course material",
                local_exercise.exercise_name, local_exercise.location.display()
            ),
            PrintColor::Normal,
        )?;
        }
        None => {
            io.println(
            &format!(
                "Failed to find exercise '{}' (located at '{}') on the server, it may have been deleted",
                local_exercise.exercise_name, local_exercise.location.display()
            ),
            PrintColor::Failed,
        )?;
        }
    }
    Ok(())
}
