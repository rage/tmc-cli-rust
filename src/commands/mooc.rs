//! Contains courses.mooc.fi-related commands.

pub mod download_exercises;
pub mod submit_exercise;
pub mod update_exercises;

use crate::{
    client::Client,
    config::{LocalMoocExercise, TmcCliConfig},
};
use anyhow::Context;
use std::{
    collections::HashMap,
    io::Cursor,
    ops::Deref,
    path::{Path, PathBuf},
};
use tmc_langs::{
    mooc::{CourseInstance, TmcExerciseSlide},
    Compression,
};

// Ok(Some(course)) => found course by slug or selection
// Ok(None) => user declined to select a course
// Err => something went wrong
fn get_course_by_slug_or_selection(
    client: &mut Client,
    slug: Option<&str>,
) -> anyhow::Result<CourseInstance> {
    let courses = client.enrolled_mooc_courses()?;
    if courses.is_empty() {
        anyhow::bail!("No enrolled courses found");
    }

    let course = if let Some(slug) = slug {
        courses
            .into_iter()
            .find(|c| c.course_slug == slug)
            .with_context(|| format!("Failed to find course with the given slug '{slug}'"))?
    } else {
        let mut course_name_to_course = courses
            .into_iter()
            .map(|c| {
                let key = self::course_identifier(&c);
                (key, c)
            })
            .collect::<HashMap<_, _>>();
        let keys = course_name_to_course
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>();

        let help_message = "If the course you're looking for is not listed here, please log in to https://courses.mooc.fi using the same account you use here. Then, access the course material and choose a course instance from the dialog that pops up.";
        match crate::interactive::interactive_list("Select course:", &keys, Some(help_message))? {
            Some(selection) => course_name_to_course
                .remove(&selection)
                .expect("Invalid selection"),
            None => {
                anyhow::bail!("Did not select a course");
            }
        }
    };

    Ok(course)
}

// Ok(Some(exercise)) => selected an exercise
// Ok(None) => user declined to select an exercise
// Err => something went wrong
fn select_exercise(
    course: &CourseInstance,
    config: &TmcCliConfig,
) -> anyhow::Result<LocalMoocExercise> {
    let exercises = config.get_local_mooc_exercises();
    let course_exercises = exercises
        .deref()
        .iter()
        .filter(|le| le.course_instance_id == course.id)
        .collect::<Vec<_>>();

    let mut exercise_name_to_exercise = course_exercises
        .into_iter()
        .map(|e| (&e.exercise_name, e))
        .collect::<HashMap<_, _>>();
    let keys = exercise_name_to_exercise
        .keys()
        .map(|s| s.as_str())
        .collect::<Vec<_>>();

    let course = match crate::interactive::interactive_list("Select exercise:", &keys, None)? {
        Some(selection) => exercise_name_to_exercise
            .remove(&selection)
            .expect("Invalid selection"),
        None => {
            anyhow::bail!("Did not select an exercise");
        }
    };

    Ok(course.clone())
}

fn course_identifier(course: &CourseInstance) -> String {
    let instance_name = course
        .instance_name
        .as_deref()
        .unwrap_or("default instance");
    format!(
        "{} ({} | {})",
        course.course_name, course.course_slug, instance_name
    )
}

fn exercise_path(
    root: &Path,
    course_instance: &CourseInstance,
    exercise: &TmcExerciseSlide,
) -> PathBuf {
    let dir_name = format!(
        "{}-{}",
        exercise.exercise_order_number, exercise.exercise_name
    );
    root.join("courses")
        .join(&course_instance.course_slug)
        .join(course_instance.id.to_string())
        .join(dir_name)
}

fn download_and_extract_exercise(
    client: &mut Client,
    url: String,
    target_location: &Path,
) -> anyhow::Result<()> {
    let bytes = client.mooc_download_exercise(url)?;
    tmc_langs::extract_project(
        Cursor::new(bytes.as_ref()),
        target_location,
        Compression::TarZstd,
        false,
        false,
    )?;
    Ok(())
}
