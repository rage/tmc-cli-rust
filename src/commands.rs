use crate::updater;
use command_util::{get_organization, Client, ClientProduction};
use courses_command::list_courses;
use download_command::download_or_update;
use exercises_command::list_exercises;
use login_command::login;
use logout_command::logout;
use organization_command::organization;
use test_command::test;
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

use crate::io_module::{Io, PrintColor};

pub fn handle(matches: &clap::ArgMatches, io: &mut dyn Io) {
    //println!("{:#?}", matches.subcommand());

    let mut client = ClientProduction::new(matches.is_present("testmode"));

    // Authorize the client and raise error if not logged in when required
    match matches.subcommand().0 {
        "login" => {
            if let Ok(()) = client.load_login() {
                io.println(
                    "Already logged in. Please logout first with 'tmc logout'",
                    PrintColor::Failed,
                );
                return;
            }
        }
        "test" => (),
        _ => {
            if let Err(_) = client.load_login() {
                io.println(
                    "No login found. Login to use this command with 'tmc login'",
                    PrintColor::Failed,
                );
                return;
            }
        }
    };

    // Check that organization is set
    match matches.subcommand().0 {
        "download" | "courses" => {
            if get_organization().is_none() {
                io.println(
                    "No organization found. Run 'tmc organization' first.",
                    PrintColor::Failed,
                );
                return;
            }
        }
        _ => (),
    };

    match matches.subcommand() {
        ("login", args) => {
            if let Some(args) = args {
                let interactive_mode;
                if args.is_present("non-interactive") {
                    interactive_mode = false;
                } else {
                    interactive_mode = true;
                }
                login(io, &mut client, interactive_mode)
            }
        }
        ("download", args) => {
            if let Some(a) = args {
                download_or_update(
                    io,
                    &mut client,
                    a.value_of("course"),
                    a.is_present("currentdir"),
                );
            } else {
                io.println("Error: Arguments not found", PrintColor::Failed);
            }
        }
        ("update", args) => {
            //TODO: Make own commandfile when tmc-langs-rust supports update
            //with folder as a parameter
            if let Some(a) = args {
                download_or_update(
                    io,
                    &mut client,
                    a.value_of("course"),
                    a.is_present("currentdir"),
                );
            } else {
                io.println("Error: Arguments not found", PrintColor::Failed);
            }
        }
        ("organization", args) => {
            if let Some(args) = args {
                let interactive_mode;
                if args.is_present("non-interactive") {
                    interactive_mode = false;
                } else {
                    interactive_mode = true;
                }
                organization(io, &mut client, interactive_mode)
            }
        }
        ("courses", _) => list_courses(io, &mut client),
        ("submit", args) => {
            let path;
            if let Some(a) = args {
                path = a.value_of("exercise").unwrap_or("");
            } else {
                path = "";
            }
            submit_command::submit(io, &mut client, path);
        }
        ("exercises", args) => {
            if let Some(a) = args {
                if let Some(c) = a.value_of("course") {
                    list_exercises(io, &mut client, String::from(c));
                } else {
                    io.println("argument for course not found", PrintColor::Normal);
                }
            } else {
                io.println("argument not found for course", PrintColor::Normal);
            }
        }
        ("test", args) => {
            if let Some(a) = args {
                test(io, a.value_of("exercise"));
            } else {
                test(io, None);
            }
        }
        ("paste", args) => {
            let path;
            if let Some(a) = args {
                path = a.value_of("exercise").unwrap_or("");
            } else {
                path = "";
            }
            paste_command::paste(io, &mut client, path);
        }
        ("logout", _) => logout(io, &mut client),
        ("fetchupdate", _) => {
            updater::process_update();
        }
        ("cleartemp", _) => {
            updater::cleartemp().unwrap();
        }
        ("elevateddownload", _) => {
            download_command::elevated_download(io, &mut client);
        }
        (_, Some(_)) => (), // Not implemented yet
        (_, None) => (),    // No subcommand was given
    }
}
