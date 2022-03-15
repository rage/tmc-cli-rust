mod courses;
mod download;
mod exercises;
mod login;
mod logout;
mod organization;
mod paste;
mod submit;
mod test;
mod update;
pub mod util;

use crate::io::{Io, PrintColor};
use anyhow::Context;
use util::{Client, ClientProduction};

pub fn handle(matches: &clap::ArgMatches, io: &mut dyn Io) -> anyhow::Result<()> {
    let mut client = ClientProduction::new(matches.is_present("testmode"))?;

    // Authorize the client and raise error if not logged in when required
    match matches.subcommand() {
        Some(("login", _)) => {
            if client.load_login().is_ok() {
                anyhow::bail!("Already logged in. Please logout first with 'tmc logout'",);
            }
        }
        Some(("test", _)) => (),
        _ => {
            client
                .load_login()
                .context("No login found. Login to use this command with 'tmc login'")?;
        }
    };

    // Check that organization is set
    if let Some(("download" | "courses", _)) = matches.subcommand() {
        util::get_organization().context("No organization found. Run 'tmc organization' first.")?;
    };

    match matches.subcommand() {
        Some(("login", args)) => {
            let interactive_mode = !args.is_present("non-interactive");
            login::login(io, &mut client, interactive_mode)?;
        }
        Some(("download", args)) => download::download_or_update(
            io,
            &mut client,
            args.value_of("course"),
            args.is_present("currentdir"),
        )?,
        Some(("update", args)) => {
            update::update(io, &mut client, args.is_present("currentdir"))?;
        }
        Some(("organization", args)) => {
            let interactive_mode = !args.is_present("non-interactive");
            organization::organization(io, &mut client, interactive_mode)?
        }
        Some(("courses", _)) => courses::list_courses(io, &mut client)?,
        Some(("submit", args)) => {
            submit::submit(io, &mut client, args.value_of("exercise"))?;
        }
        Some(("exercises", args)) => {
            if let Some(c) = args.value_of("course") {
                exercises::list_exercises(io, &mut client, c)?;
            } else {
                io.println("argument for course not found", PrintColor::Normal)?;
            }
        }
        Some(("test", args)) => {
            test::test(io, args.value_of("exercise"))?;
        }
        Some(("paste", args)) => {
            paste::paste(io, &mut client, args.value_of("exercise"))?;
        }
        Some(("logout", _)) => logout::logout(io, &mut client)?,
        Some(("fetchupdate", _)) => {
            #[cfg(target_os = "windows")]
            crate::updater::process_update()?;
        }
        Some(("cleartemp", _)) => {
            #[cfg(target_os = "windows")]
            crate::updater::cleartemp()?;
        }
        Some(("elevateddownload", _)) => {
            download::elevated_download(io, &mut client)?;
        }
        Some(("elevatedupdate", _)) => {
            update::elevated_update(io, &mut client)?;
        }
        _ => (), // Unknown subcommand or no subcommand was given
    }
    Ok(())
}
