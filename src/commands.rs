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
    client::Client,
    config::TmcCliConfig,
    io::Io,
};
use anyhow::Context;
use std::env;

pub fn handle(cli: Cli, io: &mut Io, mut config: TmcCliConfig) -> anyhow::Result<()> {
    let tmc_root_url = match env::var("TMC_LANGS_TMC_ROOT_URL") {
        Ok(url) => url
            .parse()
            .with_context(|| format!("Failed to parse TMC_LANGS_TMC_ROOT_URL ({url}) as a URL"))?,
        Err(_) => "https://tmc.mooc.fi".parse().expect("known to work"),
    };
    let mut client = Client::new(tmc_root_url, cli.testmode)?;

    let require_logged_out = |client: &mut Client| {
        let exists = client.load_login(&config).is_ok();
        if exists {
            anyhow::bail!("Already logged in. Please logout first with 'tmc logout'");
        }
        anyhow::Ok(())
    };
    let require_logged_in = |client: &mut Client| {
        let exists = client.load_login(&config).is_ok();
        if !exists {
            anyhow::bail!("No login found. Login to use this command with 'tmc login'");
        }
        anyhow::Ok(())
    };
    let require_org = || {
        config.get_organization().ok_or_else(|| anyhow::anyhow!("No organization selected. You can select an organization with the `organization` command."))
    };

    match cli.subcommand {
        // tmc commands
        Command::Login { non_interactive } => {
            require_logged_out(&mut client)?;
            let interactive_mode = !non_interactive;
            login::login(io, &mut client, interactive_mode, &mut config)?;
        }
        Command::Download { course, currentdir } => {
            require_logged_in(&mut client)?;
            let org = require_org()?;
            download::download_or_update(
                io,
                &mut client,
                course.as_deref(),
                currentdir,
                &config,
                org,
            )?;
        }
        Command::Update { currentdir } => {
            require_logged_in(&mut client)?;
            update::update(io, &mut client, currentdir, &config)?;
        }
        Command::Organization { non_interactive } => {
            require_logged_in(&mut client)?;
            let interactive_mode = !non_interactive;
            organization::organization(io, &mut client, interactive_mode, &mut config)?;
        }
        Command::Courses => {
            require_logged_in(&mut client)?;
            let org = require_org()?;
            courses::list_courses(io, &mut client, org)?;
        }
        Command::Submit { exercise } => {
            require_logged_in(&mut client)?;
            submit::submit(io, &mut client, exercise.as_deref(), &config)?;
        }
        Command::Exercises { course } => {
            require_logged_in(&mut client)?;
            let org = require_org()?;
            exercises::list_exercises(io, &mut client, course.as_deref(), org)?
        }
        Command::Test { exercise } => {
            test::test(io, exercise.as_deref(), &config)?;
        }
        Command::Paste { exercise } => {
            require_logged_in(&mut client)?;
            paste::paste(io, &mut client, exercise.as_deref(), &config)?;
        }
        Command::Logout => {
            require_logged_in(&mut client)?;
            logout::logout(io, &mut client, &mut config)?;
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
            let org = require_org()?;
            download::elevated_download(io, &mut client, &config, org)?;
        }
        Command::Elevatedupdate => {
            update::elevated_update(io, &mut client, &config)?;
        }
        Command::GenerateCompletions { shell } => {
            generate_completions::generate(shell);
        }
    }
    Ok(())
}
