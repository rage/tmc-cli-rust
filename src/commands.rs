mod courses;
mod download;
mod exercises;
mod generate_completions;
mod login;
mod logout;
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
use std::env;
use util::Client;

pub fn handle(cli: Cli, io: &mut Io) -> anyhow::Result<()> {
    let tmc_root_url = match env::var("TMC_LANGS_TMC_ROOT_URL") {
        Ok(url) => url
            .parse()
            .with_context(|| format!("Failed to parse TMC_LANGS_TMC_ROOT_URL ({url}) as a URL"))?,
        Err(_) => "https://tmc.mooc.fi".parse().expect("known to work"),
    };
    let mut client = Client::new(tmc_root_url, cli.testmode)?;

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
