#[cfg(target_os = "windows")]
use crate::updater;
use command_util::{get_organization, Client, ClientProduction};
use courses_command::list_courses;
use download_command::download_or_update;
use exercises_command::list_exercises;
use login_command::login;
use logout_command::logout;
use organization_command::organization;
use update_command::update;
pub mod command_util;
mod courses_command;
mod download_command;
mod exercises_command;
mod login_command;
mod logout_command;
mod organization_command;
mod paste_command;
mod submit_command;
mod test_command;
mod update_command;

use crate::io_module::{Io, PrintColor};

pub fn handle(matches: &clap::ArgMatches, io: &mut dyn Io) {
    let mut client = ClientProduction::new(matches.is_present("testmode"));

    // Authorize the client and raise error if not logged in when required
    match matches.subcommand() {
        Some(("login", _)) => {
            if client.load_login().is_ok() {
                io.println(
                    "Already logged in. Please logout first with 'tmc logout'",
                    PrintColor::Failed,
                );
                return;
            }
        }
        Some(("test", _)) => (),
        _ => {
            if client.load_login().is_err() {
                io.println(
                    "No login found. Login to use this command with 'tmc login'",
                    PrintColor::Failed,
                );
                return;
            }
        }
    };

    // Check that organization is set
    if let Some(("download" | "courses", _)) = matches.subcommand() {
        if get_organization().is_none() {
            io.println(
                "No organization found. Run 'tmc organization' first.",
                PrintColor::Failed,
            );
            return;
        }
    };

    match matches.subcommand() {
        Some(("login", args)) => {
            let interactive_mode = !args.is_present("non-interactive");
            login(io, &mut client, interactive_mode)
        }
        Some(("download", args)) => download_or_update(
            io,
            &mut client,
            args.value_of("course"),
            args.is_present("currentdir"),
        ),
        Some(("update", args)) => {
            update(io, &mut client, args.is_present("currentdir"));
        }
        Some(("organization", args)) => {
            let interactive_mode = !args.is_present("non-interactive");
            organization(io, &mut client, interactive_mode)
        }
        Some(("courses", _)) => list_courses(io, &mut client),
        Some(("submit", args)) => {
            submit_command::submit(io, &mut client, args.value_of("exercise"));
        }
        Some(("exercises", args)) => {
            if let Some(c) = args.value_of("course") {
                list_exercises(io, &mut client, String::from(c));
            } else {
                io.println("argument for course not found", PrintColor::Normal);
            }
        }
        Some(("test", args)) => {
            test_command::test(io, args.value_of("exercise"));
        }
        Some(("paste", args)) => {
            paste_command::paste(io, &mut client, args.value_of("exercise"));
        }
        Some(("logout", _)) => logout(io, &mut client),
        Some(("fetchupdate", _)) => {
            #[cfg(target_os = "windows")]
            updater::process_update();
        }
        Some(("cleartemp", _)) => {
            #[cfg(target_os = "windows")]
            updater::cleartemp().unwrap();
        }
        Some(("elevateddownload", _)) => {
            download_command::elevated_download(io, &mut client);
        }
        Some(("elevatedupdate", _)) => {
            update_command::elevated_update(io, &mut client);
        }
        _ => (), // Unknown subcommand or no subcommand was given
    }
}
