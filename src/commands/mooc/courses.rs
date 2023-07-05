use super::super::util::Client;
use crate::{Io, PrintColor};
use tmc_langs::mooc::CourseInstance;

pub fn run(io: &mut dyn Io, client: &mut dyn Client) -> anyhow::Result<()> {
    let mut courses = client.mooc_courses()?;
    courses.sort_by_cached_key(|c| c.course_name.clone());
    print_courses(io, &courses)?;
    Ok(())
}

/// Prints information about given exercises
fn print_courses(io: &mut dyn Io, course: &[CourseInstance]) -> anyhow::Result<()> {
    io.println(
        "\nCurrently enrolled courses.mooc.fi courses",
        PrintColor::Normal,
    )?;

    for course in course {
        let course_identifier = super::course_identifier(course);
        io.println(&format!("  {}", course_identifier), PrintColor::Normal)?;
    }

    Ok(())
}
