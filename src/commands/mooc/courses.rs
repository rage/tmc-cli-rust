use crate::{client::Client, Io, PrintColor};
use tmc_langs::mooc::CourseInstance;

pub fn run(io: &mut Io, client: &mut Client) -> anyhow::Result<()> {
    let mut courses = client.mooc_courses()?;
    courses.sort_by_cached_key(|c| c.course_name.clone());
    print_courses(io, &courses)?;
    Ok(())
}

/// Prints information about given exercises
fn print_courses(io: &mut Io, courses: &[CourseInstance]) -> anyhow::Result<()> {
    if courses.is_empty() {
        io.println("No enrolled courses found", PrintColor::Normal)?;
        return Ok(());
    }

    io.println(
        "\nCurrently enrolled courses.mooc.fi courses",
        PrintColor::Normal,
    )?;
    for course in courses {
        let course_identifier = super::course_identifier(course);
        io.println(&format!("  {}", course_identifier), PrintColor::Normal)?;
    }

    Ok(())
}
