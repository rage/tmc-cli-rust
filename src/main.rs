use clap::{App, Arg, ArgMatches, SubCommand};
//use std::io::{Write, Read};
use std::io::{stdin, stdout};

pub mod config;

pub mod io_module;
use io_module::IoProduction;
pub mod commands;
mod updater;

fn main() {
    let mut stdin = stdin();
    let mut output = stdout();

    let mut io = IoProduction::new(&mut output, &mut stdin);

    let matches = get_matches();
    match matches.occurrences_of("no-update") {
        0 => {
            let os = std::env::consts::OS;
            if os == "windows" {
                updater::check_for_update();
            }
        }
        _ => println!("No Auto-Updates"),
    }
    commands::handle(&matches, &mut io);
}

fn get_matches() -> ArgMatches<'static> {
    let matches = App::new("Test My Code client written in Rust")
        .version("0.2.5")
        .about("Command line interface for using TestMyCode functionality")
        .subcommand(SubCommand::with_name("courses").about("List the available courses"))
        .subcommand(
            SubCommand::with_name("download")
                .about("Sets the level of verbosity")
                .arg(Arg::with_name("course").value_name("course").required(true))
                .arg(
                    Arg::with_name("download_folder")
                        .value_name("download_folder")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("exercises")
                .about("List the exercises for a specific course")
                .arg(Arg::with_name("course").value_name("course").required(true)),
        )
        .subcommand(SubCommand::with_name("login").about("Login to TMC server"))
        .subcommand(SubCommand::with_name("logout").about("Logout from TMC server"))
        .subcommand(SubCommand::with_name("organization").about("Change organization"))
        .subcommand(
            SubCommand::with_name("paste")
                .about("Submit exercise to TMC pastebin")
                .arg(
                    Arg::with_name("exercise")
                        .value_name("exercise")
                        .required(true),
                )
                .arg(
                    Arg::with_name("paste message")
                        .value_name("paste_message")
                        .required(false),
                ),
        )
        .subcommand(SubCommand::with_name("submit").about("Submit exercises to TMC server"))
        .subcommand(
            SubCommand::with_name("test")
                .about("Run local exercise tests")
                .arg(
                    Arg::with_name("exercise")
                        .value_name("exercise")
                        .required(false),
                ),
        )
        .arg(
            Arg::with_name("no-update")
                .short("d")
                .long("no-update")
                .help("Disable auto update temporarily"),
        )
        .arg(
            Arg::with_name("testmode")
                .long("testmode")
                .help("Only for internal testing, runs commands in offline mode"),
        )
        .get_matches();

    matches
}
