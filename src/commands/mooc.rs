//! Contains courses.mooc.fi-related commands.

pub mod course_exercises;
pub mod courses;
pub mod download_exercises;
pub mod submit_exercise;
pub mod update_exercises;

use super::util::Client;
use crate::{
    config::{LocalMoocExercise, TmcCliConfig},
    Io, PrintColor,
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
    io: &mut Io,
    client: &mut dyn Client,
    slug: Option<&str>,
) -> anyhow::Result<Option<CourseInstance>> {
    let courses = client.mooc_courses()?;
    if courses.is_empty() {
        io.println("No enrolled courses found", PrintColor::Normal)?;
        return Ok(None);
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

        match crate::interactive::interactive_list("Select course:", &keys)? {
            Some(selection) => course_name_to_course
                .remove(&selection)
                .expect("Invalid selection"),
            None => {
                io.print("Did not select a course", PrintColor::Normal)?;
                return Ok(None);
            }
        }
    };

    Ok(Some(course))
}

// Ok(Some(exercise)) => selected an exercise
// Ok(None) => user declined to select an exercise
// Err => something went wrong
fn select_exercise(
    io: &mut Io,
    course: &CourseInstance,
    config: &TmcCliConfig,
) -> anyhow::Result<Option<LocalMoocExercise>> {
    let exercises = config.get_mooc_exercises();
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

    let course = match crate::interactive::interactive_list("Select exercise:", &keys)? {
        Some(selection) => exercise_name_to_exercise
            .remove(&selection)
            .expect("Invalid selection"),
        None => {
            io.print("Did not select an exercise", PrintColor::Normal)?;
            return Ok(None);
        }
    };

    Ok(Some(course.clone()))
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

fn exercise_path(root: &Path, course_slug: &str, exercise: &TmcExerciseSlide) -> PathBuf {
    let dir_name = format!(
        "{}-{}",
        exercise.exercise_order_number, exercise.exercise_name
    );
    root.join("courses").join(course_slug).join(dir_name)
}

fn download_and_extract_exercise(
    client: &mut dyn Client,
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
