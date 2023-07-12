use super::super::util::Client;
use crate::{Io, PrintColor};
use tmc_langs::mooc::{CourseInstance, TmcExerciseSlide};

pub fn run(io: &mut Io, client: &mut Client, slug: Option<&str>) -> anyhow::Result<()> {
    let Some(course) = super::get_course_by_slug_or_selection(io, client, slug)? else {
        return Ok(());
    };

    let mut exercises = client.mooc_course_exercises(course.id)?;
    exercises.sort_by_key(|e| e.exercise_order_number);
    print_exercises(io, &course, &exercises)?;

    Ok(())
}

/// Prints information about given exercises
fn print_exercises(
    io: &mut Io,
    course: &CourseInstance,
    exercises: &[TmcExerciseSlide],
) -> anyhow::Result<()> {
    let course_identifier = super::course_identifier(course);
    io.println(
        &format!("\nActive exercises on {}", course_identifier),
        PrintColor::Normal,
    )?;

    for exercise in exercises {
        io.println(&format!("  {}", exercise.exercise_name), PrintColor::Normal)?;
    }

    Ok(())
}
