mod courses;
mod download;
mod exercises;
mod generate_completions;
mod login;
mod logout;
mod mooc;
mod organization;
mod paste;
mod submit;
mod test;
mod update;
pub mod util;

use crate::{
    cli::{Cli, Command},
    io::Io,
};
use anyhow::Context;
use util::{Client, ClientProduction};

pub fn handle(cli: Cli, io: &mut dyn Io) -> anyhow::Result<()> {
    let mut client = ClientProduction::new(cli.testmode)?;

    // Authorize the client and raise error if not logged in when required
    match cli.subcommand {
        Command::Login { .. } => {
            if client.load_login().is_ok() {
                anyhow::bail!("Already logged in. Please logout first with 'tmc logout'",);
            }
        }
        Command::Test { .. } => {}
        _ => {
            client
                .load_login()
                .context("No login found. Login to use this command with 'tmc login'")?;
        }
    };

    // Check that organization is set
    if cli.subcommand.requires_organization_set() {
        util::get_organization().context("No organization found. Run 'tmc organization' first.")?;
    }

    match cli.subcommand {
        // tmc commands
        Command::Login { non_interactive } => {
            let interactive_mode = !non_interactive;
            login::login(io, &mut client, interactive_mode)?;
        }
        Command::Download { course, currentdir } => {
            download::download_or_update(io, &mut client, course.as_deref(), currentdir)?
        }
        Command::Update { currentdir } => {
            update::update(io, &mut client, currentdir)?;
        }
        Command::Organization { non_interactive } => {
            let interactive_mode = !non_interactive;
            organization::organization(io, &mut client, interactive_mode)?
        }
        Command::Courses => courses::list_courses(io, &mut client)?,
        Command::Submit { exercise } => {
            submit::submit(io, &mut client, exercise.as_deref())?;
        }
        Command::Exercises { course } => {
            exercises::list_exercises(io, &mut client, course.as_deref())?
        }
        Command::Test { exercise } => {
            test::test(io, exercise.as_deref())?;
        }
        Command::Paste { exercise } => {
            paste::paste(io, &mut client, exercise.as_deref())?;
        }
        Command::Logout => logout::logout(io, &mut client)?,

        // mooc commands
        Command::MoocCourses => mooc::courses::run(io, &mut client)?,
        Command::MoocCourseExercises { course } => {
            mooc::course_exercises::run(io, &mut client, course.as_deref())?
        }
        Command::MoocDownloadExercises { course, currentdir } => {
            mooc::download_exercises::run(io, &mut client, course.as_deref(), currentdir)?
        }
        Command::MoocSubmitExercise { path } => {
            mooc::submit_exercise::run(io, &mut client, path.as_deref())?
        }

        // hidden commands
        Command::Fetchupdate => {
            #[cfg(target_os = "windows")]
            crate::updater::process_update()?;
        }
        Command::Cleartemp => {
            #[cfg(target_os = "windows")]
            crate::updater::cleartemp()?;
        }
        Command::Elevateddownload => {
            download::elevated_download(io, &mut client)?;
        }
        Command::Elevatedupdate => {
            update::elevated_update(io, &mut client)?;
        }
        Command::GenerateCompletions { shell } => {
            generate_completions::generate(shell);
        }
    }
    Ok(())
}
