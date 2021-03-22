use command_util::ClientProduction;
use courses_command::list_courses;
use download_command::download_or_update;
use exercises_command::list_exercises;
use login_command::login;
use logout_command::logout;
use organization_command::organization;
use test_command::test;
mod command_util;
mod courses_command;
mod download_command;
mod exercises_command;
mod login_command;
mod logout_command;
mod organization_command;
mod paste_command;
mod submit_command;
mod test_command;

use crate::io_module::Io;

pub fn handle(matches: &clap::ArgMatches, io: &mut dyn Io) {
    //println!("{:#?}", matches.subcommand());

    let mut client = ClientProduction::new(matches.is_present("testmode"));

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
                    a.value_of("download_folder"),
                );
            } else {
                io.println("arguments not found");
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
                    io.println("argument for course not found");
                }
            } else {
                io.println("argument not found for course");
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
        (_, Some(_)) => (), // Not implemented yet
        (_, None) => (),    // No subcommand was given
    }
}
