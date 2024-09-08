use crate::{
    client::Client,
    config::TmcCliConfig,
    interactive::{self, interactive_list},
    io::{Io, PrintColor},
    PLUGIN,
};
use anyhow::Context;
use std::{env, path::PathBuf};
use tmc_langs::{tmc::response::Course, Credentials, ProjectsConfig};

pub fn get_credentials() -> Option<Credentials> {
    // Load login credentials if they exist in the file
    Credentials::load(PLUGIN).unwrap_or(None)
}

/// Returns course as: Ok(Some(Course)) or Ok(None) if not found, Err(msg) if could not get courses list
pub fn get_course_by_name(
    client: &mut Client,
    course_name: &str,
    org: &str,
) -> anyhow::Result<Option<Course>> {
    let courses = client.list_courses(org)?;
    Ok(courses.into_iter().find(|c| c.name == course_name))
}

/// Finds an exercise
/// Priority to check for valid exercise path:
/// 1. Checks optional parameter
/// 2. Checks current directory
/// 3. Checks central ProjectsConfig with interactive menu
///
/// # Errors
/// Returns an error if the last chance, interactive menu, fails.
pub fn exercise_pathfinder(path: Option<&str>, config: &TmcCliConfig) -> anyhow::Result<PathBuf> {
    // check if parameter was given
    if let Some(ex_path) = path {
        let buf = PathBuf::from(ex_path);
        if is_exercise_dir(buf.clone())? {
            return Ok(buf);
        } else {
            anyhow::bail!("Invalid exercise path given");
        }
    }

    let current_path = env::current_dir().ok();

    // check if current path is an exercise_dir,
    // in any other case use interactive menu
    match current_path {
        Some(ex_path) => match is_exercise_dir(ex_path.clone()) {
            Ok(is_ex_path) => {
                if is_ex_path {
                    Ok(ex_path)
                } else {
                    choose_exercise(config)
                }
            }
            Err(_err) => choose_exercise(config),
        },
        None => choose_exercise(config),
    }
}

pub fn choose_course(io: &mut Io, client: &mut Client, org: &str) -> anyhow::Result<String> {
    io.println("Fetching courses...", PrintColor::Normal)?;
    let courses = client
        .list_courses(org)
        .context("Could not list courses.")?;

    let mut courses = courses
        .iter()
        .map(|course| client.get_course_details(course.id))
        .collect::<Result<Vec<_>, _>>()?;
    courses.sort_by(|a, b| {
        a.course
            .title
            .to_lowercase()
            .cmp(&b.course.title.to_lowercase())
    });
    let course = get_course_name(
        &courses
            .iter()
            .map(|course| course.course.title.as_str())
            .collect::<Vec<_>>(),
    )?;
    let selection = courses
        .into_iter()
        .find(|c| c.course.title == course)
        .context("No course with the selected name was found")?
        .course
        .name;
    Ok(selection)
}

pub fn get_course_name(courses: &[&str]) -> anyhow::Result<String> {
    let course = interactive::interactive_list("Select your course:", courses)?
        .ok_or_else(|| anyhow::anyhow!("Didn't select any course"))?;

    if course.is_empty() {
        anyhow::bail!("Could not find a course by the given title");
    } else {
        Ok(course)
    }
}

/// Choose course and then exercise interactively, return exercise path
/// or Err(String) if either menu is interrupted or no items found
pub fn choose_exercise(config: &TmcCliConfig) -> anyhow::Result<PathBuf> {
    let mut courses = Vec::new();

    let projects_config = match ProjectsConfig::load(config.get_projects_dir()) {
        Ok(projects_config) => projects_config,
        Err(err) => anyhow::bail!("Could not load info about projects due to '{err}'"),
    };

    for course in projects_config.courses.keys() {
        courses.push(course.as_str());
    }

    if courses.is_empty() {
        anyhow::bail!(
            "No courses found from current or project directory. Project directory set to {}",
            config
                .get_projects_dir()
                .to_str()
                .context("invalid projects dir")?
        );
    }

    courses.sort();
    let chosen_course = interactive_list("First select course: ", &courses)?
        .ok_or_else(|| anyhow::anyhow!("Didn't select any course"))?;

    let course_config = projects_config
        .courses
        .get(&chosen_course)
        .context("Failed to find selected course")?;

    let mut exercise_list = Vec::new();

    for exercise in &course_config.exercises {
        exercise_list.push(exercise.0.as_str());
    }

    if exercise_list.is_empty() {
        anyhow::bail!(
            "No exercises found from chosen course folder. Project directory set to {}",
            config
                .get_projects_dir()
                .to_str()
                .context("invalid projects dir")?
        );
    }

    let chosen_exercise = interactive_list("Select exercise: ", &exercise_list)?
        .ok_or_else(|| anyhow::anyhow!("Didn't select any exercise"))?;

    let mut path = config.get_projects_dir().to_path_buf();
    path.push(chosen_course);
    path.push(chosen_exercise);

    Ok(path)
}

/// Parses an exercise path into (projects_dir, course_name, exercise_name)
///
/// # Errors
/// Returns an error if there was problems reading file_names
pub fn parse_exercise_dir(mut exercise_dir: PathBuf) -> anyhow::Result<(PathBuf, String, String)> {
    let exercise_slug = exercise_dir
        .file_name()
        .context("could not get exercise name")?
        .to_str()
        .context("could not get exercise name")?
        .to_string();

    exercise_dir.pop();
    let course_slug = exercise_dir
        .file_name()
        .context("could not get exercise name")?
        .to_str()
        .context("could not get exercise name")?
        .to_string();

    exercise_dir.pop();

    Ok((exercise_dir, course_slug, exercise_slug))
}

/// Checks if provided directory contains an exercise
///
/// # Errors
/// Returns an error if it failed to load ProjectsConfig
/// Or failed to read paths
pub fn is_exercise_dir(dir: PathBuf) -> anyhow::Result<bool> {
    let (projects_dir, course_slug, _exercise_slug) = parse_exercise_dir(dir)?;
    let config = ProjectsConfig::load(projects_dir.as_path()).with_context(|| {
        format!(
            "Failed to load projects config from directory '{}'",
            projects_dir.display(),
        )
    })?;

    Ok(config.courses.contains_key(&course_slug))
}

/// Returns a manual progress bar of size 'length' based on percentage of 'completed' / 'total'
pub fn get_progress_string(completed: usize, total: usize, length: usize) -> String {
    let completed_proportion = if total == 0 {
        1_f32
    } else {
        completed as f32 / total as f32
    };
    let completed_percentage_readable = (completed_proportion * 100_f32).floor() as usize;
    let progress_done = (completed_proportion * length as f32).floor() as usize;

    let mut progress_string = String::with_capacity(length);
    for _ in 0..progress_done {
        progress_string.push('█');
    }
    for _ in progress_done..length {
        progress_string.push('░');
    }

    let spaces = if completed_percentage_readable < 10 {
        "   "
    } else if completed_percentage_readable < 100 {
        "  "
    } else {
        " "
    };
    format!("{spaces}{completed_percentage_readable}%[{progress_string}]")
}
