use super::super::util::Client;
use crate::{
    config::{LocalMoocExercise, TmcCliConfig},
    Io, PrintColor,
};
use std::{collections::HashMap, path::Path};
use tmc_langs::mooc::{CourseInstance, PublicSpec, TmcExerciseSlide};
use uuid::Uuid;

pub fn run(
    io: &mut Io,
    client: &mut Client,
    slug: Option<&str>,
    current_dir: bool,
) -> anyhow::Result<()> {
    let Some(course) = super::get_course_by_slug_or_selection(io, client, slug)? else {
        return Ok(());
    };

    let exercises = client.mooc_course_exercises(course.id)?;
    download_exercises(io, client, &course, exercises, current_dir)?;

    Ok(())
}

/// Prints information about given exercises
fn download_exercises(
    io: &mut Io,
    client: &mut Client,
    course: &CourseInstance,
    exercises: Vec<TmcExerciseSlide>,
    current_dir: bool,
) -> anyhow::Result<()> {
    let mut config = TmcCliConfig::load()?;
    let projects_dir = if current_dir {
        std::env::current_dir()?
    } else {
        config.get_projects_dir().to_path_buf()
    };
    let local_exercise_map = config
        .get_mooc_exercises()
        .iter()
        .cloned()
        .map(|e| ((e.exercise_id, e.slide_id, e.task_id), e))
        .collect::<HashMap<_, _>>();

    io.println(
        &format!(
            "Downloading exercises for '{}' into '{}'...",
            course.course_name,
            projects_dir.display()
        ),
        PrintColor::Normal,
    )?;

    for exercise in exercises {
        let exercise_name = exercise.exercise_name.clone();
        if let Err(err) = download_exercise(
            io,
            client,
            &mut config,
            course,
            &projects_dir,
            exercise,
            &local_exercise_map,
        ) {
            // ignore (highly unlikely) logging errors, we'll want to save the config either way
            let _ = io.println(
                &format!("Failed to update exercise '{exercise_name}': {err}",),
                PrintColor::Failed,
            );
        }
    }
    config.save()?;

    Ok(())
}

// handles a single exercise download
// this way one download erroring out doesn't end the entire download process
fn download_exercise(
    io: &mut Io,
    client: &mut Client,
    config: &mut TmcCliConfig,
    course: &CourseInstance,
    projects_dir: &Path,
    exercise: TmcExerciseSlide,
    local_exercise_map: &HashMap<(Uuid, Uuid, Uuid), LocalMoocExercise>,
) -> anyhow::Result<()> {
    if exercise.tasks.len() > 1 {
        io.println("Exercise contains more than one task, but only one task per exercise is currently supported by the CLI. Only the first task will be processed.", PrintColor::Failed)?;
    }
    let download_location = super::exercise_path(projects_dir, &course.course_slug, &exercise);
    // only take first task
    if let Some(task) = exercise.tasks.into_iter().next() {
        let existing_exercise =
            local_exercise_map.get(&(exercise.exercise_id, exercise.slide_id, task.task_id));

        match task.public_spec {
            Some(PublicSpec::Editor {
                archive_name: _,
                archive_download_url,
                checksum,
            }) => {
                if let Some(existing_exercise) = existing_exercise {
                    if existing_exercise.checksum == checksum && existing_exercise.location.exists()
                    {
                        io.println(
                            &format!(
                                "Skipping exercise '{}' because it is already downloaded",
                                exercise.exercise_name,
                            ),
                            PrintColor::Normal,
                        )?;
                        return Ok(());
                    }
                }

                super::download_and_extract_exercise(
                    client,
                    archive_download_url.clone(),
                    &download_location,
                )?;

                if existing_exercise.is_some() {
                    io.println(
                        &format!("Updated '{}'", exercise.exercise_name,),
                        PrintColor::Success,
                    )?;
                } else {
                    io.println(
                        &format!("Downloaded '{}'", exercise.exercise_name,),
                        PrintColor::Success,
                    )?;
                }
                config.add_mooc_exercise(LocalMoocExercise {
                    course_instance_id: course.id,
                    exercise_name: exercise.exercise_name,
                    exercise_id: exercise.exercise_id,
                    slide_id: exercise.slide_id,
                    task_id: task.task_id,
                    location: download_location,
                    download_url: archive_download_url,
                    checksum,
                });
            }
            Some(PublicSpec::Browser { .. }) => {
                io.println(&format!("Skipping exercise '{}' as it is meant to be completed in the course material", exercise.exercise_name), PrintColor::Normal)?;
            }
            None => {
                io.println(
                    &format!(
                        "Skipping exercise '{}' as it is not active yet",
                        exercise.exercise_name
                    ),
                    PrintColor::Normal,
                )?;
            }
        }
    }
    Ok(())
}
