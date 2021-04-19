use termcolor::{BufferWriter, ColorChoice};

use clap::{App, Arg, ArgMatches, SubCommand};
use std::io::stdin;

pub mod io_module;
use io_module::IoProduction;
pub mod commands;
pub mod interactive;
pub mod progress_reporting;
mod updater;

const PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    let mut stdin = stdin();
    let mut bufferwriter = BufferWriter::stderr(ColorChoice::Always);
    let mut output = bufferwriter.buffer();

    let mut io = IoProduction::new(&mut bufferwriter, &mut output, &mut stdin);

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
        .version(PKG_VERSION.unwrap())
        .about("Does awesome things")
        .subcommand(
            SubCommand::with_name("config")
                .about("Set/unset TMC-CLI properties and change settings"),
        )
        .subcommand(SubCommand::with_name("courses").about("List the available courses"))
        .subcommand(
            SubCommand::with_name("download")
                .about("Downloads course exercises")
                .arg(
                    Arg::with_name("course")
                        .short("c")
                        .long("course")
                        .value_name("course name")
                        .required(false),
                )
                .arg(
                    Arg::with_name("currentdir")
                        .short("d")
                        .long("currentdir")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("exercises")
                .about("List the exercises for a specific course")
                .arg(Arg::with_name("course").value_name("course").required(true)),
        )
        .subcommand(SubCommand::with_name("help").about("List every command"))
        .subcommand(SubCommand::with_name("info").about("Show info about the current directory"))
        .subcommand(
            SubCommand::with_name("login")
                .about("Login to TMC server")
                .arg(
                    Arg::with_name("non-interactive")
                        .short("n")
                        .help("Initiates the non-interactive mode.")
                        .long("non-interactive"),
                ),
        )
        .subcommand(SubCommand::with_name("logout").about("Logout from TMC server"))
        .subcommand(
            SubCommand::with_name("organization")
                .about("Change organization")
                .arg(
                    Arg::with_name("non-interactive")
                        .short("n")
                        .help("Initiates the non-interactive mode.")
                        .long("non-interactive"),
                ),
        )
        .subcommand(
            SubCommand::with_name("paste")
                .about("Submit exercise to TMC pastebin")
                .arg(
                    Arg::with_name("exercise")
                        .value_name("exercise")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("submit")
                .about("Submit exercises to TMC server")
                .arg(
                    Arg::with_name("exercise")
                        .value_name("exercise")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("Run local exercise tests")
                .arg(
                    Arg::with_name("exercise")
                        .value_name("exercise")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("fetchupdate")
                .about("Finishes the autoupdater. Administator rights needed."),
        )
        .subcommand(
            SubCommand::with_name("cleartemp")
                .about("Removes tempfiles. Administator rights needed."),
        )
        .subcommand(SubCommand::with_name("update").about("Update exercises"))
        .arg(
            Arg::with_name("no-update")
                .short("d")
                .long("no-update")
                .help("Disable auto update temporarily"),
        )
        .arg(
            Arg::with_name("testmode")
                .long("testmode")
                .help("Only for internal testing, disables server connection"),
        )
        .get_matches();

    matches
}
