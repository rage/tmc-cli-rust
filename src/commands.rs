use crate::updater;
use command_util::ClientProduction;
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

    match matches.subcommand() {
        Some(("login", args)) => {
            let interactive_mode = !args.is_present("non-interactive");
            login(io, &mut client, interactive_mode)
        }
        Some(("download", args)) => {
            download_or_update(
                io,
                &mut client,
                args.value_of("course"),
                args.is_present("currentdir"),
            );
            //io.println("arguments not found", PrintColor::Normal);
        }
        Some(("update", args)) => {
            //TODO: Make own commandfile when tmc-langs-rust supports update
            //with folder as a parameter
            download_or_update(
                io,
                &mut client,
                args.value_of("course"),
                args.is_present("currentdir"),
            );
            //io.println("arguments not found", PrintColor::Normal);
        }
        Some(("organization", args)) => {
            let interactive_mode = !args.is_present("non-interactive");
            organization(io, &mut client, interactive_mode)
        }
        Some(("courses", _)) => list_courses(io, &mut client),
        Some(("submit", args)) => {
            let path;
            if args.is_present("exercise") {
                path = args.value_of("exercise").unwrap_or("");
            } else {
                path = "";
            }
            submit_command::submit(io, &mut client, path);
        }
        Some(("exercises", args)) => {
            if let Some(c) = args.value_of("course") {
                list_exercises(io, &mut client, String::from(c));
            } else {
                io.println("argument for course not found", PrintColor::Normal);
            }
        }
        Some(("test", args)) => {
            if args.is_present("exercise") {
                test(io, args.value_of("exercise"));
            } else {
                test(io, None);
            }
        }
        Some(("paste", args)) => {
            let path;
            if args.is_present("exercise") {
                path = args.value_of("exercise").unwrap_or("");
            } else {
                path = "";
            }
            paste_command::paste(io, &mut client, path);
        }
        Some(("logout", _)) => logout(io, &mut client),
        Some(("fetchupdate", _)) => {
            updater::process_update();
        }
        Some(("cleartemp", _)) => {
            updater::cleartemp().unwrap();
        }
        Some(("elevateddownload", _)) => {
            download_command::elevated_download(io, &mut client);
        }
        None => (),
        _ => (),
    }
}
