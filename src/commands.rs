mod download;
mod generate_completions;
mod login;
mod logout;
mod mooc;
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
    let default_tmc_url = "https://tmc.mooc.fi";
    let default_mooc_url = "https://courses.mooc.fi";

    let tmc_root_url = match env::var("TMC_LANGS_TMC_ROOT_URL") {
        Ok(url) => url
            .parse()
            .with_context(|| format!("Failed to parse TMC_LANGS_TMC_ROOT_URL ({url}) as a URL"))?,
        Err(_) => default_tmc_url.parse().expect("known to work"),
    };
    let mooc_root_url =
        env::var("TMC_LANGS_MOOC_ROOT_URL").unwrap_or_else(|_| default_mooc_url.to_string());
    let mut client = Client::new(tmc_root_url, mooc_root_url, cli.testmode)?;

    match cli.subcommand {
        Command::Login => {
            login::login(io, &mut client, &mut config)?;
        }
        Command::Logout => {
            logout::logout(io, &mut client, &mut config)?;
        }
        Command::Download {
            organization,
            course,
            currentdir,
        } => {
            download::download_or_update(
                io,
                &mut client,
                &mut config,
                organization,
                course.as_deref(),
                currentdir,
            )?;
        }
        Command::Update { currentdir } => {
            update::update(io, &mut client, &mut config, currentdir)?;
        }
        Command::Test { exercise } => {
            test::test(io, &config, exercise.as_deref())?;
        }
        Command::Paste { exercise } => {
            paste::paste(io, &mut client, &mut config, exercise.as_deref())?;
        }
        Command::Submit { exercise } => {
            submit::submit(io, &mut client, &mut config, exercise.as_deref())?;
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
            download::elevated_download(io, &mut client, &mut config)?;
        }
        Command::Elevatedupdate => {
            update::elevated_update(io, &mut client, &mut config)?;
        }
        Command::GenerateCompletions { shell } => {
            generate_completions::generate(shell);
        }
    }
    Ok(())
}

pub enum Platform {
    Mooc,
    Tmc,
}
