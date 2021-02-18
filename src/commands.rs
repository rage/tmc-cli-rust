use courses_command::list_courses;
use download_command::download_or_update;
use exercises_command::list_excercises;
use login_command::login;
use logout_command::logout;
use organization_command::organization;
mod command_util;
mod courses_command;
mod download_command;
mod exercises_command;
mod login_command;
mod logout_command;
mod organization_command;

use crate::io_module::IO;

pub fn handle(matches: &clap::ArgMatches, io: &mut IO) {
    //println!("{:#?}", matches.subcommand());

    match matches.subcommand() {
        ("login", _) => login(io),
        ("download", args) => {
            if let Some(a) = args {
                let mut course;
                let mut download_folder;
                if let Some(c) = a.value_of("course") {
                    course = String::from(c);
                } else {
                    io.println("argument for course not found");
                    return;
                }
                if let Some(d) = a.value_of("download_folder") {
                    download_folder = String::from(d);
                } else {
                    io.println("argument for downloda folder not found");
                    return;
                }
                download_or_update(io, course, download_folder);
            } else {
                io.println("arguments not found");
            }
        }
        ("organization", _) => organization(io),
        ("courses", _) => list_courses(io),
        ("exercises", args) => {
            if let Some(a) = args {
                if let Some(c) = a.value_of("course") {
                    list_excercises(io, String::from(c));
                } else {
                    io.println("argument for course not found");
                }
            } else {
                io.println("arguments not found");
            }
        }
        ("logout", _) => logout(io),
        (_, Some(_)) => (), // Not implemented yet
        (_, None) => (),    // No subcommand was given
    }
}
