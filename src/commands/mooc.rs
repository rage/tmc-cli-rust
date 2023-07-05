use super::util::Client;
use crate::{Io, PrintColor};
use anyhow::Context;
use std::collections::HashMap;
use tmc_langs::mooc::CourseInstance;

pub mod course_exercises;
pub mod courses;
pub mod download_exercises;
pub mod submit_exercise;

// Ok(Some(course)) => found course by slug or selection
// Ok(None) => user declined to select a course
// Err => something went wrong
fn get_course_by_slug_or_selection(
    io: &mut dyn Io,
    client: &mut dyn Client,
    slug: Option<&str>,
) -> anyhow::Result<Option<CourseInstance>> {
    let courses = client.mooc_courses()?;
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
