use super::super::util::Client;
use crate::{
    config::{LocalExercise, TmcCliConfig},
    Io, PrintColor,
};
use std::{io::Cursor, path::Path};
use tmc_langs::{
    mooc::{CourseInstance, PublicSpec, TmcExerciseSlide},
    Compression,
};

pub fn run(
    io: &mut dyn Io,
    client: &mut dyn Client,
    slug: Option<&str>,
    current_dir: bool,
) -> anyhow::Result<()> {
    let Some(course) = super::get_course_by_slug_or_selection(io, client, slug)? else {
        return Ok(());
    };

    let dir = if current_dir {
        std::env::current_dir()?
    } else {
        crate::commands::util::get_projects_dir()?
    };

    let exercises = client.mooc_course_exercises(course.id)?;
    download_exercises(io, client, &course, &exercises, &dir)?;

    Ok(())
}

/// Prints information about given exercises
fn download_exercises(
    io: &mut dyn Io,
    client: &mut dyn Client,
    course: &CourseInstance,
    exercises: &[TmcExerciseSlide],
    dir: &Path,
) -> anyhow::Result<()> {
    io.println(
        &format!(
            "Downloading exercises for {} into {}...",
            course.course_name,
            dir.display()
        ),
        PrintColor::Normal,
    )?;
    let mut config = TmcCliConfig::load()?;

    for exercise in exercises {
        for task in &exercise.tasks {
            match &task.public_spec {
                Some(PublicSpec::Editor {
                    archive_name: _,
                    archive_download_url,
                }) => {
                    let bytes = client.mooc_download_exercise(archive_download_url.to_owned())?;
                    let dir_name = format!(
                        "{}-{}-{}",
                        exercise.exercise_order_number, exercise.exercise_name, task.order_number
                    );
                    let target_location = dir.join(dir_name);
                    tmc_langs::extract_project(
                        Cursor::new(bytes.as_ref()),
                        &target_location,
                        Compression::TarZstd,
                        false,
                        false,
                    )?;

                    io.println(
                        &format!(
                            "Downloaded '{}' to '{}'",
                            exercise.exercise_name,
                            target_location.display()
                        ),
                        PrintColor::Success,
                    )?;
                    config.add_exercise(LocalExercise {
                        exercise_id: exercise.exercise_id,
                        slide_id: exercise.slide_id,
                        task_id: task.task_id,
                        location: target_location,
                    });
                }
                Some(PublicSpec::Browser { .. }) => {
                    io.println(&format!("Skipping exercise {} as it is meant to be completed in the course material", exercise.exercise_name), PrintColor::Normal)?;
                }
                None => {
                    io.println(
                        &format!(
                            "Skipping exercise {} as it is not active yet",
                            exercise.exercise_name
                        ),
                        PrintColor::Normal,
                    )?;
                }
            }
        }
    }

    config.save()?;

    Ok(())
}
